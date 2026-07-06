---
name: add-wasm-widget
description: >
  Add a widget or feature to the Rust/WASM blog (a card, live-data fetch, animation,
  terminal command, or timer). Use when building any new blog UI in Yew. Pairs with
  ship-blog for the deploy+verify workflow.
---

# Adding a Yew/WASM widget without breaking the build

The blog compiles only in CI (no local toolchain), so favor patterns already PROVEN to
compile in this repo. When in doubt, use the exact idioms below.

## Proven idioms (copy these)
- **Component**: `#[function_component(Name)] fn name() -> Html { ... }`, used as `<Name />`.
- **State**: `let s = use_state(|| initial);` → read `&*s`, write `s.set(v)`. Clone the
  handle before moving into closures. NEVER read `*s` inside an interval/callback captured
  at mount (stale) — only `.set(fresh_value)`.
- **Timer**: `gloo_timers::callback::Interval::new(ms, move || s.set(fresh()))`, and return
  `move || drop(interval)` from `use_effect_with((), ...)` to clean up.
- **Fetch (async)**: inside `use_effect_with((), move |_| { wasm_bindgen_futures::spawn_local(async move {
  if let Ok(r) = gloo_net::http::Request::get(url).send().await { if let Ok(v) = r.json::<T>().await { s.set(Some(v)); } } }); || () })`.
  Parse with a `#[derive(serde::Deserialize)]` struct; extra JSON fields are ignored.
- **Time**: `js_sys::Date::now()` (ms, f64) and `Date::to_date_string().as_string()`. Do
  NOT use `get_milliseconds/get_day/get_month/get_date/get_full_year` (don't compile here).
- **Raw HTML** (iframes, anything `html!` rejects): `yew::virtual_dom::VNode::from_html_unchecked(yew::AttrValue::from(r#"...")).`
- **Numbers in html!**: `{ x.to_string() }` (don't rely on ToHtml for ints).
- **DOM side-effect** (set attr / storage / reload): `gloo_utils::document().document_element()`,
  `web_sys::window()` — enable the web-sys feature in Cargo.toml FIRST.

## Deps / features cost a build (verify each)
Adding a crate, a `web-sys` feature, or a new js-sys method = a build risk you can't test
locally. Add it, deploy, and watch the **build step** specifically (see ship-blog/verify.md).

## Data that must be live but the site is static
GitHub Pages is static. For "live" data either (a) fetch a public CORS-enabled API
client-side (e.g. wttr.in `?format=j1`, github api), or (b) have `deploy.sh` write a
`status.json`/data file the widget fetches same-origin.

## Terminal commands
Add to `run_command()` (pure). DOM effects (theme/crt/reboot) go in `onkeydown`. Update
the `help` string. Test the arm ordering — specific slice patterns before catch-all.

## Then: follow ship-blog to deploy + verify. Do not skip verification.
