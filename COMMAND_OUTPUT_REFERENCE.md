# Worklie Command Output Reference

Complete examples of all worklie commands with real output from test data.

---

## 1. STANDUP REPORT

```
$ worklie standup

Standup Report
==============

Yesterday
---------
• General Activity
• Code Changes
• Code Changes
• Code Changes
• Code Changes
• Code Changes
• Code Changes

Today
-----
• Continue with ongoing tasks

Blockers
--------
None detected
```

**Use Case**: Daily standup meetings - copy/paste this into your team chat.

---

## 2. DAILY REPORT

```
$ worklie report

Daily Work Summary
==================

Project: worklie-demo

Activities Detected:
  • General Activity
  • Code Changes
  • Code Changes
  • Code Changes
  • Code Changes
  • Code Changes
  • Code Changes

Commits:
  • 1 commits
    • test: add auth endpoint tests

Files Modified:
  • No files modified

Focus Areas:
  • refactoring
```

**Use Case**: End-of-day summary of what you accomplished.

---

## 3. WORK SESSIONS

```
$ worklie sessions

Work Sessions Today
===================

Session 1 (1773831540 UTC) - 20m - Testing
  Commands: 5
  Commits: 0
  Focus Score: 0.30 / 1.0

Session 2 (1773835140 UTC) - 35m - Testing
  Commands: 8
  Commits: 1
  Focus Score: 0.30 / 1.0
  Work: test: add auth endpoint tests

Session 3 (1773842340 UTC) - 20m - Testing
  Commands: 5
  Commits: 0
  Focus Score: 0.30 / 1.0

Session 4 (1773845940 UTC) - 10m - Testing
  Commands: 3
  Commits: 0
  Focus Score: 0.30 / 1.0

Session 5 (1773849540 UTC) - 5m - Other Work
  Commands: 2
  Commits: 0
  Focus Score: 0.30 / 1.0

Session 6 (1773853140 UTC) - 10m - DevOps/Infrastructure
  Commands: 3
  Commits: 0
  Focus Score: 0.30 / 1.0

SUMMARY:
  Total work time: 1h 40
  Sessions: 6
  Average duration: 16m
  Average focus: 0.30 / 1.0
```

**Use Case**: Understanding your work patterns - when did you get deep work in?

**Interpretation**:
- Focus Score 0.30 = scattered work (gaps between commands)
- Multiple sessions show context switching
- See what types of work each session involved

---

## 4. WORK CLASSIFICATION

```
$ worklie classify

Work Classification
===================

  • Testing: 67% (4 sessions)
  • DevOps/Infrastructure: 17% (1 sessions)
  • Other Work: 17% (1 sessions)
```

**Use Case**: Interview prep - "What sort of work do I do?" or resume building.

**Interpretation**:
- Majority of time spent on Testing
- DevOps/Infrastructure second
- Can be used to profile your skills

---

## 5. CONTEXT SWITCHING ANALYSIS

```
$ worklie switches

Context Switching Analysis
==========================

Total switches: 5
Estimated recovery time: ~15 minutes per switch
Estimated lost productivity: ~75m (1.2h)

⚠️  Context switching cost is significant!
Recommendation: Batch your work by task type
```

**Use Case**: Productivity optimization - "Why am I exhausted?"

**Interpretation**:
- 5 context switches × 15 min = 75 minutes lost
- That's 1.2 hours of wasted time just switching focus
- Recommendation: Block focus time on calendar

---

## 6. DEEP WORK DETECTION

```
$ worklie focus

Deep Work Sessions
==================

No deep work sessions (2+ hours) detected
```

**Or when deep work IS detected:**

```
$ worklie focus

Deep Work Sessions
==================

Tuesday - 3h 20m (Backend API Development)
Thursday - 2h 15m (Authentication System)
Friday - 1h 15m (Docker Configuration)

SUMMARY:
  Total deep work: 6h 50m
  Best focus day: Tuesday
  Best focus time: 9:00 AM - 12:00 PM

RECOMMENDATIONS:
  ✓ Tuesday mornings are your best time
  ✓ Protect 9-12 AM time block
  ✓ Schedule important work then
```

**Use Case**: Calendar optimization - "When should I schedule important work?"

**Interpretation**:
- Deep work sessions are 2+ hours uninterrupted
- Shows which days/times you're most productive
- Use this to optimize your calendar

---

## 7. PRODUCTIVITY METRICS

```
$ worklie metrics

Worklie Productivity Metrics
=============================

TIME ANALYSIS:
  Total work time: 1h 40m
  Deep work: 0h 0m (0%)
  Sessions: 6
  Average session: 16m

COMMITS & CODE:
  Total commits: 7
  Avg commits/day: 1

FOCUS PATTERNS:
  Average focus: 0.30 / 1.0

RECOMMENDATIONS:
→ Try to reduce context switching
```

**Use Case**: One-on-ones with manager or self-reflection.

**Interpretation**:
- Total work time: How much did I work?
- Deep work %: Am I getting focused time?
- Sessions: How fragmented was my day?
- Commits/day: Productivity metric
- Focus patterns: Where can I improve?

---

## 8. PORTFOLIO GENERATION

```
$ worklie portfolio --format markdown

## Engineering Work Summary

### worklie-demo

#### Work Session 1

- Duration: 20m
- Type: Testing
- Commits: 0

#### Work Session 2

test: add auth endpoint tests

- Duration: 35m
- Type: Testing
- Commits: 1

#### Work Session 3

- Duration: 20m
- Type: Testing
- Commits: 0

#### Work Session 4

- Duration: 10m
- Type: Testing
- Commits: 0

#### Work Session 5

- Duration: 5m
- Type: Other Work
- Commits: 0

#### Work Session 6

- Duration: 10m
- Type: DevOps/Infrastructure
- Commits: 0
```

