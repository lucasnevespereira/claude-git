# claude-git

AI-powered git helpers using [Claude CLI](https://docs.anthropic.com/en/docs/claude-code).

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/lucasnevespereira/claude-git/main/install.sh | bash
```

Requires: [Claude CLI](https://docs.anthropic.com/en/docs/claude-code)

## Commands

| Command | Alias | What it does |
|---------|-------|-------------|
| `claude-git msg` | `gpm` | Propose a commit message |
| `claude-git commit` | `gac` | Propose + commit with confirmation |
| `claude-git commit -y` | `gacf` | Propose + commit, skip confirmation |
| `claude-git review` | `grev` | Review changes for bugs and issues |
| `claude-git pr [base]` | `gpr` | Generate a PR description |
| `claude-git explain` | `gex` | Explain what the changes do |

## Config

Create `~/.claude-git` or set environment variables:

```bash
CLAUDE_GIT_MODEL=haiku        # Model to use (default: haiku)
CLAUDE_GIT_MAX_DIFF=5000      # Max diff lines (default: 5000)
```

## Uninstall

```bash
claude-git uninstall
```
