//! Pure, browser-independent logic for the blog.
//!
//! Everything here is std-only (no yew / web-sys / js_sys), so it compiles and runs
//! on the native host — which means it can be unit-tested and coverage-measured in CI.
//! The wasm crate (`blog`) depends on this crate and calls these functions directly, so
//! the tested code IS the shipped code. A CI job runs `cargo llvm-cov -p blog-logic
//! --fail-under-lines 100`, so a change that drops this crate below 100% coverage blocks
//! the deploy. Keep only genuinely pure, fully-reachable logic here.

/// Knowledge-graph node index -> cluster/domain id (0..=5).
pub fn kg_domain(i: usize) -> usize {
    match i {
        0 | 4 => 0,                     // dark-factory, automation
        // brain & AI: llms, brain, dgx-spark, vllm, router, self-improve + 2026 agentic context
        3 | 5 | 16 | 17 | 20 | 22 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 35 => 1,
        // pipeline: security, trunk, gh-pages, opengrep, pipeline + evals/guardrails
        8 | 13 | 14 | 15 | 21 | 33 | 34 => 2,
        1 | 2 | 12 | 18 => 3,           // rust, wasm, yew, matrix
        19 | 23 | 24 => 4,              // ai-feed, seo, linkedin
        6 | 7 | 9 | 10 | 11 => 5,       // coffee, maine-coon, posts
        _ => 0,
    }
}

/// Domain id -> CSS class for its node color.
pub fn kg_dom_cls(d: usize) -> &'static str {
    match d {
        1 => "kg-d1",
        2 => "kg-d2",
        3 => "kg-d3",
        4 => "kg-d4",
        5 => "kg-d5",
        _ => "kg-d0",
    }
}

/// Base node radius by kind.
pub fn kg_r(kind: u8) -> f64 {
    match kind {
        0 => 8.0,
        3 => 4.5,
        4 => 7.0,
        _ => 6.0,
    }
}

/// Format an SVG coordinate to one decimal place.
pub fn kg_fmt(v: f64) -> String {
    format!("{:.1}", v)
}

/// Pipeline event kind -> CSS class.
pub fn evt_cls(kind: &str) -> &'static str {
    match kind {
        "router" => "pipe-k pipe-k-router",
        "autopost" => "pipe-k pipe-k-post",
        "self-improve" => "pipe-k pipe-k-improve",
        "deploy" => "pipe-k pipe-k-deploy",
        _ => "pipe-k",
    }
}

/// Minutes of daylight (sunset - sunrise, both minutes-since-midnight) as "16h 31m".
/// Proposed by the Idea Engine (2026-07-09); a fully-testable extension of the astronomy set.
pub fn day_length_hm(sunrise_min: i32, sunset_min: i32) -> String {
    let d = (sunset_min - sunrise_min).max(0);
    format!("{}h {:02}m", d / 60, d % 60)
}

/// Estimated reading time in whole minutes for `body`: word count over ~200 wpm,
/// rounded to the nearest minute with a floor of 1. Pure and total, mirroring
/// `day_length_hm`. The wasm post header calls this directly, so the tested value
/// IS the value shipped. Keep it covered by the 100% gate.
pub fn reading_time(body: &str) -> u32 {
    (((body.split_whitespace().count() as u32) + 100) / 200).max(1)
}

