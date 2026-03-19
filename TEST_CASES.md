# Worklie Test Cases & Demonstrations

## ✅ Complete Feature Testing

This document describes test cases for demonstrating all worklie commands with realistic data.

---

## 🎯 Test Setup

### Prerequisites
```bash
# 1. Create a test git repository
mkdir -p /tmp/test-project && cd /tmp/test-project
git init
git config user.email "dev@example.com"
git config user.name "Test Developer"
```

### Create Test Git Commits

```bash
# Create commits with timestamps from the last 7 hours
NOW=$(date +%s)

# File 1: README
echo "# My Project" > README.md
git add README.md
GIT_AUTHOR_DATE="$((NOW - 25200))" GIT_COMMITTER_DATE="$((NOW - 25200))" \
  git commit -m "feat: implement authentication system"

# File 2: Source code
echo "auth logic" > auth.js
git add auth.js
GIT_AUTHOR_DATE="$((NOW - 21600))" GIT_COMMITTER_DATE="$((NOW - 21600))" \
  git commit -m "feat: add jwt validation"

# File 3: Tests
echo "test auth" > auth.test.js
git add auth.test.js
GIT_AUTHOR_DATE="$((NOW - 18000))" GIT_COMMITTER_DATE="$((NOW - 18000))" \
  git commit -m "test: add auth endpoint tests"

# Bug fix
echo "auth fix" >> auth.js
git add auth.js
GIT_AUTHOR_DATE="$((NOW - 14400))" GIT_COMMITTER_DATE="$((NOW - 14400))" \
  git commit -m "fix: token expiry validation bug"

# Refactoring
echo "refactored" >> auth.js
git add auth.js
GIT_AUTHOR_DATE="$((NOW - 10800))" GIT_COMMITTER_DATE="$((NOW - 10800))" \
  git commit -m "refactor: simplify token verification logic"

# Documentation
echo "## Usage" >> README.md
git add README.md
GIT_AUTHOR_DATE="$((NOW - 7200))" GIT_COMMITTER_DATE="$((NOW - 7200))" \
  git commit -m "docs: add authentication guide"

# DevOps
echo "FROM node:18" > Dockerfile
git add Dockerfile
GIT_AUTHOR_DATE="$((NOW - 3600))" GIT_COMMITTER_DATE="$((NOW - 3600))" \
  git commit -m "chore: add docker configuration"
```

### Create Test Shell History

Add to your `~/.zsh_history` file with timestamps from today:

```
: 1773831540:0;git status
: 1773831840:0;vim auth.js
: 1773832140:0;npm test
: 1773832440:0;git add auth.js
: 1773832740:0;git commit -m "feat: add jwt validation"
: 1773835140:0;npm test
: 1773835440:0;git log
: 1773835740:0;docker build .
: 1773838340:0;git status
: 1773838640:0;vim auth.test.js
: 1773838940:0;npm run test:integration
: 1773839240:0;git add .
: 1773839540:0;git commit -m "test: add auth endpoint tests"
: 1773845940:0;git pull origin main
: 1773846240:0;vim auth.js
: 1773846540:0;npm test
: 1773846840:0;git diff
: 1773847140:0;git commit -am "fix: token expiry validation bug"
: 1773849540:0;vim auth.js
: 1773849840:0;npm test
: 1773850140:0;git commit -am "refactor: simplify token verification logic"
: 1773853140:0;vim README.md
: 1773853440:0;git commit -am "docs: add authentication guide"
: 1773856740:0;vim Dockerfile
: 1773857040:0;docker build .
: 1773857340:0;git commit -am "chore: add docker configuration"
```

**Note**: Replace timestamps with actual Unix timestamps from today: `NOW=$(date +%s)`

---

## 🧪 Test Cases

### Test Case 1: Standup Report
**Command**: `worklie standup`

**Expected Output**:
- Yesterday section with activities detected
- Today section with next tasks
- Optional blockers section

**Success Criteria**:
- ✅ Should show 7+ activities from the test commits
- ✅ Should format cleanly with sections
- ✅ Should show relevant work types

---

### Test Case 2: Daily Report
**Command**: `worklie report`

