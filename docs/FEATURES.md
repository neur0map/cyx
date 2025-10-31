# Cyx Features & Capabilities

## [X] Core Features

### 1. **Straight-to-the-Point Responses**
Cyx is designed for speed and efficiency:
- **Commands FIRST** - No theory, just the command you need
- **Brief explanations** - 1-2 sentences max
- **Code blocks** - All commands properly formatted
- **Citations** - References to official documentation

Example query: "nmap stealth scan"
```bash
nmap -sS <target>
```
-sS performs a TCP SYN (stealth) scan. Root privileges required.

### 2. **Cybersecurity-Optimized**
Built specifically for:
- Penetration testing
- Ethical hacking
- CTF competitions
- Security research
- Red team operations
- Bug bounty hunting

### 3. **LLM Provider Support**

| Provider | Model | Speed | Features |
|----------|-------|-------|----------|
| **Perplexity** | sonar-pro | [X] Fast | Built-in web search, excellent citations |
| **Groq** | llama-3.3-70b-versatile | [X][X] Very Fast | Free tier, great for offline knowledge |

**Recommendation:** Use Perplexity for best results - it has web search built into the model.

### 4. **CLI Flags for Power Users**

```bash
# Quiet mode - only show the answer
cyx -q "reverse shell one liner"

# Verbose mode - show all details
cyx -v "sql injection"

# No-TTY mode - for scripting/automation
cyx --no-tty "privilege escalation"

# Skip web search - use LLM knowledge only
cyx --no-search "metasploit"

# Limit search results
cyx --max-results 3 "buffer overflow"
```

### 5. **Secure Configuration**
- API keys stored in `~/.config/cyx/config.toml`
- File permissions: `600` (owner read/write only)
- No environment variables needed
- Set once, use forever

### 6. **Easy Setup**
```bash
# One command setup
cyx setup

# Enter your API key when prompted
# That's it! Start using immediately
cyx "your query"
```

## [X] Security Features

### Prompt Injection Protection
Cyx sanitizes all web content to prevent malicious prompts:
- Removes "ignore previous instructions" patterns
- Blocks role manipulation attempts
- Prevents DoS via content repetition
- Limits content size (12k chars max per source)

### Resource Limits
- HTTP timeouts (30s search, 120s LLM)
- Redirect limits (max 10)
- Token limits (8000 max per request)
- Fetch limits (max 3 sources per query)

### Secure API Communication
- HTTPS only
- Proper timeout handling
- Error messages never expose API keys
- Sandboxed HTML parsing

## [X] Performance

| Metric | Value |
|--------|-------|
| Binary size | 6.2MB (release, stripped) |
| Average query time | 2-5 seconds |
| Memory usage | ~50MB |
| Startup time | <100ms |

## [X] Example Queries

### Reconnaissance
```bash
cyx "nmap service version detection"
cyx "gobuster directory enumeration"
cyx "amass subdomain discovery"
```

### Web Application Security
```bash
cyx "sqlmap automated sql injection"
cyx "burp suite intruder"
cyx "xss bypass csp"
cyx "jwt token manipulation"
```

### Privilege Escalation
```bash
cyx "linux privilege escalation checklist"
cyx "windows privilege escalation techniques"
cyx "sudo gtfobins"
cyx "suid binaries exploitation"
```

### Password Attacks
```bash
cyx "hydra ssh brute force"
cyx "john the ripper hashcat"
cyx "crack wifi wpa2"
cyx "responder ntlm relay"
```

### Exploitation
```bash
cyx "metasploit reverse shell"
cyx "msfvenom payload generation"
cyx "buffer overflow exploit development"
cyx "rop chain creation"
```

### Post-Exploitation
```bash
cyx "mimikatz dump credentials"
cyx "bloodhound active directory"
cyx "lateral movement techniques"
cyx "persistence windows"
```

## [X][X] Usage Modes

### 1. One-Shot Queries
Perfect for quick lookups:
```bash
cyx "reverse shell bash"
```

### 2. Interactive Mode
For deeper research with follow-up questions:
```bash
cyx
# Enter interactive session
cyx> how to crack zip files
# Get response, ask follow-ups
cyx> what about password-protected rar files?
cyx> /exit
```

### 3. Quiet Mode (Scripting)
For automation and pipelines:
```bash
cyx -q "command" | grep "specific output"
```