**Use Case**: Update LinkedIn, resume, portfolio website.

**Export to file**:
```bash
worklie portfolio --format markdown > my_work_$(date +%Y-%m-%d).md
```

---

## 9. PR DESCRIPTION

```
$ worklie pr

PR Description
===============

## Summary

7 commits, 7 activities

## Changes

- General Activity
- Code Changes
- Code Changes
- Code Changes
- Code Changes
- Code Changes
- Code Changes
```

**Use Case**: Generate initial PR description, then refine.

**How to use**:
```bash
worklie pr | pbcopy  # macOS - copy to clipboard
worklie pr | xclip   # Linux - copy to clipboard
```

---

## 10. CHANGELOG GENERATION

```
$ worklie changelog

Changelog
=========

### Features
- feat: add jwt validation
- feat: implement authentication system

### Bug Fixes
- fix: token expiry validation bug

### Documentation
- docs: add authentication guide

### Other Changes
- chore: add docker configuration
- refactor: simplify token verification logic
- test: add auth endpoint tests
```

**Use Case**: Generate CHANGELOG.md for release notes.

**Export to file**:
```bash
worklie changelog > CHANGELOG_new.md
```

---

## 11. JSON OUTPUT (All Commands Support `--json`)

```
$ worklie report --json

{
  "project": "worklie-demo",
  "domain": null,
  "work_type": "refactoring",
  "branch": "Koundinya-Wedding",
  "activities": [
    0,
    1,
    2,
    3,
    4,
    5,
    6
  ],
  "total_commits": 1,
  "commit_messages": [
    "test: add auth endpoint tests"
  ],
  "files_modified": []
}
```

**Use Case**: Feed data to external tools, custom scripts, dashboards.

**Examples**:
```bash
# Extract commit messages
worklie report --json | jq '.commit_messages'

# Get total commits
worklie report --json | jq '.total_commits'

# Save to file
worklie report --json > report_$(date +%Y-%m-%d).json
```

---

## 12. COMMAND-LINE ALIASES (Recommended Setup)

Add to your `~/.zshrc` or `~/.bashrc`:

```bash
# Worklie aliases
alias ws="worklie standup"      # ws = work standup
alias wr="worklie report"       # wr = work report
alias wm="worklie metrics"      # wm = work metrics
alias wf="worklie focus"        # wf = work focus
alias wc="worklie switches"     # wc = context switches
alias wp="worklie portfolio"    # wp = work portfolio
```

**Usage**:
```bash
$ ws                            # See standup
$ wm --json | jq '.total_commits'  # Extract metrics
$ wp --format markdown > cv.md  # Export portfolio
```

---

## 📊 Real-World Workflows

### Morning Standup

```bash
# At 9 AM - prepare for standup meeting
worklie standup | pbcopy

# Paste into Slack/Teams
```

### Daily Metrics Check

```bash
# End of day - understand productivity
worklie metrics
worklie switches  # See if I'm too scattered
```

### Weekly Productivity Review

```bash
# Every Friday - review the week
worklie focus      # Find my best times
worklie classify   # What skills did I develop?
worklie switches   # How can I improve focus?
```

### Interview Preparation

```bash
# Before interview - prepare talking points
worklie portfolio --format markdown > interview_prep.md

# Shows:
# - What I've built
# - Skills demonstrated
# - Quantified work (commits, hours)
```

### Performance Review

```bash
# Supply manager with data
worklie metrics --json > my_metrics_$(date +%Y).json

# Shows:
# - Total work time
# - Productivity trends
# - Skill distribution
# - Focus patterns
```

---

## 🎯 Key Metrics Interpreted

| Metric | Low | Medium | High | What It Means |
|--------|-----|--------|------|---------------|
| **Deep Work %** | <10% | 10-30% | >30% | Uninterrupted focus time |
| **Sessions/Day** | 1-2 | 3-5 | >5 | Fragmentation (higher = worse) |
| **Context Switches** | 0-2 | 3-5 | >5 | Attention taxation (lower = better) |
| **Focus Score** | <0.3 | 0.3-0.7 | >0.7 | Concentration level |
| **Commits/Day** | <1 | 1-5 | >5 | Code activity (context dependent) |

---

## 🚀 Tips for Best Results

1. **Commit often with clear messages**:
   ```bash
   git commit -m "feat: add new feature"  # Good
   git commit -m "fix: bug X"             # Good
   git commit -m "update"                 # Bad - too vague
   ```

2. **Use standard commit prefixes**:
   - `feat:` - New feature
   - `fix:` - Bug fix
   - `refactor:` - Code restructure
   - `test:` - Test additions
   - `docs:` - Documentation
   - `chore:` - Maintenance

3. **Keep shell history**:
   - Don't clear history frequently
   - Enable timestamps in zsh: `setopt EXTENDED_HISTORY`
   - Use worklie weekly for trend data

4. **Export regularly**:
   ```bash
   worklie metrics --json >> ~/worklie_history.jsonl
   ```

---

## 📈 Data Visualization Ideas

**Export weekly metrics and track:**
```bash
# Collect weekly metrics
echo "Date,DeepWork%,Sessions,Commits" > metrics.csv
for week in {1..12}; do
  worklie metrics --json | jq -r '"\(now | todate),\(.deep_work_percent),\(.sessions),\(.commits)"' >> metrics.csv
done

# Import to Excel/Google Sheets for trending
```

---

**For more examples, visit**: `/Users/vamanakhil/worklie/QUICK_START.md`
**For detailed docs**, see: `/Users/vamanakhil/worklie/CLI_GUIDE.md`
