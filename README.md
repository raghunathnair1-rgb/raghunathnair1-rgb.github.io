# raghu // dark-factory — blog (Rust → WASM)

A personal blog written in **Rust** (Yew), compiled to **WebAssembly** with **Trunk**,
and shipped by the harness brain / dark factory. Hosted on **GitHub Pages**.

## Structure
- `src/main.rs` — the Yew app (posts live in `posts()` for now).
- `index.html` — Trunk entry. `styles.css` — the AI/terminal theme.
- `.github/workflows/deploy.yml` — builds WASM + deploys to Pages on every push to `main`.
- `deploy.sh` — commit + push (triggers the deploy).

## Adding a post (on demand)
Add a `Post { .. }` to `posts()` in `src/main.rs`, then `./deploy.sh "post: <title>"`.
(The harness brain does exactly this when you give it a "write a post about X" task.)

## Deploy pipeline
`edit → ./deploy.sh → git push → GitHub Actions (install Rust + trunk build --release)
→ upload dist/ → Pages → live at https://raghunathnair1-rgb.github.io`

## First-time GitHub setup — see DEPLOY.md (3 steps, done once).
