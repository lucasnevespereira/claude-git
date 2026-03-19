# claude-git

AI-powered git helpers using [Claude](https://www.anthropic.com/claude). Commit messages, code review, PR descriptions — all from your terminal.

Works with the [Claude Code CLI](https://docs.anthropic.com/en/docs/claude-code) or directly via the [Anthropic API](https://docs.anthropic.com/en/docs/initial-setup).

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/lucasnevespereira/claude-git/main/install.sh | bash
```

## Usage

```bash
claude-git msg                # propose a commit message
claude-git commit             # propose + commit with confirmation
claude-git commit -y          # propose + commit, skip confirmation
claude-git prefix             # propose + commit with prefix (auto-detects from branch)
claude-git prefix PROJ-42    # propose + commit with explicit prefix
claude-git review             # review changes for bugs
claude-git pr                 # generate PR description
claude-git pr develop         # PR description against a different base
claude-git explain            # explain what the changes do
```

### Aliases

The installer adds short aliases automatically:

| Alias | Command |
|-------|---------|
| `cg`   | `claude-git` |
| `cgm`  | `claude-git msg` |
| `cgc`  | `claude-git commit` |
| `cgcy` | `claude-git commit --yes` |
| `cgrev` | `claude-git review` |
| `cgpr` | `claude-git pr` |
| `cgex` | `claude-git explain` |
| `cgpx` | `claude-git prefix` |

## Config

```bash
claude-git config                    # show current config
claude-git config model sonnet       # use a different model
claude-git config model haiku        # back to fast mode
claude-git config max_lines 5000     # send more context to Claude
claude-git config api_key sk-ant-... # use direct API calls (faster)
```

| Key | Default | Description |
|-----|---------|-------------|
| `model` | `haiku` | Claude model to use |
| `max_lines` | `2000` | Max diff lines sent to Claude (prevents slow/expensive calls on large diffs) |
| `api_key` | _(not set)_ | Anthropic API key — enables direct API calls, skipping the Claude CLI for faster responses |
| `mode` | `auto` | `auto` (API with CLI fallback), `api` (API only), or `cli` (CLI only) |

Config is stored in `~/.claude-git`.

## Update

```bash
claude-git upgrade
```

## Uninstall

```bash
claude-git uninstall
```

## License

[MIT](LICENSE)
