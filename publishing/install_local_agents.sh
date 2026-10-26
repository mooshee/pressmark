#!/usr/bin/env zsh
# Author: Daniel Hallman
set -euo pipefail

cd "$(dirname "$0")/.."

if [[ -d "$HOME/.gemini/config/plugins/pressmark-plugin" ]]; then
  cp -R publishing/generated/antigravity/* "$HOME/.gemini/config/plugins/pressmark-plugin/"
fi

if [[ -d "$HOME/.agents/plugins/pressmark" ]]; then
  cp plugin.json "$HOME/.agents/plugins/pressmark/plugin.json"
  cp README.md "$HOME/.agents/plugins/pressmark/README.md"
  cp -R skills/* "$HOME/.agents/plugins/pressmark/skills/"
  cp -R assets/* "$HOME/.agents/plugins/pressmark/assets/"
fi
