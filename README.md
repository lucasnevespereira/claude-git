# claude-git

AI-powered git helpers using [Claude Code](https://docs.anthropic.com/en/docs/claude-code). Commit messages, code review, PR descriptions — all from your terminal.

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/lucasnevespereira/claude-git/main/install.sh | bash
```

Requires [Claude Code CLI](https://docs.anthropic.com/en/docs/claude-code).

## Usage

```bash
claude-git msg                # propose a commit message
claude-git commit             # propose + commit with confirmation
claude-git commit -y          # propose + commit, skip confirmation
claude-git review             # review changes for bugs
claude-git pr                 # generate PR description
claude-git pr develop         # PR description against a different base
claude-git explain            # explain what the changes do
```

### Aliases

The installer adds short aliases automatically:

| Alias | Command |
|-------|---------|
| `gpm` | `claude-git msg` |
| `gac` | `claude-git commit` |
| `gacf` | `claude-git commit --yes` |
| `grev` | `claude-git review` |
| `gpr` | `claude-git pr` |
| `gex` | `claude-git explain` |

## Config

Create `~/.claude-git` or set environment variables:

```bash
CLAUDE_GIT_MODEL=haiku        # model to use (default: haiku)
CLAUDE_GIT_MAX_DIFF=5000      # max diff lines (default: 5000)
```

## Uninstall

```bash
claude-git uninstall
```

## License

[MIT](LICENSE)
