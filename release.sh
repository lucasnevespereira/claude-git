#!/usr/bin/env bash
set -euo pipefail

# Usage: ./release.sh patch|minor|major

TYPE="${1:-}"

if [[ ! "$TYPE" =~ ^(patch|minor|major)$ ]]; then
  echo "Usage: ./release.sh <patch|minor|major>"
  exit 1
fi

# Read current version from script
CURRENT=$(grep '^VERSION=' claude-git | cut -d'"' -f2)
IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT"

case "$TYPE" in
  patch) PATCH=$((PATCH + 1)) ;;
  minor) MINOR=$((MINOR + 1)); PATCH=0 ;;
  major) MAJOR=$((MAJOR + 1)); MINOR=0; PATCH=0 ;;
esac

NEW="${MAJOR}.${MINOR}.${PATCH}"

# Update version in script
sed -i '' "s/^VERSION=\".*\"/VERSION=\"${NEW}\"/" claude-git

# Commit, tag, push
git add claude-git
git commit -m "release: v${NEW}"
git tag "v${NEW}"
git push && git push --tags

echo "Released v${NEW}"
