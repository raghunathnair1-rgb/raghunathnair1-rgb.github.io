# One-time GitHub setup (3 steps)

The VPS side is done (repo wired, deploy key generated, CI written). These 3 steps
need you on GitHub (no API token, so I can't do them for you). Do them once.

## 1. Create the repo
Create a **new empty repo** named exactly:
    raghunathnair1-rgb.github.io
(User site → served at https://raghunathnair1-rgb.github.io). Do NOT add a README.

## 2. Add the deploy key (write access)
Repo → **Settings → Deploy keys → Add deploy key**:
- Title: `harness-blog`
- Key: paste the PUBLIC key below
- ✅ **Allow write access**

```
<PASTE THE PUBLIC KEY PRINTED IN THE TERMINAL — .deploy/blog_deploy_key.pub>
```

## 3. Enable Pages via Actions
Repo → **Settings → Pages → Build and deployment → Source: GitHub Actions**.

## Then ship
From the VPS:
    cd ~/harness/blog && ./deploy.sh "initial deploy"
GitHub Actions builds Rust→WASM and publishes to Pages. Live in ~1–2 min at
https://raghunathnair1-rgb.github.io

(After this, every `./deploy.sh` — or every blog task the brain runs — auto-deploys.)
