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

```bash
claude-git config                    # show current config
claude-git config model sonnet       # use a different model
claude-git config model haiku        # back to fast mode
claude-git config max_lines 5000     # send more context to Claude
```

| Key | Default | Description |
|-----|---------|-------------|
| `model` | `haiku` | Claude model to use |
| `max_lines` | `2000` | Max diff lines sent to Claude (prevents slow/expensive calls on large diffs) |

Config is stored in `~/.claude-git`.

## Uninstall

```bash
claude-git uninstall
```

## License

[MIT](LICENSE)
