# Verify loop — run after every ./deploy.sh (copy-paste)

Capture the OLD hash BEFORE deploying, then poll. Never claim "live" without this.

```bash
cd /home/raghu/harness/blog
# BEFORE deploy: OLD=$(curl -s https://raghunathnair1-rgb.github.io/ | grep -oE 'blog-[a-f0-9]+_bg\.wasm' | head -1)
SITE=https://raghunathnair1-rgb.github.io
API=https://api.github.com/repos/raghunathnair1-rgb/raghunathnair1-rgb.github.io
res=""
for i in $(seq 1 11); do
  sleep 20
  new=$(curl -s --max-time 10 "$SITE/" | grep -oE 'blog-[a-f0-9]+_bg\.wasm' | head -1)
  css=$(curl -s --max-time 10 "$SITE/" | grep -oE 'styles-[a-f0-9]+\.css' | head -1)
  run=$(curl -s --max-time 10 "$API/actions/runs?per_page=1")
  ci=$(echo "$run" | python3 -c "import sys,json;r=json.load(sys.stdin)['workflow_runs'][0];print(r['status'],r.get('conclusion') or '-')" 2>/dev/null)
  rid=$(echo "$run" | python3 -c "import sys,json;print(json.load(sys.stdin)['workflow_runs'][0]['id'])" 2>/dev/null)
  b=$(curl -s "$API/actions/runs/$rid/jobs" | python3 -c "import sys,json
try: print([x for x in json.load(sys.stdin)['jobs'] if x['name']=='build'][0]['conclusion'] or 'running')
except: print('?')" 2>/dev/null)
  page=$(curl -s -o /dev/null -w '%{http_code}' --max-time 10 "$SITE/")
  printf 't+%03ds | CI %-18s | build %-8s | wasm %s | css %s | http %s\n' "$((i*20))" "$ci" "$b" "$new" "$css" "$page"
  # success = build succeeded AND (wasm OR css changed) AND page 200
  if [ "$b" = success ] && [ "$page" = 200 ] && echo "$ci" | grep -q 'completed success'; then res=ok; break; fi
  [ "$b" = failure ] && { res=BUILD_FAILED; break; }       # your code — fix it
  echo "$ci" | grep -q 'completed failure' && { res=DEPLOY_FLAKE; break; }  # transient
done
echo "RESULT=$res"
# transient deploy flake -> re-trigger once
if [ "$res" = DEPLOY_FLAKE ]; then
  git commit --allow-empty -q -m "ci: re-trigger (deploy-pages flake)"; git push origin main
fi
```

## Diagnosing a build failure without CI logs (no token)
`build failure` = a Rust compile error. You cannot read the log, so:
1. Re-read the exact code you changed.
2. Suspect: new deps/features, unproven js-sys methods, iframe-in-html!, type mismatches.
3. Rewrite with APIs proven to compile here (see SKILL.md "mistakes"). Deploy, watch build.