**Expected Output**:
- Project name detected (test-project)
- Activities list with descriptions
- Commit count and messages
- Files modified count
- Focus areas detected

**Success Criteria**:
- ✅ Shows 7 commits detected
- ✅ Lists activity types (Code Changes, etc.)
- ✅ Shows focus area: refactoring

---

### Test Case 3: Work Sessions
**Command**: `worklie sessions`

**Expected Output**:
```
Work Sessions Today
===================

Session 1 (timestamp) - 20m - Testing
  Commands: 5
  Commits: 0
  Focus Score: 0.30 / 1.0

Session 2 (timestamp) - 35m - Testing
  Commands: 8
  Commits: 1
  Focus Score: 0.30 / 1.0
```

**Success Criteria**:
- ✅ Detects 4-6 sessions based on command gaps
- ✅ Calculates work type for each session
- ✅ Shows focus score (0.0 - 1.0)
- ✅ Shows total work time and average session duration

---

### Test Case 4: Work Classification
**Command**: `worklie classify`

**Expected Output**:
```
Work Classification
===================

  • Testing: 67% (4 sessions)
  • DevOps/Infrastructure: 17% (1 sessions)
  • Other Work: 17% (1 sessions)
```

**Success Criteria**:
- ✅ Shows percentage breakdown by work type
- ✅ Identifies Testing (from test commits)
- ✅ Identifies DevOps (from docker build commands)
- ✅ Totals to 100%

---

### Test Case 5: Context Switching Analysis
**Command**: `worklie switches`

**Expected Output**:
```
Context Switching Analysis
==========================

Total switches: 5
Estimated recovery time: ~15 minutes per switch
Estimated lost productivity: ~75m (1.2h)

⚠️  Context switching cost is significant!
```

**Success Criteria**:
- ✅ Counts session transitions (gaps > 30min)
- ✅ Estimates recovery time per switch
- ✅ Calculates total productivity loss
- ✅ Shows recommendations

---

### Test Case 6: Deep Work Detection
**Command**: `worklie focus`

**Expected Output**:
```
Deep Work Sessions
==================

No deep work sessions (2+ hours) detected
```

**Or** (if sessions > 2 hours):
```
Deep Work Sessions
==================

Tuesday - 3h 20m (Backend API)
Thursday - 2h 15m (Authentication)
```

**Success Criteria**:
- ✅ Identifies sessions >= 2 hours uninterrupted
- ✅ Shows work type for each deep session
- ✅ Provides time window recommendations

---

### Test Case 7: Productivity Metrics
**Command**: `worklie metrics`

**Expected Output**:
```
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
```

**Success Criteria**:
- ✅ Shows accurate total time
- ✅ Calculates deep work percentage
- ✅ Counts sessions and commits
- ✅ Provides actionable recommendations

---

### Test Case 8: Portfolio Generation
**Command**: `worklie portfolio --format markdown`

**Expected Output**:
```markdown
## Engineering Work Summary

### test-project

#### Work Session 1
- Duration: 20m
- Type: Testing
- Commits: 0

#### Work Session 2
test: add auth endpoint tests
- Duration: 35m
- Type: Testing
- Commits: 1
```

**Success Criteria**:
- ✅ Generates markdown-formatted output
- ✅ Shows work sessions with descriptions
- ✅ Includes commit messages
- ✅ Can be exported to file for resume/portfolio

---

### Test Case 9: PR Description
**Command**: `worklie pr`

**Expected Output**:
```
PR Description
===============

## Summary
7 commits, 7 activities

## Changes
- feat: implement authentication system
- feat: add jwt validation
- test: add auth endpoint tests
- fix: token expiry validation bug
- refactor: simplify token verification logic
```

**Success Criteria**:
- ✅ Generates formatted PR description
- ✅ Lists all commits with messages
- ✅ Organizes by type (feat, fix, etc.)
- ✅ Can be copied to GitHub PR body

---

### Test Case 10: Changelog Generation
**Command**: `worklie changelog`

**Expected Output**:
```
Changelog
=========

### Features
- feat: implement authentication system
- feat: add jwt validation

### Bug Fixes
- fix: token expiry validation bug

### Documentation
- docs: add authentication guide

### Refactoring
- refactor: simplify token verification logic

### Chores
- chore: add docker configuration
```