### 4. Verbose Mode (Debugging)
See detailed progress:
```bash
cyx -v "complex query"
```

## 📋 Interactive Commands

| Command | Purpose |
|---------|---------|
| `/exit` | Exit the session |
| `/quit` | Same as /exit |
| `/clear` | Clear conversation history |
| `/help` | Show available commands |
| `/sources` | Show recently fetched sources |

## [X] Output Formatting

Cyx provides beautiful, easy-to-read output:
- **Color-coded** - Errors in red, success in green, info in cyan
- **Tables** - Search results displayed in clean tables
- **Code blocks** - Syntax highlighting for commands
- **Sections** - Clear visual separation
- **Emoji indicators** - Quick visual feedback ([X] [X] [X] [X] [X])

## [X] Configuration Management

```bash
# Show current config
cyx config show

# Change provider
cyx config set provider groq
cyx config set provider perplexity

# Update API keys
cyx config set groq_api_key YOUR_KEY
cyx config set perplexity_api_key YOUR_KEY

# Get specific value
cyx config get provider
cyx config get config_path
```

## [X] Target Audience

### Perfect For:
- [X] Penetration testers
- [X] Security researchers
- [X] CTF players
- [X] Bug bounty hunters
- [X] Red teamers
- [X] Students learning cybersecurity
- [X] Anyone needing quick command reference

### Not Designed For:
- [X] General programming questions (use GitHub Copilot)
- [X] Lengthy theoretical explanations (use ChatGPT)
- [X] Code generation (use Cursor/Copilot)

## [X] Pro Tips

1. **Use Perplexity for latest techniques** - It searches the web in real-time
2. **Use Groq for offline knowledge** - Faster, no web dependency
3. **Combine with other tools** - Pipe output to grep, awk, etc.
4. **Save common queries** - Create shell aliases for frequent tasks
5. **Use --quiet in scripts** - Clean output for automation
6. **Use --verbose when learning** - See how it works under the hood

## [X] Integration Examples

### Shell Alias
```bash
# Add to ~/.bashrc or ~/.zshrc
alias hack='cyx -q'

# Usage
hack "sql injection cheatsheet"
```

### Script Integration
```bash
#!/bin/bash
# Get nmap command for specific scan type
SCAN_CMD=$(cyx --no-tty --quiet "nmap $1 scan" | grep "^nmap")
eval "$SCAN_CMD $2"
```

### Pipeline Usage
```bash
# Extract just the command
cyx -q "hydra ssh brute force" | grep -o "^hydra.*"
```

## [X] Comparison with Alternatives

| Feature | Cyx | Manual Googling | ChatGPT | Security-specific tools |
|---------|-----|-----------------|---------|------------------------|
| Speed | [X][X][X] | [X] | [X][X] | [X][X] |
| Accuracy | [X] | ❓ | [X] | [X] |
| Citations | [X] | [X] | [X] | Sometimes |
| Offline mode | [X] | [X] | [X] | Sometimes |
| Concise | [X] | [X] | [X] | [X] |
| CLI | [X] | [X] | [X] | [X] |
| Free tier | [X] | [X] | [X] | Varies |

## [X] Learning with Cyx

Cyx is excellent for learning:
```bash
# Learn nmap basics
cyx "nmap beginner guide"

# Learn specific technique
cyx "how does sql injection work"

# Compare techniques
cyx "hydra vs medusa vs ncrack"

# Get cheat sheets
cyx "metasploit cheat sheet"
```

## [X] Getting Started (Quick)

```bash
# 1. Build and install
cargo install --path .

# 2. Setup
cyx setup

# 3. Start hacking
cyx "your query"
```

## [X][X] Ethics & Legal

**IMPORTANT:** Cyx assumes you have proper authorization for all security testing.

- [X] Use on systems you own
- [X] Use in authorized penetration tests
- [X] Use in lab environments
- [X] Use for education (CTFs, courses)
- [X] Never use on systems without permission
- [X] Never use for malicious purposes

## [X] Future Enhancements (Potential)

- [ ] Local LLM support (Ollama integration)
- [ ] Custom trusted sources
- [ ] Query history/favorites
- [ ] Export results to markdown/PDF
- [ ] Integration with Metasploit/Burp Suite APIs
- [ ] Multi-language support

---

**Cyx: Fast. Secure. Straight to the point. [X][X]**
