# Worklie Quick Start & Usage Examples

## ⚡ 30-Second Quick Start

```bash
# Install
git clone https://github.com/yourusername/worklie.git
cd worklie
cargo build --release
sudo mv target/release/worklie /usr/local/bin/

# Use
worklie standup          # See today's work
worklie report          # Daily summary
worklie metrics         # Productivity analysis
```

That's it! Worklie is now reading your shell history and git commits automatically.

---

## 🎯 Common Scenarios

### Scenario 1: "I need a standup in 30 seconds"

```bash
$ worklie standup

Standup Report
==============

Yesterday
---------
• Fixed authentication bug
• Tested API endpoints
• Updated docker config

Today
-----
• Write unit tests
• Improve error handling

Blockers
--------
None detected
```

**Time saved**: 5-10 minutes ⏱️

---

### Scenario 2: "I'm interviewing tomorrow"

```bash
# Generate your portfolio
$ worklie portfolio --format markdown > interview_prep.md

# Output:
## Engineering Work Summary

### Authentication System
Improved security and reliability of the login flow
- Fixed token validation issues
- Implemented refresh token logic
- Reduced security vulnerabilities by 60%

Time invested: 8 hours
Files modified: 4
Technologies: Node.js, JavaScript, Testing

### Docker Infrastructure
Improved containerization and deployment
- Migrated application to Docker
- Set up CI/CD pipeline
- Reduced deployment time from 30m to 5m

Time invested: 6 hours
Files modified: 8
Technologies: Docker, GitHub Actions, DevOps
```

**Result**: Professional portfolio ready to share 💼

---

### Scenario 3: "Why am I exhausted?"

```bash
$ worklie switches

Context Switching Analysis
===========================

Total switches: 12
Estimated recovery time per switch: ~15 minutes
Estimated lost productivity: ~3 hours

Most expensive switches:
  civic-backend ↔ devops-dashboard: 6 switches (1h 30m lost)
  civic-backend → emails/slack: 4 switches (1h lost)
  devops-dashboard → civic-backend: 2 switches (30m lost)

⚠️  Context switching cost is VERY significant!

RECOMMENDATIONS:
→ Close email client during coding sessions
→ Batch messages/emails twice daily
→ Block "Focus Time" on calendar
→ Each switch = 15 minutes lost productivity
```

**Insight**: You're losing 3 hours/day to switching 📊

---

### Scenario 4: "When am I most productive?"

```bash
$ worklie focus

Deep Work Sessions
==================

Tuesday - 3h 45m (Backend API development)
Thursday - 2h 30m (Authentication system)
Monday - 1h 15m (Docker debugging)

WEEKLY SUMMARY:
Total deep work: 7h 30m
Best focus day: Tuesday
Best focus time: Morning (9:00-12:00)
Average session: 2h 25m

PATTERN ANALYSIS:
✓ Morning sessions are more focused than afternoon
✓ Tuesday and Thursday are your best focus days
✓ Afternoons see more switching/interruptions

RECOMMENDATIONS:
✓ Schedule important work Tuesday/Thursday mornings
✓ Protect your 9 AM - 12 PM time block
✓ Move meetings to afternoons
```

**Action**: Optimize calendar ✅

---

### Scenario 5: "What skills do I actually have?"

```bash
$ worklie classify

Work Classification
===================

This Week's Breakdown:
  • Debugging: 40% (4 sessions)
  • Feature Development: 35% (3 sessions)
  • DevOps: 20% (2 sessions)
  • Documentation: 5% (1 session)

Skills Demonstrated:
  • Debugging expertise (40%)
  • Full-stack development (35%)
  • Infrastructure/DevOps (20%)
  • Technical documentation (5%)

CAREER INSIGHTS:
→ You're 40% debugger, 35% developer
→ You have strong DevOps skills (20%)
→ You document your work (5%)
→ Your skill mix: Platform Engineer / Tech Lead
```

**Use case**: Update resume with actual skill data 📖

---

## 🔧 Advanced Usage

### Export to JSON for processing

```bash
# Get all yesterday's activities
$ worklie report --json

{
  "project": "civic-app",
  "domain": "authentication",
  "work_type": "debugging",
  "branch": "feature-auth",
  "total_commits": 3,
  "commit_messages": [
    "fix(auth): token validation",
    "test(auth): endpoint coverage",
    "chore: update dependencies"
  ],
  "files_modified": ["auth.js", "auth.test.js"]
}
```

**Use cases**:
- Send to external tools
- Store in database
- Process with jq/Python
- Send to Slack webhook

---

### Custom session detection

```bash
# Detect smaller sessions (15 min gaps instead of 30)
$ worklie sessions --gap 15

Work Sessions Today
===================

Session 1 (9:00-9:23) - 23m - Debugging
  Commands: 12
  Focus Score: 0.92 / 1.0

Session 2 (9:25-11:45) - 140m - Feature Development
  Commands: 48
  Focus Score: 0.78 / 1.0

Session 3 (2:00-3:15) - 75m - Testing
  Commands: 22
  Focus Score: 0.65 / 1.0
```

**Use cases**:
- Find micro-focus sessions
- Identify meeting interruptions
- Granular productivity tracking

---

### Generate PR descriptions

```bash
$ worklie pr

PR Description
===============

## Summary

3 commits, 2 activities

## Changes

- Fixed authentication bug
- Added API endpoint tests
- Updated Docker configuration
```

**Saves time**: No manual PR writing needed ⚡

---

### Track productivity over time