**Success Criteria**:
- ✅ Organizes commits by type
- ✅ Creates release-ready changelog format
- ✅ Shows all work done in period
- ✅ Can be copied to CHANGELOG.md

---

## 📋 Test Data Creation Script

**Save as `create_test_data.sh`**:

```bash
#!/bin/bash
set -e

TEST_DIR="${1:-.}"
cd "$TEST_DIR"

git init
git config user.email "test@example.com"
git config user.name "Test Developer"

NOW=$(date +%s)

# Create commits
for i in {1..7}; do
  OFFSET=$((NOW - (25200 - i * 3600)))
  echo "# Project" > file$i.txt
  git add file$i.txt
  GIT_AUTHOR_DATE="$OFFSET" GIT_COMMITTER_DATE="$OFFSET" \
    git commit -m "feat: commit $i"
done

# Create zsh_history
cat >> ~/.zsh_history << 'EOF'
# Your history entries here
EOF

echo "✅ Test data created in $TEST_DIR"
echo "Run: cd $TEST_DIR && worklie report"
```

---

## 🚀 Running the Full Test Suite

```bash
# Create test directory
mkdir -p /tmp/worklie-test
cd /tmp/worklie-test

# Initialize git repo with test data
git init
git config user.email "dev@example.com"
git config user.name "Developer"

# Create test commits (see above)
# ... [create commits as shown above] ...

# Update shell history
# ... [add entries to ~/.zsh_history] ...

# Run all tests
echo "1. Standup:" && worklie standup
echo "2. Report:" && worklie report
echo "3. Sessions:" && worklie sessions
echo "4. Classification:" && worklie classify
echo "5. Switching:" && worklie switches
echo "6. Focus:" && worklie focus
echo "7. Metrics:" && worklie metrics
echo "8. Portfolio:" && worklie portfolio --format markdown
echo "9. PR:" && worklie pr
echo "10. Changelog:" && worklie changelog
```

---

## ✅ Success Checklist

- [ ] All 10 commands run without errors
- [ ] Commits are detected (7 expected)
- [ ] Sessions are identified (4-6 expected)
- [ ] Work types are classified correctly
- [ ] Timestamps are recognized
- [ ] JSON output works (`--json` flags)
- [ ] Output is human-readable
- [ ] No panics or crashes
- [ ] Performance < 500ms
- [ ] All features work end-to-end

---

## 📝 Notes

1. **Timestamps Matter**: Both git commits and shell history need proper timestamps for correlation
2. **30-Minute Rule**: Sessions are split by 30-minute gaps by default
3. **24-Hour Window**: By default, worklie analyzes the last 24 hours
4. **Noise Filtering**: Very short commands (ls, cd, pwd) are automatically filtered
5. **Git Repository**: Commands must be run in a git repository

---

## 🔧 Troubleshooting

### Issue: "No commits detected"
**Solution**: Ensure git commits have timestamps within the last 24 hours

### Issue: "No sessions detected"
**Solution**: Check shell history has proper format (`: timestamp:0;command`)

### Issue: "0 activities"
**Solution**: Verify both git commits and shell commands have overlapping time ranges

### Issue: "Commands: 0"
**Solution**: Verify ~/.zsh_history exists and contains entries with timestamps

---

## 📞 Quick Start

**One-liner to create full test environment:**

```bash
bash -c '
mkdir -p /tmp/worklie-demo && cd /tmp/worklie-demo && \
git init && git config user.email "test@test.com" && git config user.name "Test" && \
NOW=$(date +%s) && \
for i in {0..6}; do
  echo "test $i" > file$i.txt
  git add file$i.txt
  GIT_AUTHOR_DATE="$((NOW - 25200 + i*3600))" GIT_COMMITTER_DATE="$((NOW - 25200 + i*3600))" \
    git commit -m "feat: commit $i"
done && \
echo "✅ Test repo created in /tmp/worklie-demo" && \
echo "Run: cd /tmp/worklie-demo && worklie report"
'
```

---

**Happy testing! 🚀**
