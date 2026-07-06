---
name: ship-blog
description: >
  Deploy the Rust/WASM blog (raghunathnair1-rgb.github.io) safely. Use this WHENEVER
  editing anything under blog/ and shipping it — before running ./deploy.sh, before
  claiming something is "live", or when a build/deploy fails. Encodes the hard-won
  rules so no session repeats past mistakes.
---

# Shipping the blog — the rules that prevent mistakes

The blog is a **Rust → WebAssembly** app (Yew + Trunk), its OWN git repo, deployed to
**GitHub Pages** via GitHub Actions. The VPS has **no C toolchain** → you CANNOT build
locally. CI is the only build. So: never claim "done" until you have *verified* the
live result. `./deploy.sh` runs a **3-layer security gate → snapshots brain status →
pushes → Actions builds+deploys**.

## The security gate (deploy.sh, before any push)
1. **Secret regex** — `git grep` for `sk-…`, private keys, `ghp_…`, `AKIA…` (aborts on hit).
2. **opengrep SAST** — `--severity ERROR` language-agnostic static analysis (aborts on error-level).
3. **Fable AI review** — `claude -p --model claude-fable-5` reads the `git diff HEAD` and
   returns one line of JSON `{verdict,severity,reason}`. Aborts the deploy on a `fail`
   (medium/high real security issue: secrets, XSS/injection, unsafe raw-HTML from untrusted
   input, exfiltration, supply-chain). **Fails OPEN** if `claude` is missing/unparseable/
   times out (infra never blocks a deploy) — but a parsed `fail` **hard-blocks**. Verified:
   catches planted secret+XSS (fail/high), passes clean code (pass/none).
   Do NOT remove or weaken this gate. If it false-blocks, tighten the prompt, don't delete it.

## Non-negotiable workflow
1. Make the change (edit `blog/src/main.rs`, `index.html`, `styles.css`, etc.).
2. Run `./deploy.sh "clear message"` from `blog/`. It gates, commits, pushes.
3. **Verify — do not trust "pushed".** Poll until you have ALL of:
   - the **build job** step = `success` (not just the run "completed"),
   - the **wasm hash changed** (Rust/asset change) OR the **CSS hash changed** (CSS-only), and
   - the site returns **HTTP 200**.
   Use the verify snippet in `verify.md`.
4. Only then say it's live. If build failed → it's YOUR code (fix). If build succeeded
   but the run failed → it's the transient `deploy-pages` flake → re-trigger.

## Mistakes already made here — do not repeat
- **js-sys Date getters**: `get_milliseconds/get_day/get_month/get_date/get_full_year`
  did NOT compile in the pinned crate. Use `js_sys::Date::now()` and
  `Date::to_date_string().as_string()` instead. Only `get_hours/get_minutes/get_seconds`
  are known-good.
- **iframes in `html!`**: Yew's `html!` chokes on iframe attrs like `allow`/`sandbox`.
  Inject raw HTML with `yew::virtual_dom::VNode::from_html_unchecked(...)`.
- **CSS-only changes**: the **wasm hash will NOT change** — verify via the **CSS hash**
  and CI success, not the wasm hash. (Track `styles-<hash>.css`.)
- **`deploy-pages` transient failure**: build succeeds, the `deploy` job fails. It is a
  GitHub flake, not your code. Fix = `git commit --allow-empty -m "ci: re-trigger" && git push`.
- **New Rust deps / web-sys features / js-sys methods** are a real build risk you cannot
  test locally. Add them, deploy, and watch the **build step** specifically.
- **Never hardcode secrets**; the gate scans and will (correctly) abort.
- **Audio can't autoplay** on first load without a user gesture — that's a browser law,
  not a bug. Use a "power on" tap or a first-interaction unlock.
- **Flag emojis** don't render on Windows (show the 2 letters) — always show the code too.

## Adding a terminal command
Command logic lives in `run_command()` (pure `&str -> String`). Side-effects that touch
the DOM (`theme`, `crt`, `reboot`) are handled specially in the `onkeydown` handler, not
in `run_command`. Add the name to the `help` string too.

## The brain-status widget
It fetches `/status.json` (written by `deploy.sh` from the real `harness-brain.service`)
and ticks live uptime. If you touch the widget, keep `deploy.sh`'s status.json writer.

See `verify.md` in this skill for the exact copy-paste verify loop.
