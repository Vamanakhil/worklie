# Worklie CLI - Complete Usage Guide

## 🚀 Quick Start

### Installation

```bash
# From source
git clone https://github.com/yourusername/worklie.git
cd worklie
cargo build --release
sudo mv target/release/worklie /usr/local/bin/

# Or via Homebrew (coming soon)
brew install worklie

# Or via cargo
cargo install worklie
```

### First Run

```bash
worklie report
```

You'll see your daily work summary generated automatically!

---

## 📋 All Commands Reference

### Core Commands

#### `worklie report` - Daily Summary
**What it does**: Generates a summary of your work today (last 24 hours)

```bash
# Basic usage
worklie report

# Output as JSON (for scripting/integration)
worklie report --json

# Example output:
Daily Work Summary
==================

Project: civic-app

Activities Detected:
  • Fixed authentication bug
  • Tested API endpoints

Commits:
  • 2 commits
    • fix(auth): token validation
    • test(api): endpoint coverage

Files Modified:
  • src/auth.js
  • src/api/routes.js

Focus Areas:
  • authentication
  • testing
```

**Use case**: Morning standup prep, end-of-day wrap-up

---

#### `worklie weekly` - Weekly Report
**What it does**: Aggregates your work across the entire week

```bash
worklie weekly
worklie weekly --json

# Shows:
# - All projects worked on
# - Major work areas
# - Total commits and activities
# - Key contributions
```

**Use case**: Weekly reviews, sprint retrospectives

---

#### `worklie standup` - Standup Format
**What it does**: Generates standup-ready format (Yesterday/Today/Blockers)

```bash
worklie standup
worklie standup --json

# Output format:
Yesterday
---------
• Fixed authentication bug
• Tested login APIs

Today
-----
• Write unit tests
• Improve error handling

Blockers
--------
None detected
```

**Use case**: Daily standups, team syncs
**Time saved**: ~5-10 minutes per standup

---

#### `worklie stats` - Quick Statistics
**What it does**: Shows activity statistics

```bash
worklie stats
worklie stats --json

# Output:
Worklie Statistics
==================

Commands executed: 312
Commits made: 5
Unique commands: 89
```

**Use case**: Productivity tracking, metrics gathering

---

### Phase 2: Advanced Features (NEW)

#### `worklie sessions` - Session Analysis
**What it does**: Shows your work sessions (grouped by activity)

```bash
worklie sessions

# Output:
Work Sessions Today
===================

Session 1 (9:00-11:45) - 2h 45m - DEBUGGING
  Commands: 28
  Commits: 2
  Focus Score: 0.85 / 1.0
  Work: Fixed authentication bug
  Files: auth.js, tokenService.js

Session 2 (2:00-5:30) - 3h 30m - FEATURE DEVELOPMENT
  Commands: 42
  Commits: 1
  Focus Score: 0.72 / 1.0
  Work: New API endpoint
  Files: api/routes.js, api/handlers.js

Session 3 (5:45-6:00) - 0h 15m - DOCUMENTATION
  Commands: 3
  Commits: 1
  Focus Score: 0.5 / 1.0
  Work: Updated README
  Files: README.md

SUMMARY:
Total deep work: 6h 15m
Sessions: 3
Average duration: 2h 5m
Average focus: 0.69 / 1.0
```

**Use case**: Understanding your work patterns, context switching visibility
**Key insight**: "I switch context 3 times, lose 45 min... let me batch work better"

---

#### `worklie focus` - Deep Work Detection
**What it does**: Identifies your best focus/deep work sessions

```bash
worklie focus

# Output:
Deep Work Sessions
==================

Monday - 2h 45m (Backend API development)
Tuesday - 3h 20m (Authentication system)
Friday - 1h 15m (Docker debugging)

WEEKLY SUMMARY:
Total deep work: 7h 20m
Best focus day: Tuesday
Best focus time: Morning (9:00-12:00)
Average session: 2h 27m

RECOMMENDATIONS:
✓ Schedule important work in the morning
✓ Reduce meetings after 2 PM on deep work days
✓ Aim for 2-3 hour uninterrupted blocks
```

**Use case**: Calendar optimization, productivity analysis
**Why it matters**: "I'm most productive in the morning - let me protect that time"

---

#### `worklie classify` - Work Type Breakdown
**What it does**: Shows what types of work you actually do

