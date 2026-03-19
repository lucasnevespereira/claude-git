#!/usr/bin/env bash
set -euo pipefail

# claude-git installer
# curl -fsSL https://raw.githubusercontent.com/lucasnevespereira/claude-git/main/install.sh | bash

REPO="lucasnevespereira/claude-git"
INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="claude-git"

echo "Installing claude-git..."

# Check for claude CLI
if ! command -v claude &>/dev/null; then
  echo ""
  echo "Error: 'claude' CLI not found."
  echo "Install it first: https://docs.anthropic.com/en/docs/claude-code"
  exit 1
fi

# Create install dir
mkdir -p "$INSTALL_DIR"

# Download
curl -fsSL "https://raw.githubusercontent.com/${REPO}/main/claude-git" -o "$INSTALL_DIR/$BINARY_NAME"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

# Check if install dir is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
  SHELL_RC="$HOME/.zshrc"
  [[ "$SHELL" == */bash ]] && SHELL_RC="$HOME/.bashrc"

  if ! grep -q '.local/bin' "$SHELL_RC" 2>/dev/null; then
    echo '' >> "$SHELL_RC"
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
    echo "Added $INSTALL_DIR to PATH in $SHELL_RC"
  fi
fi

# Offer to install aliases
SHELL_RC="$HOME/.zshrc"
[[ "$SHELL" == */bash ]] && SHELL_RC="$HOME/.bashrc"

if ! grep -q 'claude-git aliases' "$SHELL_RC" 2>/dev/null; then
  echo '' >> "$SHELL_RC"
  echo '# claude-git aliases' >> "$SHELL_RC"
  echo 'source <(claude-git aliases)' >> "$SHELL_RC"
  echo "Added aliases to $SHELL_RC"
fi

echo ""
echo "Installed! Run: source $SHELL_RC"
echo ""
echo "Commands:"
echo "  claude-git msg       Propose a commit message"
echo "  claude-git commit    Propose + commit"
echo "  claude-git review    Review changes for bugs"
echo "  claude-git pr        Generate PR description"
echo "  claude-git explain   Explain current changes"
echo ""
echo "Aliases: gpm, gac, gacf, grev, gpr, gex"
