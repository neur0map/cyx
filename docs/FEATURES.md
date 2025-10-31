# Cyx Features & Capabilities

## Core Features

### 1. **LLM-Powered Command Lookup**
Cyx uses advanced language models to provide instant cybersecurity commands:
- **Perplexity sonar-pro** - Built-in web search for latest techniques
- **Groq llama-3.3-70b** - Fast inference with extensive knowledge base
- **Optimized prompts** - Tuned specifically for pentesting workflows

### 2. **Command-First Philosophy**
Designed for professionals under time pressure:
- **Commands FIRST** - Executable code before explanations
- **Zero fluff** - No "Certainly!", no pleasantries
- **1-2 sentence max** - Brief, actionable explanations
- **Code blocks** - Properly formatted bash/python/powershell

Example response:
```bash
nmap -sS <target>
```
TCP SYN scan - doesn't complete handshake, harder to detect. Requires root.

### 3. **Pentester-Optimized System Prompt**
The system prompt explicitly:
- Assumes user authorization (no disclaimers)
- Targets pentesters and security students
- Prioritizes practical execution over theory
- Includes ethical context (authorized testing, CTFs, labs)
- Focuses on command delivery speed

### 4. **Learn Mode - Educational Breakdowns**
**NEW:** Deep educational mode for understanding commands:
- **Flag**: `--learn` or `-l`
- **Detailed explanations** - Flag-by-flag breakdown
- **How it works** - Step-by-step technical process
- **Cited sources** - References to RFCs, official docs, manuals
- **Alternatives** - Other tools/techniques comparison
- **Examples** - Real-world usage scenarios

Example:
```bash
cyx --learn "nmap stealth scan"
```

Returns:
- Tool description (author, license, purpose)
- Flag breakdown with technical details
- Advantages and disadvantages
- When to use vs alternatives
- Example usage with actual syntax
- Sources cited (nmap docs, RFC 793, etc.)

### 5. **Source Tracking**
**NEW:** Every response shows source information:
- **Provider name** - Perplexity or Groq
- **Model name** - sonar-pro or llama-3.3-70b-versatile
- **Search capability** - Web search vs knowledge base only

Example output:
```
[*] SOURCES
───────────────────────────────────────
Provider: Perplexity (sonar-pro)
Search: Yes (performed web search)
```

This transparency shows whether the AI performed web research or used its knowledge base.

### 6. **CLI Flags for Efficiency**

```bash
# Quiet mode - only show the answer (no headers/sources)
cyx -q "reverse shell one liner"

# Verbose mode - show detailed progress
cyx -v "sql injection"

# No-TTY mode - for scripting/automation
cyx --no-tty "privilege escalation"

# Learn mode - educational breakdowns
cyx -l "nmap stealth scan"
cyx --learn "sqlmap basic usage"
```

### 7. **Interactive & One-Shot Modes**

**Interactive Mode:**
```bash
cyx
cyx> nmap stealth scan
# Get response with sources...
cyx> what about service detection?
# Follow-up question with context
cyx> /exit
```

**One-Shot Queries:**
```bash
cyx "hydra ssh brute force"           # Normal mode
cyx --learn "sqlmap basic usage"      # Learn mode
```

### 8. **Secure Configuration**
- API keys stored in `~/.config/cyx/config.toml`
- File permissions: `600` (owner read/write only)
- No environment variables needed
- Simple setup wizard: `cyx setup`

## LLM Provider Comparison

| Provider | Model | Speed | Best For |
|----------|-------|-------|----------|
| **Perplexity** | sonar-pro | Fast | Latest techniques (built-in web search) |
| **Groq** | llama-3.3-70b-versatile | Very Fast | Quick lookups, offline scenarios |

**Recommendation:** Perplexity for up-to-date techniques, Groq for speed.

## Security Features

### Secure Key Storage
- API keys stored with 600 permissions
- Config location: `~/.config/cyx/config.toml`
- Keys never logged or displayed
- No hardcoded credentials