```bash
worklie classify

# Output:
Work Classification
===================

This Week's Breakdown:
  • Debugging: 40% (4 commits, 3h 20m)
  • Features: 30% (3 commits, 2h 45m)
  • DevOps: 20% (2 commits, 1h 30m)
  • Documentation: 10% (1 commit, 45m)

Commits by Type:
  • fix: bug in parser (3h)
  • feat: new endpoint (2h 15m)
  • fix: docker networking (1h 30m)
  • docs: setup guide (45m)

Skills Demonstrated:
  • Debugging expertise
  • Full-stack development
  • Infrastructure/DevOps
  • Technical documentation
```

**Use case**: Resume writing, career assessment, interview prep
**Why it matters**: "I'm 40% debugging, 30% features - that's my real skill mix"

---

#### `worklie switches` - Context Switch Analysis
**What it does**: Shows how much you're context switching (and the cost)

```bash
worklie switches

# Output:
Context Switching Analysis
===========================

Total switches today: 8
Estimated recovery time per switch: ~15 minutes
Estimated lost productivity: ~2 hours

Most expensive switches:
  civic-backend ↔ devops-dashboard: 4 switches (1h lost)
  civic-backend → emails/slack: 2 switches (30m lost)
  devops-dashboard → civic-backend: 2 switches (30m lost)

PATTERNS:
✗ Morning: Heavy switching (4 switches in 30m)
✓ Afternoon: More focused (working on one thing 3h+)

RECOMMENDATIONS:
✓ Batch emails/messages twice daily
✓ Use "Do Not Disturb" in mornings
✓ Block deep work time on calendar
✓ Warning: Each switch = 15 min recovery
```

**Use case**: Identifying productivity killers
**Why it matters**: Developers don't realize "meeting -> code -> message -> code" costs 1 hour

---

#### `worklie portfolio` - Resume/Portfolio Generator
**What it does**: Generates professional resume bullets from your actual work

```bash
worklie portfolio --format markdown
worklie portfolio --format text
worklie portfolio --format json

# Output (markdown):
## Engineering Work Summary

### Backend API System
Architected and maintained Node.js backend service
- Implemented authentication system with JWT tokens
- Reduced API response time by 40% through optimization
- Added comprehensive test coverage (85%)
- 15 commits over 8 hours

**Skills**: Node.js, Express, PostgreSQL, Testing

---

### Docker Infrastructure
Improved containerization and deployment process
- Migrated application to Docker containerization
- Set up CI/CD pipeline with GitHub Actions
- Reduced deployment time from 30m to 5m
- 5 commits over 4 hours

**Skills**: Docker, Kubernetes, CI/CD, DevOps

---

### Authentication System
Redesigned authentication for improved security
- Implemented refresh token rotation
- Added 2FA support
- Reduced security vulnerabilities by 60%
- 10 commits over 12 hours

**Skills**: Security, Authentication, OAuth, Cryptography
```

**Use case**: LinkedIn profile updates, resume writing, interview preparation
**Why it matters**: "My work is automatically documented for my career"

---

#### `worklie metrics` - Detailed Productivity Metrics
**What it does**: Comprehensive productivity dashboard

```bash
worklie metrics

# Output:
Worklie Productivity Metrics
=============================

TIME ANALYSIS:
  Total work time: 38h 45m
  Deep work: 24h 30m (63%)
  Context switching: 14h 15m (37%)
  Average session: 2h 15m

COMMITS & CODE:
  Total commits: 23
  Files changed: 47
  Avg commit size: 2 files/commit
  Commit frequency: ~3 per day

FOCUS PATTERNS:
  Best time: 9 AM - 12 PM
  Worst time: 2 PM - 3 PM
  Best day: Tuesday
  Worst day: Friday

WORK DISTRIBUTION:
  Debugging: 40%
  Feature Dev: 35%
  DevOps: 15%
  Documentation: 10%

TOOL USAGE:
  Most used: git (245x)
  Second: npm (89x)
  Third: vim (67x)

RECOMMENDATIONS:
✓ You're most productive Tuesday morning
✓ Protect your 9-12 AM slot for important work
✓ Consider blocking your calendar 2-3 PM (your worst time)
✓ Your 2h-3h sessions are productive - try to maintain this
```

**Use case**: Weekly reviews, optimization, self-awareness
**Why it matters**: Data-driven productivity improvement

---

## 🔧 Advanced Usage

### JSON Output for Integration

All commands support `--json` flag for scripting:

```bash
# Pipe to jq for filtering
worklie report --json | jq '.commit_messages'

# Send to external tool
worklie report --json | curl -X POST http://api.example.com/report -d @-

# Store in database
worklie report --json > worklie_report_$(date +%Y-%m-%d).json
```

### Scripting Examples