/// Rank posts by keyword overlap with the post at `idx`, most-related first.
/// Each `keywords` entry is that post's searchable text (e.g. `"tag title"`);
/// relatedness is the number of distinct lowercased word tokens two posts share.
/// Returns up to `limit` other-post indices with non-zero overlap, ties broken by
/// original order. Pure, total, and std-only — the wasm post view calls it directly,
/// so keep it covered by the 100% gate.
pub fn related_posts(keywords: &[&str], idx: usize, limit: usize) -> Vec<usize> {
    if idx >= keywords.len() {
        return Vec::new();
    }
    let toks = |s: &str| -> std::collections::BTreeSet<String> {
        s.split(|c: char| !c.is_alphanumeric())
            .filter(|w| !w.is_empty())
            .map(|w| w.to_lowercase())
            .collect()
    };
    let target = toks(keywords[idx]);
    let mut scored: Vec<(usize, usize)> = keywords
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != idx)
        .map(|(i, s)| (i, toks(s).intersection(&target).count()))
        .filter(|&(_, score)| score > 0)
        .collect();
    // strongest overlap first; equal overlap keeps original post order
    scored.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    scored.into_iter().take(limit).map(|(i, _)| i).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_covers_every_cluster() {
        assert_eq!(kg_domain(0), 0); // dark-factory
        assert_eq!(kg_domain(4), 0); // automation (same arm)
        assert_eq!(kg_domain(3), 1);
        assert_eq!(kg_domain(8), 2);
        assert_eq!(kg_domain(1), 3);
        assert_eq!(kg_domain(19), 4);
        assert_eq!(kg_domain(6), 5);
        assert_eq!(kg_domain(999), 0); // default arm — unlisted index
    }

    #[test]
    fn dom_cls_covers_every_color() {
        assert_eq!(kg_dom_cls(1), "kg-d1");
        assert_eq!(kg_dom_cls(2), "kg-d2");
        assert_eq!(kg_dom_cls(3), "kg-d3");
        assert_eq!(kg_dom_cls(4), "kg-d4");
        assert_eq!(kg_dom_cls(5), "kg-d5");
        assert_eq!(kg_dom_cls(0), "kg-d0"); // default arm
        assert_eq!(kg_dom_cls(9), "kg-d0"); // out of range -> default
    }

    #[test]
    fn radius_by_kind() {
        assert_eq!(kg_r(0), 8.0);
        assert_eq!(kg_r(3), 4.5);
        assert_eq!(kg_r(4), 7.0);
        assert_eq!(kg_r(1), 6.0); // default arm
    }

    #[test]
    fn fmt_one_decimal() {
        // values kept clear of the .?5 rounding boundary so the assertion can't flake
        assert_eq!(kg_fmt(12.34), "12.3");
        assert_eq!(kg_fmt(0.0), "0.0");
        assert_eq!(kg_fmt(7.98), "8.0");
        assert_eq!(kg_fmt(-3.14), "-3.1");
    }

    #[test]
    fn event_class() {
        assert_eq!(evt_cls("router"), "pipe-k pipe-k-router");
        assert_eq!(evt_cls("autopost"), "pipe-k pipe-k-post");
        assert_eq!(evt_cls("self-improve"), "pipe-k pipe-k-improve");
        assert_eq!(evt_cls("deploy"), "pipe-k pipe-k-deploy");
        assert_eq!(evt_cls("anything-else"), "pipe-k"); // default arm
    }

    #[test]
    fn daylight_length() {
        assert_eq!(day_length_hm(330, 1321), "16h 31m"); // 05:30 -> 22:01
        assert_eq!(day_length_hm(360, 1080), "12h 00m"); // 06:00 -> 18:00
        assert_eq!(day_length_hm(1000, 500), "0h 00m"); // clamped: sunset before sunrise
    }

    #[test]
    fn reading_time_rounds_and_floors() {
        assert_eq!(reading_time(""), 1); // empty -> floor of 1
        assert_eq!(reading_time("just a few words"), 1); // 4 words rounds down to 1
        assert_eq!(reading_time(&"word ".repeat(300)), 2); // (300+100)/200 = 2
        assert_eq!(reading_time(&"word ".repeat(500)), 3); // (500+100)/200 = 3
    }

    #[test]
    fn related_ranks_by_overlap() {
        // comma+space exercises the non-alphanumeric split (and its empty-token filter)
        let ks = ["rust, wasm", "rust systems", "coffee cat", "wasm rust yew"];
        // post 0 {rust,wasm}: shares 2 with post 3, 1 with post 1, 0 with post 2
        assert_eq!(related_posts(&ks, 0, 5), vec![3, 1]); // strongest overlap first
        assert_eq!(related_posts(&ks, 0, 1), vec![3]); // limit is respected
        // post 1 {rust,systems}: ties with 0 and 3 (both share only "rust")
        assert_eq!(related_posts(&ks, 1, 5), vec![0, 3]); // equal overlap -> original order
        assert!(related_posts(&ks, 2, 5).is_empty()); // "coffee cat" shares nothing
        assert!(related_posts(&ks, 9, 5).is_empty()); // idx past the end -> empty
    }
}
