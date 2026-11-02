#!/usr/bin/env bash

set -e

cd "$(dirname "$0")/../.."

version=$(node -p 'require("../package.json").version')
branch=$(git branch --show-current)

if ! command -v gh >/dev/null 2>&1; then
  echo "❌ Install the GitHub CLI from https://cli.github.com to release the desktop app." >&2
  exit 1
fi

if ! gh auth status >/dev/null 2>&1; then
  echo "🔑 Logging in to GitHub..."
  gh auth login
fi

echo "🚀 Starting the desktop release for v$version..."
gh workflow run release.yml --repo PolsarCode/orbis.desktop --ref "${branch:-main}" -f version="$version"
echo "✅ GitHub is building the installers. Follow along at https://github.com/PolsarCode/orbis.desktop/actions"
