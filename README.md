# claude-git

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![GitHub release](https://img.shields.io/github/v/release/lucasnevespereira/claude-git)](https://github.com/lucasnevespereira/claude-git/releases)

A CLI that generates commit messages, code reviews, and PR descriptions using [Claude](https://www.anthropic.com/claude).

Works with the [Claude Code CLI](https://docs.anthropic.com/en/docs/claude-code) or directly via the [Anthropic API](https://docs.anthropic.com/en/docs/initial-setup).

## Why?

Sometimes I'd spend more time thinking about a commit message than writing the actual code. This tool reads your diff and lets Claude propose one for you.

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/lucasnevespereira/claude-git/main/install.sh | bash
```

## Usage

| Command                     | What it does                                             |
| --------------------------- | -------------------------------------------------------- |
| `claude-git msg`            | Propose a commit message from your staged changes        |
| `claude-git commit`         | Propose a message and commit with confirmation           |
| `claude-git commit -y`      | Commit without confirmation                              |
| `claude-git prefix`         | Commit with a prefix auto-detected from your branch name |
| `claude-git prefix PROJ-42` | Commit with an explicit prefix                           |
| `claude-git review`         | Review your changes for potential bugs and issues        |
| `claude-git pr`             | Generate a PR description from your branch diff          |
| `claude-git pr develop`     | PR description against a different base branch           |
| `claude-git explain`        | Get a plain-English explanation of what your changes do  |

### Aliases

The installer adds short aliases automatically:

| Alias   | Command                   |
| ------- | ------------------------- |
| `cg`    | `claude-git`              |
| `cgm`   | `claude-git msg`          |
| `cgc`   | `claude-git commit`       |
| `cgcy`  | `claude-git commit --yes` |
| `cgrev` | `claude-git review`       |
| `cgpr`  | `claude-git pr`           |
| `cgex`  | `claude-git explain`      |
| `cgpx`  | `claude-git prefix`       |

## Config

```bash
claude-git config                    # show current config
claude-git config model sonnet       # use a different model
claude-git config max_lines 5000     # send more context to Claude
claude-git config api_key sk-ant-... # use direct API calls (faster)
```

| Key         | Default     | Description                                                |
| ----------- | ----------- | ---------------------------------------------------------- |
| `model`     | `haiku`     | Claude model (`haiku`, `sonnet`, `opus`)                   |
| `max_lines` | `2000`      | Max diff lines sent to Claude                              |
| `api_key`   | _(not set)_ | Anthropic API key for direct API calls (faster than CLI)   |
| `mode`      | `auto`      | `auto`, `api`, or `cli`                                    |

Config is stored in `~/.claude-git`.

## Update / Uninstall

```bash
claude-git upgrade     # update to latest version
claude-git uninstall   # remove claude-git and its config
```

## Contributing

If you have ideas or find bugs, open an issue. PRs are welcome too.

## License

[MIT](LICENSE)
