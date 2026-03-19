# Worklie - Developer Activity Intelligence CLI

**Worklie** is a production-grade, local-first CLI tool that reconstructs developer daily and weekly work narratives from system activity traces.

## The Problem

Modern developers experience high-context switching:
- Run hundreds of terminal commands
- Switch between repositories
- Debug systems and run containers
- Write commits, read logs, test APIs

When standup meetings happen, developers struggle to recall their work accurately. **Worklie converts machine activity traces into human-readable work narratives automatically.**

## Philosophy

> "Your computer already knows what you worked on."

Worklie simply translates activity signals (shell history, git logs, timestamps) into structured work reports.

## Features

### Core Commands

#### `worklie report`
Generate a daily work summary with detected activities, commits, and files modified.

```bash
$ worklie report

Daily Work Summary
==================

Project: my-project
Activities Detected:
  • Fixed authentication bug
  • Updated API endpoints

Commits:
  • 2 commits
    • fix(auth): token validation
    • feat(api): new endpoint

Files Modified:
  • auth.js
  • api/routes.js

Focus Areas:
  • authentication
  • debugging
```

#### `worklie weekly`
Generate a weekly engineering report with aggregated metrics and contributions.

#### `worklie standup`
Generate a standup-optimized report with Yesterday/Today/Blockers structure.

#### `worklie stats`
Show engineering statistics (commands executed, commits made, unique commands).

### Advanced Features

#### `worklie focus`
Analyze your main focus areas and most-used tools.

```bash
$ worklie focus

Focus Analysis
==============

Primary Domain: authentication
Work Type: debugging

Most Used Tools:
  • git (152x)
  • npm (74x)
  • vim (45x)
```

#### `worklie changelog`
Generate a release changelog from recent commits, automatically categorizing features and bug fixes.

#### `worklie pr`
Generate a PR (Pull Request) description from your recent activity.

```bash
$ worklie pr

PR Description
===============

## Summary

5 commits, 3 activities

## Changes

- Fixed authentication bug
- Updated API error handling
- Improved Docker configuration
```

#### `worklie resume`
Generate resume bullets from your engineering work for portfolios or professional profiles.

## Architecture

Worklie uses a **4-layer production architecture**:

### Layer 1: Signal Collection
- **HistoryCollector**: Reads shell history from zsh/bash (with timestamps and noise filtering)
- **GitCollector**: Extracts git repository metadata and recent commits

### Layer 2: Parsing
- **HistoryParser**: Parses timestamped commands into structured data
- **GitParser**: Parses commit metadata (hash, timestamp, author, message)

### Layer 3: Analysis
- **ActivityClusterer**: Groups commands and commits into meaningful activities using temporal clustering (30-minute time windows)
- **ContextInferenceEngine**: Infers project context, domain, and work type from git metadata

### Layer 4: Reporting
- **DailyReportGenerator**: Generates human-readable daily reports and JSON output
- **WeeklyReportGenerator**: Aggregates daily data for weekly summaries
- **StandupReportGenerator**: Formats data for standup meetings

### Performance

- **First run**: ~400-550ms (includes filesystem I/O)
- **Subsequent runs**: ~50-100ms (benefits from OS filesystem caching)
- **Memory**: ~5-10MB typical usage
- **Data filtering**: Automatically limits to last 24 hours of recent commands

### Privacy & Security

✅ **Local-first**: All data processing happens on your machine
✅ **No telemetry**: Zero external API calls by default
✅ **Zero cloud tracking**: Your work stays on your computer
✅ **Open source**: Full source code transparency

## Installation

### From Source
```bash
git clone https://github.com/yourusername/worklie.git
cd worklie
cargo build --release
sudo mv target/release/worklie /usr/local/bin/
```

### Requirements
- Rust 1.70+
- Git (for repository metadata)
- Zsh or Bash with history enabled

## Usage Examples

### Daily Standup
```bash
$ worklie standup
```

### Weekly Review
```bash
$ worklie weekly
```

