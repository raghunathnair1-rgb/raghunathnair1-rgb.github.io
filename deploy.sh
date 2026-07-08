#!/usr/bin/env bash
# Ship the blog through the dark-factory gate, then push -> GitHub Actions builds
# the WASM and deploys to Pages. Uses the scoped blog deploy key (core.sshCommand).
set -euo pipefail
cd "$(dirname "$0")"
msg="${1:-blog: update $(date -u +%FT%TZ)}"
SEC=/home/raghu/harness/security
# cron has no login session -> give systemctl --user a runtime dir so the brain snapshot works
# (and can't kill set -e). This is why the cron silently stopped deploying before.
export XDG_RUNTIME_DIR="${XDG_RUNTIME_DIR:-/run/user/$(id -u)}"

# --- snapshot the real harness-brain status for the blog's brain widget ---
bstate=$(systemctl --user is-active harness-brain 2>/dev/null || echo unknown)
bstart=$(systemctl --user show harness-brain -p ActiveEnterTimestamp --value 2>/dev/null || echo "")
bepoch=$(date -d "$bstart" +%s 2>/dev/null || echo 0)
bpid=$(systemctl --user show harness-brain -p MainPID --value 2>/dev/null || echo 0)
printf '{"healthy": %s, "service": "harness-brain.service", "started_epoch": %s, "pid": %s, "updated_epoch": %s}\n' \
  "$([ "$bstate" = active ] && echo true || echo false)" "${bepoch:-0}" "${bpid:-0}" "$(date +%s)" > status.json
echo "🧠 brain snapshot: $bstate (pid ${bpid:-0})"

# --- snapshot the DGX Spark GPU telemetry (fail-open; never blocks a deploy) ---
timeout 25 python3 /home/raghu/harness/spark_stats.py 2>/dev/null || echo "⚡ spark snapshot skipped (unreachable)"
# --- aggregate the router's brain.log (on-device vs cloud split) for the cost-meter widget ---
python3 /home/raghu/harness/router_stats.py >/dev/null 2>&1 || true
# --- regenerate crawlable SEO pages (post pages, feed, sitemap, robots) from posts.json + news.json ---
python3 /home/raghu/harness/ssg.py >/dev/null 2>&1 || echo "📄 ssg skipped"
# --- snapshot latest change + recent deploys for the pipeline console ---
python3 /home/raghu/harness/gen_deploy.py "$msg" >/dev/null 2>&1 || echo "📦 deploy.json skipped"
# --- sanitized factory execution-activity feed (router/autopost/self-improve/deploys) ---
python3 /home/raghu/harness/gen_activity.py >/dev/null 2>&1 || echo "📡 activity.json skipped"

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

# --- AI security review (Fable 5) of the diff — catches what regex/SAST miss ---
if command -v claude >/dev/null 2>&1; then
  echo "🔍 fable security review…"
  # exclude telemetry data files: a telemetry-only refresh has an empty diff here -> AI review skipped (no model cost)
  DIFF=$(git --no-pager diff HEAD -- . ':(exclude)status.json' ':(exclude)spark.json' ':(exclude)router.json' ':(exclude)deploy.json' ':(exclude)activity.json' 2>/dev/null | head -c 10000 || true)
  if [ -n "$DIFF" ]; then
    RES=$(timeout 90 claude -p "You are a STRICT application-security reviewer for a PUBLIC Rust/WASM blog on GitHub Pages. Review ONLY the git diff below for REAL, exploitable problems: hardcoded secrets/tokens/keys, XSS or HTML/JS injection, unsafe raw-HTML built from untrusted input, dangerous eval/fetch, data exfiltration, or supply-chain risk. Ignore style, naming and non-security nits. Reply with ONE line of JSON and nothing else: {\"verdict\":\"pass\"|\"fail\",\"severity\":\"none|low|medium|high\",\"reason\":\"short\"}. Set verdict=fail ONLY for a medium or high severity real security problem.

GIT DIFF:
$DIFF" --model claude-fable-5 --output-format json 2>/dev/null \
      | python3 -c "import sys,json,re
try:
    r=json.load(sys.stdin).get('result','')
    d=json.loads(re.search(r'\{.*\}', r, re.S).group(0))
    print(d.get('verdict','pass'), d.get('severity','none'), '::', str(d.get('reason',''))[:180])
except Exception: print('pass none :: (review unavailable — failing open)')" 2>/dev/null) || true
    RES=${RES:-"pass none :: (no result — failing open)"}
    if [ "$(printf '%s' "$RES" | awk '{print $1}')" = "fail" ]; then
      echo "❌ fable security review FAILED — $RES"
      echo "   deploy aborted. review the finding, fix, and re-run."
      exit 1
    fi
    echo "   ✅ fable review: $RES"
  fi
else
  echo "   (claude CLI not found — skipping AI security review)"
fi
echo "✅ gate clean."

# --- ship ---
git add -A
if git diff --cached --quiet; then echo "no new changes to commit"; else git commit -m "$msg"; fi
git push -u origin main   # always push (pushes any unpushed commits; idempotent)
echo "🚀 pushed — GitHub Actions is building Rust→WASM and deploying to Pages."
echo "   actions: https://github.com/raghunathnair1-rgb/raghunathnair1-rgb.github.io/actions"
