---
name: impeccable
description: "Use when making ANY visual/design change to the blog — new widget, CSS, layout, colour, typography, motion, or a design review/polish pass. A design-language + AI-slop check: catches interface tells (the 'could someone say an AI made this?' test) and enforces craft rules (contrast, type, spacing, motion). Applies to styles.css, index.html, and the Yew render in src/main.rs. Not for backend/logic changes."
user-invocable: true
allowed-tools:
  - Read
  - Edit
  - Bash
license: Apache-2.0
---

# impeccable (blog adaptation)

A faithful, **guidelines-only** local install of [pbakaus/impeccable](https://github.com/pbakaus/impeccable) —
"the design language that makes your AI harness better at design." The upstream project is a full Node
toolchain (browser automation, palette generators, edit hooks); **none of that is installed here** because this
is a minimal Rust/WASM site. What lives here is the part that matters for us: the **AI-slop test** and the
**universal craft + ban rules**, applied by hand.

## Setup (do this before any design change)

1. Read the current design: `styles.css`, `index.html`, and the `html! { … }` render blocks in `src/main.rs`.
   Reuse the existing tokens/patterns; don't reinvent what already works.
2. **Know the register.** This blog is deliberately **terminal-native / CRT-hacker** — monospace, scanlines,
   phosphor glow, boot sequence, ASCII. That is a *committed, intentional* voice, not a default. impeccable's
   job here is to keep it **sharp and slop-free**, never to sand it into generic "clean SaaS." Identity
   preservation wins: enhance the voice, don't neutralize it.
3. Apply the rules below. Then run the **AI-slop test** before shipping.

## The AI slop test (run before every design change)

> If someone could look at this interface and say "AI made that" without doubt, it has failed.

- **First-order:** could someone guess the theme + palette from the category alone? If a "developer blog"
  obviously implies what we shipped, it's the first training-data reflex. (We pass this — terminal/CRT is a
  committed choice, not the reflex.)
- **Second-order:** could someone guess the aesthetic from *category-plus-anti-reference* ("dev blog that's
  not corporate → terminal dark mode")? That's the trap one tier deeper. Keep the specifics *ours* (the dark
  factory, the real cluster, the ASCII fortune, the knowledge graph) so the identity is earned, not generic.

## Universal bans (never ship these — they read as AI grammar)

- **Gradient text** (`background-clip: text` on a gradient). Decorative, never meaningful. One solid colour;
  emphasis via weight/size.
- **Glassmorphism as default** — blur/glass cards used decoratively. Rare and purposeful, or none.
- **Ghost-card** — `border: 1px solid X` **and** `box-shadow: 0 … ≥16px` on the same element. Pick one.
- **Side-stripe borders** — `border-left/right` > 1px as a coloured accent. Use full borders / bg tints / a
  leading glyph instead.
- **`border-radius: 32px+`** on cards/sections/inputs. Oversized radii are a tell.
- **Eyebrow on every section** — tiny uppercase tracked kicker ("ABOUT" / "PROCESS") above each heading. One
  deliberate kicker is voice; on every section it's AI scaffolding.
- **Numbered section markers as default** (`01 · … / 02 · …`) unless the section genuinely IS an ordered
  sequence that carries information.
- **Hero-metric template** — big number / small label / gradient accent stat block. SaaS cliché.
- **Identical card grids** — same-size icon + heading + text cards repeated endlessly.
- **Decorative grid / `repeating-linear-gradient` stripe backgrounds** — texture for texture's sake.
- **Hand-drawn / sketchy SVG mascots** as filler.
- **Meta-criticism copy** ("no more boring blogs", "not your average…"). Show, don't announce.
- **Text that overflows its container** at any breakpoint — test heading copy at every width; the viewport is
  part of the design.

## Craft rules

**Colour**
- Body text ≥ **4.5:1** contrast (large/bold ≥ 3:1); placeholders too. Light-gray "for elegance" is the #1
  reason AI designs are hard to read — bump toward the ink end if it's even close.
- Gray text on a coloured background looks washed out — use a darker shade of the bg's own hue, or a
  transparency of the text colour.
- Theme (dark/light) is never a default — justify it with a one-sentence physical scene (who/where/ambient
  light). Ours: a developer reading a hacker's terminal at night → dark, phosphor. It forces the answer.

**Typography**
- Body line length **65–75ch**.
- Don't pair two similar fonts (two geometric sans). Pair on a contrast axis (serif + sans) or one family in
  multiple weights.
- Display/hero heading `clamp()` max **≤ 6rem**; letter-spacing floor **≥ -0.04em** (tighter and letters touch).

**Layout & motion**
- Semantic **z-index scale** (dropdown → sticky → modal → toast → tooltip). Never `9999`.
- No uniform section-fade reflex — one identical entrance on every section is the tell; each reveal should fit
  what it reveals. Staggering items *within* one list is fine.
- **Reveal-safety:** animations must enhance an already-visible default. Never gate content visibility on a
  class-triggered transition (it never fires on hidden tabs / headless renders → section ships blank). This
  matters here: the TTY consoles only mount the active tab.
- Never animate an `<img>` on hover (or via a parent `:hover`). Animate the card's bg/border/shadow instead.

## How to use it on this blog

- **New widget / CSS / layout:** run Setup → make the change → check it against the bans + craft rules → run
  the AI-slop test. Preserve the terminal register.
- **Polish/review pass:** read `styles.css` + the render, list any ban/craft violations, fix the real ones.
- The upstream reference files (`reference/*.md`) and Node tooling are **not** installed; if a deeper,
  register-specific pass is ever wanted, fetch them from the repo on demand.