### Export as JSON (for integration)
```bash
$ worklie report --json > daily-report.json
```

### Analyze Focus Areas
```bash
$ worklie focus
```

### Generate Release Changelog
```bash
$ worklie changelog > CHANGELOG.md
```

### Create PR Description
```bash
$ worklie pr
```

### Build Portfolio Bullets
```bash
$ worklie resume
```

## How It Works

### 1. Signal Collection
Worklie reads:
- **Shell history**: Commands from `~/.zsh_history` or `~/.bash_history`
- **Timestamps**: Extracted from zsh history format (`: timestamp:0;command`)
- **Git metadata**: Recent commits, branch info, author details

### 2. Intelligent Filtering
- Removes noise (ls, cd, pwd, etc.)
- Limits to last 24 hours
- Caps at 1000 recent commands for efficiency

### 3. Activity Clustering
Groups commands into tasks using:
- **Temporal proximity**: 30-minute time windows
- **Semantic context**: Git commit associations
- **File extraction**: Identifies modified files

### 4. Context Inference
Determines work type from:
- Git branch names (feature-*, bugfix-*, etc.)
- Commit message patterns (fix:, feat:, etc.)
- Recent command patterns

### 5. Report Generation
Outputs in multiple formats:
- **Plain text**: Human-readable formatted reports
- **JSON**: Structured data for integration and automation

## Architecture Decisions

### Why Rust?
- ✅ Fast startup and execution (<100ms after caching)
- ✅ Memory efficient (minimal allocations)
- ✅ Single binary distribution (no runtime dependencies)
- ✅ Production-grade error handling

### Why Local-First?
- ✅ User privacy and control
- ✅ Offline capability
- ✅ No latency from API calls
- ✅ No data leaving user's machine

### Why Command-Line?
- ✅ Easy shell integration and piping
- ✅ Perfect for CI/CD pipelines
- ✅ Scriptable and automation-friendly
- ✅ Works with existing developer tools

## Configuration

Shell history configuration (already works out-of-the-box):

**Zsh** (~/.zshrc):
```bash
setopt EXTENDED_HISTORY  # Add timestamps to history
HISTFILE=~/.zsh_history
HISTSIZE=50000
SAVEHIST=50000
```

**Bash** (~/.bashrc):
```bash
export HISTFILE=~/.bash_history
export HISTFILESIZE=50000
export HISTSIZE=50000
```

## Project Context

This tool was inspired by observing that:
1. Developers generate massive amounts of activity traces daily
2. These traces contain complete information about work done
3. Human memory is episodic and unreliable for work reconstruction
4. The gap between "machine knows what I did" and "I remember what I did" is unacceptable

Worklie bridges this gap with **behavioral telemetry interpretation** - the same principle used in:
- Distributed systems observability
- Application performance monitoring
- Development analytics

## Contributing

Contributions welcome! Areas for enhancement:
- [ ] Additional collectors (file modification times, cloud IDE activity)
- [ ] Machine learning for smarter activity clustering
- [ ] Export to external tools (Jira, Linear, Asana)
- [ ] Real-time activity monitoring daemon
- [ ] Web dashboard for historical analysis

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

**Test Coverage**: 15 unit tests covering collectors, parsers, clustering, and reporting

## License

MIT License - See LICENSE file for details

## Performance Benchmarks

| Operation | Time | Notes |
|-----------|------|-------|
| First run (full scan) | 400-550ms | Includes filesystem I/O |
| Cached run | 50-100ms | Filesystem cache benefit |
| Memory usage | 5-10MB | Typical session |
| Max history limit | 1000 commands | Configurable |

## Roadmap

- [ ] v0.2.0: Caching layer with persistent storage
- [ ] v0.3.0: Multi-repository support
- [ ] v0.4.0: Web UI for visualization
- [ ] v0.5.0: AI-powered activity categorization

---

**Status**: Production-Ready (v0.1.0)
**Last Updated**: March 2026