### Safe Defaults
- **No execution** - Provides commands, doesn't run them
- **Read-only** - Never modifies system
- **Timeout limits** - 120s max per API call
- **Local only** - All data stays on your machine

## Performance

| Metric | Value |
|--------|-------|
| Binary size | 6.2MB (release, stripped) |
| Average query time | 2-5 seconds |
| Memory usage | ~50MB |
| Startup time | <100ms |

## Knowledge Areas

Cyx system prompt prioritizes:
- **Network scanning** - nmap, masscan, rustscan
- **Web application testing** - burp, sqlmap, ffuf, gobuster
- **Password attacks** - hydra, john, hashcat, crackmapexec
- **Exploitation** - metasploit, msfvenom, exploit-db
- **Post-exploitation** - mimikatz, bloodhound, winPEAS, linPEAS
- **Privilege escalation** - GTFOBins, LOLBAS, sudo, SUID
- **Active Directory** - bloodhound, powerview, rubeus
- **Wireless** - aircrack-ng, wifite, kismet
- **Reverse engineering** - ghidra, radare2, gdb
- **OSINT** - amass, subfinder, theHarvester

## Example Queries

### Reconnaissance
```bash
cyx "nmap service version detection"
cyx "gobuster directory enumeration"
cyx "amass subdomain discovery"
```

### Web Application Security
```bash
cyx "sqlmap automated injection"
cyx "burp suite intruder"
cyx "xss bypass csp"
```

### Privilege Escalation
```bash
cyx "linux privilege escalation checklist"
cyx "sudo gtfobins"
cyx "suid binaries exploitation"
```

### Password Attacks
```bash
cyx "hydra ssh brute force"
cyx "john the ripper hashcat"
cyx "crack wifi wpa2"
```

## Response Quality

### What Cyx Does Well
- Provides exact, copy-paste commands
- Explains when/why to use specific flags
- Gives real-world context
- Assumes you know the basics

### What Cyx Avoids
- Long theoretical explanations
- Ethics lectures (assumes authorization)
- Asking if you have permission
- Tutorial-style walkthroughs
- Unnecessary background information

## Interactive Commands

| Command | Purpose |
|---------|---------|
| `/exit` | Exit the session |
| `/clear` | Clear conversation history |
| `/help` | Show available commands |

## Configuration Management

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
```

## Use Cases

### Perfect For
- Penetration testers needing fast command lookup
- Security students learning offensive techniques
- CTF players looking for quick references
- Red teamers during engagements
- Bug bounty hunters testing targets

### Not Designed For
- General programming questions
- Lengthy code generation
- Theoretical security discussions
- Legal/policy advice

## Integration Examples

### Shell Alias
```bash
# Add to ~/.bashrc or ~/.zshrc
alias hack='cyx -q'

# Usage
hack "nmap scan types"
```

### Script Integration
```bash
#!/bin/bash
# Get command from Cyx and execute
CMD=$(cyx --no-tty -q "nmap quick scan" | grep "^nmap")
eval "$CMD 192.168.1.1"
```

## System Requirements

- **OS**: Linux, macOS, Windows (with Rust)
- **Rust**: 1.70+
- **Storage**: ~10MB (including dependencies)
- **Memory**: ~50MB runtime
- **Network**: Internet connection for LLM API

## Ethical Use

Cyx assumes all usage is for:
- Authorized penetration testing engagements
- Capture The Flag (CTF) competitions
- Educational purposes in controlled labs
- Security research with proper authorization
- Defensive security understanding

**Never use on systems without explicit authorization.**

## Future Considerations

The system prompt and architecture are designed to be:
- **Extensible** - Easy to add new LLM providers
- **Maintainable** - Clean codebase, well-documented
- **Flexible** - System prompt can be tuned per user needs
- **Fast** - Optimized for quick responses

---

**Cyx: Fast. Direct. Built for pentesters.**