#### Daily Report Email
```bash
#!/bin/bash
worklie standup | mail -s "Daily Standup" team@example.com
```

#### Weekly Slack Message
```bash
#!/bin/bash
REPORT=$(worklie weekly | sed 's/"/\\"/g')
curl -X POST -H 'Content-type: application/json' \
  --data "{\"text\":\"$REPORT\"}" \
  $SLACK_WEBHOOK_URL
```

#### Track Productivity Over Time
```bash
#!/bin/bash
worklie metrics --json >> productivity_log.jsonl
# Then analyze with: cat productivity_log.jsonl | jq -s '.[] | .TIME_ANALYSIS'
```

---

## ⚙️ Configuration & Customization

### Time Windows

Some commands accept customization:

```bash
# Session detection with custom 15-minute gap
worklie sessions --gap 15

# Weekly report for specific date range
worklie weekly --from 2026-03-10 --to 2026-03-16

# Filter by project
worklie report --project civic-backend
```

### Output Formats

```bash
# Plain text (default)
worklie report

# JSON (structured data)
worklie report --json

# Markdown (for documentation)
worklie report --format markdown

# CSV (for spreadsheets)
worklie report --format csv
```

---

## 🔍 Common Use Cases & Commands

### "What did I do yesterday?" (5 seconds)
```bash
worklie standup
```

### "Generate my PR description" (2 clicks)
```bash
worklie pr
```

### "Create resume bullets" (1 command)
```bash
worklie portfolio --format markdown
```

### "Why am I tired?" (Context analyzing)
```bash
worklie switches
```

### "When am I most productive?" (Understanding patterns)
```bash
worklie focus
worklie metrics
```

### "What's my skill mix?" (Career assessment)
```bash
worklie classify
```

### "Send team a standup" (Automation)
```bash
worklie standup | mail -s "Daily Update" team@company.com
```

### "Track productivity over time" (Analytics)
```bash
# Setup: Add to crontab
0 17 * * * worklie metrics --json >> ~/worklie_history.jsonl

# Analyze: Weekly
cat ~/worklie_history.jsonl | jq -s 'map(.FOCUS_PATTERNS) | add'
```

---

## 🔐 Security & Privacy

### What Worklie Collects
✅ Shell command history
✅ Git commit metadata (message, hash, author, time)
✅ File names from git diffs

### What Worklie DOESN'T Collect
❌ Passwords or credentials
❌ API keys
❌ Private data
❌ Anything outside your machine
❌ Any external telemetry

### Data Location
```
All data stays on your machine:
~/.zsh_history
~/.bash_history
.git/logs/
```

### No Cloud Sync
Worklie is local-first. No Slack, no email, no cloud required.

---

## 🆘 Troubleshooting

### "No history found"
- Check if `~/.zsh_history` or `~/.bash_history` exists
- For Zsh: Add to `~/.zshrc`: `setopt EXTENDED_HISTORY`
- For Bash: History may not be timestamped

### "Git commits not showing"
- Make sure you're in a git repository
- Check: `git log` works manually
- Verify commits are recent (last 24 hours)

### "Sessions seem off"
- Default gap is 30 minutes
- Try: `worklie sessions --gap 15` (15 min gap)
- Or: `worklie sessions --gap 60` (60 min gap)

### "Activities not detected"
- Add more commits to your repository
- Run commands that leave histories

---

## 📊 Real Examples

### Example 1: Developer Morning Routine

```bash
# Check yesterday
worklie standup

# Output:
Yesterday
---------
• Fixed auth token bug
• Updated docker config
• Reviewed API design

Today
-----
• Write unit tests
• Deploy to staging

# Share with team
worklie standup | mail -s "Daily Update" team@company.com
```

### Example 2: Interview Preparation

```bash
# Generate portfolio
worklie portfolio --format markdown

# Create file
worklie portfolio --format markdown > my_work_2026_q1.md

# Share with interviewer
cat my_work_2026_q1.md | pbcopy  # paste to LinkedIn, resume, etc.
```

### Example 3: Productivity Analysis

```bash
# Weekly metrics
worklie metrics

# Output shows:
# - You're most productive Tuesday morning
# - Context switching costs you 14 hours/week
# - Your best work: feature development

# Action: Protect Tuesday 9-12 for important work
```

---

## 📚 More Resources

- **README.md** - Feature overview
- **ARCHITECTURE.md** - Technical design
- **FEATURE_ROADMAP.md** - What's coming next
- **GitHub Issues** - Bug reports & suggestions

---

**Happy shipping! 🚀**
