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

# Generated output is a draft. It can contain placeholder review fields and
# OAuth discovery metadata that does not apply to this guest connection.
# Package the reviewed repository files instead; never overwrite them here.
cargo run --locked --manifest-path tools/pressmark-package/Cargo.toml -- . dist
