#!/usr/bin/env bash
# Ship the blog through the dark-factory gate, then push -> GitHub Actions builds
# the WASM and deploys to Pages. Uses the scoped blog deploy key (core.sshCommand).
set -euo pipefail
cd "$(dirname "$0")"
msg="${1:-blog: update $(date -u +%FT%TZ)}"
SEC=/home/raghu/harness/security

# --- snapshot the real harness-brain status for the blog's brain widget ---
bstate=$(systemctl --user is-active harness-brain 2>/dev/null || echo unknown)
bstart=$(systemctl --user show harness-brain -p ActiveEnterTimestamp --value 2>/dev/null)
bepoch=$(date -d "$bstart" +%s 2>/dev/null || echo 0)
bpid=$(systemctl --user show harness-brain -p MainPID --value 2>/dev/null || echo 0)
printf '{"healthy": %s, "service": "harness-brain.service", "started_epoch": %s, "pid": %s, "updated_epoch": %s}\n' \
  "$([ "$bstate" = active ] && echo true || echo false)" "${bepoch:-0}" "${bpid:-0}" "$(date +%s)" > status.json
echo "🧠 brain snapshot: $bstate (pid ${bpid:-0})"

# --- dark-factory pre-deploy security gate (SAST + secret scan) ---
echo "🛡️  pre-deploy gate…"
if [ -x "$SEC/bin/opengrep" ]; then
  "$SEC/bin/opengrep" scan --config auto --error --quiet --severity ERROR --timeout 30 \
    --exclude target --exclude dist . \
    || { echo "❌ SAST error-level findings — deploy aborted (fix, then re-run)"; exit 1; }
else
  echo "   (opengrep not installed — skipping SAST; run harness security/install-sast.sh)"
fi
if git grep -nIE -e 'sk-[A-Za-z0-9]{20,}|-----BEGIN [A-Z ]*PRIVATE KEY-----|ghp_[A-Za-z0-9]{36}|AKIA[0-9A-Z]{16}' -- . ':!:*.md' 2>/dev/null | grep -q .; then
  echo "❌ possible hardcoded secret in blog — deploy aborted"; exit 1
fi
echo "✅ gate clean."

# --- ship ---
git add -A
if git diff --cached --quiet; then echo "no new changes to commit"; else git commit -m "$msg"; fi
git push -u origin main   # always push (pushes any unpushed commits; idempotent)
echo "🚀 pushed — GitHub Actions is building Rust→WASM and deploying to Pages."
echo "   actions: https://github.com/raghunathnair1-rgb/raghunathnair1-rgb.github.io/actions"
