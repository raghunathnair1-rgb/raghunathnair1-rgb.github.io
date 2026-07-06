#!/usr/bin/env bash
# Ship the blog: commit + push -> GitHub Actions builds the WASM and deploys to Pages.
# Uses the scoped blog deploy key (configured as this repo's core.sshCommand).
set -euo pipefail
cd "$(dirname "$0")"
msg="${1:-blog: update $(date -u +%FT%TZ)}"
git add -A
if git diff --cached --quiet; then echo "nothing to ship"; exit 0; fi
git commit -m "$msg"
git push origin main
echo "✅ pushed — GitHub Actions is building Rust→WASM and deploying to Pages."
echo "   watch: https://github.com/raghunathnair1-rgb/raghunathnair1-rgb.github.io/actions"
