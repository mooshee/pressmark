#!/usr/bin/env zsh
# Author: Daniel Hallman
set -euo pipefail

cd "$(dirname "$0")/.."

agent-publish build \
  --product publishing/product.public.json \
  --out publishing/generated \
  --submission chatgpt-app-submission.json \
  --assets assets \
  --skills skills

cp publishing/generated/gemini/gemini-extension.json ./gemini-extension.json
cp publishing/generated/gemini/research-instructions.md ./research-instructions.md
cp publishing/generated/cursor/.cursor-plugin/plugin.json ./.cursor-plugin/plugin.json
cp publishing/generated/cursor/.cursor/mcp.json ./.cursor/mcp.json
cp publishing/generated/portable/plugin.json ./plugin.json
mkdir -p dist/pressmark/skills
cp publishing/generated/dist/pressmark-flat.zip dist/pressmark-flat.zip
cp publishing/generated/dist/pressmark-agent-plugin.zip dist/pressmark-agent-plugin.zip
cp plugin.json dist/pressmark/plugin.json
cp -R skills/* dist/pressmark/skills/