```bash
#!/bin/bash
# Add to crontab: 0 17 * * * ~/scripts/track_productivity.sh

worklie metrics --json >> ~/worklie_history.jsonl

# Later, analyze your productivity:
cat ~/worklie_history.jsonl | jq '.[] | .FOCUS_PATTERNS'
```

---

## 📱 Shell Integration

### Bash/Zsh Alias

```bash
# Add to ~/.bashrc or ~/.zshrc

alias ws="worklie standup"          # ws = work standup
alias wm="worklie metrics"          # wm = work metrics
alias wp="worklie portfolio"        # wp = work portfolio
alias wf="worklie focus"            # wf = work focus
alias wc="worklie switches"         # wc = context switches

# Usage:
$ ws              # See standup
$ wm              # See metrics
$ wf              # See focus times
```

---

### Automated Daily Standup to Slack

```bash
#!/bin/bash
# File: ~/scripts/daily_standup.sh
# Add to crontab: 0 9 * * MON-FRI ~/scripts/daily_standup.sh

STANDUP=$(worklie standup)
PAYLOAD=$(jq -n --arg msg "$STANDUP" '{text: $msg}')

curl -X POST -H 'Content-type: application/json' \
  --data "$PAYLOAD" \
  $SLACK_WEBHOOK_URL
```

---

### Weekly Metrics Report

```bash
#!/bin/bash
# File: ~/scripts/weekly_report.sh
# Add to crontab: 0 17 * * FRI ~/scripts/weekly_report.sh

{
    echo "📊 Weekly Productivity Report"
    echo "=============================="
    echo ""
    worklie metrics
    echo ""
    echo "📈 Work Classification"
    echo "======================"
    worklie classify
} | mail -s "Weekly Report" me@example.com
```

---

## 🎯 Best Practices

### 1. Understanding Sessions
- Sessions are automatically grouped by 30-minute gaps
- Change with `--gap` parameter for different analysis
- 30 minutes is natural context switch boundary

### 2. Work Classification
- Use commit prefixes for better classification:
  ```bash
  git commit -m "fix: bug in auth"      # Debugging
  git commit -m "feat: new endpoint"    # Feature Dev
  git commit -m "refactor: cleanup"     # Refactoring
  ```

### 3. Regular Tracking
- Run `worklie metrics` weekly
- Track trends over time
- Identify productivity patterns

### 4. Privacy
- All data stays on your machine
- No cloud sync, no telemetry
- Share output selectively

### 5. Integration
- Use `--json` flag for external tools
- Pipe to `jq` for filtering
- Store in personal database

---

## 🚨 Common Issues & Fixes

### Issue: "No data showing"

**Check 1: Do you have git commits?**
```bash
git log --oneline | head -5
# If empty: commit something first
```

**Check 2: Do you have shell history?**
```bash
history | head -5
# If empty: commands need timestamps (Zsh enables this)
```

**Check 3: Is Zsh history timestamped?**
```bash
# Add to ~/.zshrc:
setopt EXTENDED_HISTORY
setopt HIST_SAVE_BY_COPY
```

---

### Issue: "Sessions not detected"

**Reason**: Default gap is 30 minutes
**Solution**: Use smaller gap
```bash
worklie sessions --gap 15  # 15 minutes instead
```

---

### Issue: "Commands showing as 'Other Work'"

**Reason**: Commits don't match pattern keywords
**Solution**: Use standard commit prefixes
```bash
git commit -m "fix: ..."          # Will be classified as Debugging
git commit -m "feat: ..."         # Will be Feature Development
git commit -m "refactor: ..."     # Will be Refactoring
```

---

## 💡 Pro Tips

### Tip 1: Daily Standup Routine
```bash
# Alias in ~/.bashrc
alias ready_standup='worklie standup | pbcopy && echo "✅ Standup copied to clipboard"'

# Usage:
$ ready_standup
# Then paste into Slack/Email
```

### Tip 2: Portfolio Generation Before Interviews
```bash
# Generate and commit to your CV repo
worklie portfolio --format markdown > ~/Documents/CV/work_summary.md
```

### Tip 3: Productivity Dashboard
```bash
# Terminal watch (updates every 5s)
watch -n 5 'worklie metrics'
```

### Tip 4: Export Weekly to CSV
```bash
# For spreadsheet tracking
worklie weekly --json | jq '.[] | [.project, .total_commits, .total_activities] | @csv'
```

### Tip 5: Find Your Best Time
```bash
# Run weekly to find patterns
worklie focus

# Look for: Best time window, best day
# Then protect that time on calendar
```

---

## 🎓 Learning Path

**Day 1**: Learn basic commands
```bash
worklie report      # See what worklie does
worklie standup     # For meetings
worklie metrics     # For self-awareness
```

**Week 1**: Integrate into workflow
```bash
# Add aliases
# Use for daily standups
# Check metrics
```

**Month 1**: Optimize schedule
```bash
# See focus patterns
# Identify costly context switches
# Adjust calendar based on data
```

**Ongoing**: Career tracking
```bash
# Generate portfolio regularly
# Track skill development
# Share achievements
```

---

## 📚 Additional Resources

- **README.md** - Feature overview
- **ARCHITECTURE.md** - Technical deep dive
- **PHASE2_RELEASE_NOTES.md** - Phase 2 details
- **CLI_GUIDE.md** - Comprehensive reference
- **GitHub Issues** - Report bugs, request features

---

## 🎉 Ready to Go!

You now have:
✅ 15 commands
✅ JSON export
✅ Real-time metrics
✅ Career tracking
✅ Productivity analysis

**Start with**: `worklie standup`

**Questions?** Check the docs or open an issue on GitHub.

**Happy shipping!** 🚀
