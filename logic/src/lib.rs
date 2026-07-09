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
        3 | 5 | 16 | 17 | 22 | 24 => 1, // llms, brain, dgx-spark, vllm, router, self-improve
        8 | 13 | 14 | 15 | 23 => 2,     // security, trunk, gh-pages, opengrep, pipeline
        1 | 2 | 12 | 18 | 19 | 20 => 3, // rust, wasm, yew, matrix, terminal, orrery
        21 | 25 | 26 => 4,              // ai-feed, seo, linkedin
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

/// Node kind -> legacy kind CSS class.
pub fn kg_cls(kind: u8) -> &'static str {
    match kind {
        0 => "kg-root",
        2 => "kg-post",
        3 => "kg-tool",
        4 => "kg-feed",
        _ => "kg-concept",
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

/// Fraction (0.0..1.0) through the lunar synodic cycle for a given epoch-ms instant.
pub fn moon_phase_frac(now_ms: f64) -> f64 {
    let synodic = 29.530_588_853 * 86_400_000.0;
    let diff = now_ms - 947_182_440_000.0; // 2000-01-06 18:14 UTC — a known new moon
    let mut p = (diff / synodic).fract();
    if p < 0.0 {
        p += 1.0;
    }
    p
}

/// Human name for a phase fraction.
pub fn moon_name(p: f64) -> &'static str {
    match ((p * 8.0).round() as i64).rem_euclid(8) {
        0 => "New Moon",
        1 => "Waxing Crescent",
        2 => "First Quarter",
        3 => "Waxing Gibbous",
        4 => "Full Moon",
        5 => "Waning Gibbous",
        6 => "Last Quarter",
        _ => "Waning Crescent",
    }
}

/// Percent illuminated (0..=100) for a phase fraction.
pub fn moon_illum(p: f64) -> u32 {
    ((1.0 - (2.0 * std::f64::consts::PI * p).cos()) / 2.0 * 100.0).round() as u32
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
        assert_eq!(kg_domain(21), 4);
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
    fn kind_class() {
        assert_eq!(kg_cls(0), "kg-root");
        assert_eq!(kg_cls(2), "kg-post");
        assert_eq!(kg_cls(3), "kg-tool");
        assert_eq!(kg_cls(4), "kg-feed");
        assert_eq!(kg_cls(1), "kg-concept"); // default arm
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
    fn phase_frac_in_unit_interval_and_both_branches() {
        // reference instant -> exactly a new moon (diff = 0, non-negative branch)
        assert!((moon_phase_frac(947_182_440_000.0)).abs() < 1e-9);
        // an instant BEFORE the reference -> negative diff -> the `p += 1.0` branch
        let before = moon_phase_frac(0.0);
        assert!((0.0..1.0).contains(&before));
        // a normal later instant stays in range
        let later = moon_phase_frac(1_800_000_000_000.0);
        assert!((0.0..1.0).contains(&later));
    }

    #[test]
    fn moon_name_covers_all_eight_phases() {
        assert_eq!(moon_name(0.0 / 8.0), "New Moon");
        assert_eq!(moon_name(1.0 / 8.0), "Waxing Crescent");
        assert_eq!(moon_name(2.0 / 8.0), "First Quarter");
        assert_eq!(moon_name(3.0 / 8.0), "Waxing Gibbous");
        assert_eq!(moon_name(4.0 / 8.0), "Full Moon");
        assert_eq!(moon_name(5.0 / 8.0), "Waning Gibbous");
        assert_eq!(moon_name(6.0 / 8.0), "Last Quarter");
        assert_eq!(moon_name(7.0 / 8.0), "Waning Crescent"); // the `_` arm
    }

    #[test]
    fn illumination_endpoints() {
        assert_eq!(moon_illum(0.0), 0); // new moon -> dark
        assert_eq!(moon_illum(0.5), 100); // full moon -> lit
    }
}
