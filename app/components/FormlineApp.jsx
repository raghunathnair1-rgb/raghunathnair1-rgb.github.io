"use client";

import { Component } from "react";
import AppShell from "./AppShell";

export default class FormlineApp extends Component {
  render() {
    return <AppShell model={this.renderVals()} />;
  }
  state = { view: "session", running: false, t: 0, reps: [], cues: [] };

  MOVES = [
    {
      name: "Pull-up",
      tier: "INTERMEDIATE",
      form: 91,
      rom: "96%",
      tempo: "2.1s",
      symm: "94%",
      grade: "A-",
      mastery: "82%",
    },
    {
      name: "Push-up",
      tier: "FOUNDATION",
      form: 95,
      rom: "98%",
      tempo: "1.9s",
      symm: "97%",
      grade: "A",
      mastery: "94%",
    },
    {
      name: "Dip",
      tier: "FOUNDATION",
      form: 88,
      rom: "91%",
      tempo: "2.4s",
      symm: "90%",
      grade: "B+",
      mastery: "76%",
    },
    {
      name: "Pistol squat",
      tier: "INTERMEDIATE",
      form: 74,
      rom: "82%",
      tempo: "3.0s",
      symm: "71%",
      grade: "C",
      mastery: "48%",
    },
    {
      name: "L-sit",
      tier: "INTERMEDIATE",
      form: 83,
      rom: "88%",
      tempo: "—",
      symm: "86%",
      grade: "B",
      mastery: "61%",
    },
    {
      name: "Handstand",
      tier: "ADVANCED",
      form: 69,
      rom: "74%",
      tempo: "—",
      symm: "64%",
      grade: "C-",
      mastery: "35%",
    },
    {
      name: "Muscle-up",
      tier: "ADVANCED",
      form: 77,
      rom: "85%",
      tempo: "1.6s",
      symm: "79%",
      grade: "B-",
      mastery: "44%",
    },
    {
      name: "Front lever",
      tier: "ELITE",
      form: 58,
      rom: "63%",
      tempo: "—",
      symm: "57%",
      grade: "D+",
      mastery: "21%",
    },
  ];

  componentDidMount() {
    this.iv = setInterval(() => {
      if (!this.state.running) return;
      this.setState((s) => {
        const t = s.t + 1;
        let reps = s.reps,
          cues = s.cues;
        if (t % 26 === 0) {
          const score = Math.round(76 + 20 * Math.abs(Math.sin(t / 31 + 1.2)));
          reps = [...s.reps, { n: reps.length + 1, score }];
          cues = [this.cueFor(score, reps.length), ...s.cues].slice(0, 5);
        }
        return { t, reps, cues };
      });
    }, 95);
  }
  componentWillUnmount() {
    clearInterval(this.iv);
  }

  get accent() {
    return this.props.accentColor || "#5fe08f";
  }
  get warn() {
    return "#e8b13a";
  }
  get bad() {
    return "#ef6a5a";
  }
  get strict() {
    return this.props.strictMode ?? false;
  }
  get bar() {
    return this.strict ? 90 : 84;
  }

  tone(v) {
    return v >= this.bar
      ? this.accent
      : v >= this.bar - 10
        ? this.warn
        : this.bad;
  }

  cueFor(score, n) {
    const lib = [
      {
        head: "Elbow flare on the pull",
        body: "Right elbow drifted 11° wide at mid-range. Drive the shoulder blade down first.",
      },
      {
        head: "Clean rep",
        body: "Full range, even bar path, scapula engaged before the arms.",
      },
      {
        head: "Tempo collapsed",
        body: "Eccentric dropped to 0.7s. Target a 2s lower to keep tension.",
      },
      {
        head: "Hip sway detected",
        body: "Pelvis rotated 6° left. Squeeze glutes to lock the line.",
      },
      {
        head: "Range cut short",
        body: "Chin stopped 4cm below the bar. Finish the rep at the top.",
      },
    ];
    const pick = score >= this.bar ? lib[1] : lib[(n + score) % lib.length];
    return {
      head: "Rep " + n + " · " + pick.head,
      body: pick.body,
      color: this.tone(score),
    };
  }

  renderVals() {
    const { view, running, t, reps, cues } = this.state;
    const A = this.accent,
      W = this.warn,
      B = this.bad;
    const target = this.props.repTarget ?? 12;
    const go = (v) => () => this.setState({ view: v });
    const phase = (t % 26) / 26;
    const live = reps.length ? reps[reps.length - 1].score : 88;
    const acc = running
      ? Math.round(live * 0.7 + (82 + 12 * Math.sin(t / 17)) * 0.3)
      : reps.length
        ? Math.round(reps.reduce((a, r) => a + r.score, 0) / reps.length)
        : 88;

    const navDefs = [
      ["session", "Live session", running ? "REC" : "IDLE"],
      ["dashboard", "Dashboard", "12W"],
      ["library", "Move library", "8"],
      ["report", "Session report", reps.length ? String(reps.length) : "—"],
    ];

    const jointDefs = [
      ["Shoulder L", 3, 14, 26],
      ["Shoulder R", 11, 30, 26],
      ["Elbow L", 5, 12, 44],
      ["Elbow R", 9, 32, 44],
      ["Spine", 4, 22, 38],
      ["Hip line", 7, 22, 58],
      ["Knee L", 2, 17, 80],
      ["Knee R", 3, 27, 80],
      ["Scapula", 6, 22, 22],
    ];
    const bob = running ? Math.sin(phase * Math.PI * 2) * 5 : 0;

    const cueList = cues.length
      ? cues
      : [
          {
            head: "Awaiting first rep",
            body: "Step into frame and begin. Scoring starts on the first detected concentric.",
            color: "#767d74",
          },
        ];

    return {
      accentColor: A,
      athlete: this.props.athleteName || "Mara Ellis",
      initials: (this.props.athleteName || "Mara Ellis")
        .split(" ")
        .map((w) => w[0])
        .join("")
        .slice(0, 2),
      tierLabel: this.strict ? "STRICT · COACH MODE" : "INTERMEDIATE · WK 14",

      isSession: view === "session",
      isDash: view === "dashboard",
      isLib: view === "library",
      isReport: view === "report",
      nav: navDefs.map(([v, label, tag]) => ({
        label,
        tag,
        go: go(v),
        fg: view === v ? "#e9ece8" : "#8d958b",
        bg: view === v ? "#151815" : "transparent",
        tagFg: v === "session" && running ? B : "#5e655d",
      })),

      crumb: {
        session: "DEMO SESSION / SIMULATED MOVEMENT",
        dashboard: "PROGRESS / 12 WEEK WINDOW",
        library: "CATALOGUE / 8 TRACKED MOVES",
        report: "REPORT / SESSION 041",
      }[view],
      title: {
        session: "Movement accuracy, live",
        dashboard: "Where your form is going",
        library: "Every move, graded",
        report: "Session 041 breakdown",
      }[view],

      statusDot: running ? B : "#5e655d",
      statusText: running ? "DEMO RUNNING · 9 SIMULATED JOINTS" : "DEMO READY",
      ctaLabel: running
        ? "End session"
        : reps.length
          ? "Resume capture"
          : "Start capture",
      ctaBg: running ? "transparent" : A,
      ctaFg: running ? "#e9ece8" : "#0a0b0a",
      ctaBorder: running ? "#334034" : A,
      toggleRun: () =>
        this.setState((s) => ({
          running: !s.running,
          view: s.running ? "report" : "session",
        })),

      moveName: "Pull-up · strict",
      phaseLabel: running
        ? phase < 0.45
          ? "CONCENTRIC — " + Math.round(phase * 222) + "% ROM"
          : phase < 0.55
            ? "TOP HOLD"
            : "ECCENTRIC — 2.0s TARGET"
        : "STANDBY",
      phaseColor: running ? A : "#5e655d",
      focusX: 22 + bob * 0.3 + "%",
      focusY: 30 + bob + "%",
      focusColor: this.tone(acc),

      joints: jointDefs.map(([name, dev, x, y]) => ({
        x: x + bob * 0.25 + "%",
        y: y + bob * (y > 50 ? 0.3 : 1) + "%",
        size: dev > 8 ? "11px" : "9px",
        color: dev > 8 ? B : dev > 5 ? W : A,
        halo: (dev > 8 ? B : dev > 5 ? W : A) + "26",
      })),
      jointRows: jointDefs.map(([name, dev]) => {
        const d = running
          ? Math.max(1, dev + Math.round(2 * Math.sin(t / 9 + dev)))
          : dev;
        return {
          name,
          dev: d + "°",
          pct: Math.min(100, d * 9) + "%",
          color: d > 8 ? B : d > 5 ? W : A,
        };
      }),

      liveChips: [
        { label: "REPS", value: reps.length + "/" + target, color: "#e9ece8" },
        {
          label: "TEMPO",
          value: running
            ? (1.7 + 0.5 * Math.abs(Math.sin(t / 23))).toFixed(1) + "s"
            : "—",
          color: "#e9ece8",
        },
        {
          label: "SYMMETRY",
          value:
            (running ? 88 + Math.round(8 * Math.abs(Math.cos(t / 19))) : 92) +
            "%",
          color: this.tone(acc),
        },
      ],

      accuracy: acc,
      accDeg: Math.round(acc * 3.6) + "deg",
      verdict:
        acc >= this.bar
          ? "Form holding inside tolerance"
          : acc >= this.bar - 10
            ? "Drifting — correctable"
            : "Breakdown detected",
      verdictColor: this.tone(acc),
      verdictNote:
        acc >= this.bar
          ? "Bar path deviation under 2cm across the last three reps. Keep this tempo."
          : "Right elbow and hip line are the two contributors. Cut the set at the first missed rep.",
      miniStats: [
        {
          label: "RANGE OF MOTION",
          value:
            (running ? 88 + Math.round(9 * Math.abs(Math.sin(t / 13))) : 96) +
            "%",
        },
        {
          label: "TIME UNDER TENSION",
          value: Math.round(reps.length * 4.1) + "s",
        },
      ],

      repProgress: reps.length + " OF " + target + " REPS · BAR " + this.bar,
      repBars: (reps.length
        ? reps
        : Array.from({ length: 12 }, (_, i) => ({ n: i + 1, score: 0 }))
      )
        .slice(-14)
        .map((r) => ({
          n: r.n,
          h: r.score ? Math.max(6, r.score * 0.9) + "%" : "4%",
          color: r.score ? this.tone(r.score) : "#161a16",
        })),
      cues: cueList,

      tiles: [
        {
          label: "MEAN FORM SCORE",
          value: "89.4",
          delta: "+2.1",
          deltaColor: A,
          note: "Across 41 tracked sessions",
        },
        {
          label: "CLEAN REP RATE",
          value: "78%",
          delta: "+11",
          deltaColor: A,
          note: "Reps inside full tolerance",
        },
        {
          label: "SYMMETRY GAP",
          value: "6.2%",
          delta: "−1.4",
          deltaColor: A,
          note: "Left side still dominant",
        },
        {
          label: "STREAK",
          value: "23",
          delta: "days",
          deltaColor: "#767d74",
          note: "Longest run to date",
        },
      ],
      trend: [78, 80, 79, 83, 82, 86, 84, 88, 87, 90, 89, 92].map((v, i) => ({
        label: "W" + (i + 1),
        h: (v - 60) * 2.4 + "%",
        color: i === 11 ? A : v >= 86 ? A + "cc" : "#2a332b",
      })),
      weakPoints: [
        {
          score: "71",
          title: "Right elbow flare under load",
          color: W,
          body: "Appears on rep 6 onward in pull-ups and muscle-ups. Scapular retraction is late by ~0.3s.",
        },
        {
          score: "64",
          title: "Pistol squat knee tracking",
          color: B,
          body: "Knee collapses 9° medial at the bottom. Add tempo eccentrics and ankle mobility work.",
        },
        {
          score: "83",
          title: "Handstand line drift",
          color: W,
          body: "Ribs flare as hold passes 12s. Shorten holds, prioritise hollow position.",
        },
      ],
      moveRows: this.MOVES.map((m) => ({
        ...m,
        go: () => this.setState({ view: "session" }),
        form: m.form + "%",
        formColor: this.tone(m.form),
        gradeColor: m.form >= 88 ? A : m.form >= 74 ? W : B,
      })),
      library: this.MOVES.map((m) => ({
        name: m.name,
        tier: m.tier,
        grade: m.grade,
        mastery: m.mastery,
        slot: m.name.toUpperCase() + " REFERENCE CLIP",
        border: "#1c201c",
        gradeColor: m.form >= 88 ? A : m.form >= 74 ? W : B,
        go: () => this.setState({ view: "session" }),
      })),

      sessionScore: reps.length
        ? Math.round(reps.reduce((a, r) => a + r.score, 0) / reps.length)
        : 88,
      reportSummary: reps.length
        ? reps.length +
          " reps captured. " +
          reps.filter((r) => r.score >= this.bar).length +
          " cleared the " +
          this.bar +
          "-point bar. Form held for the first two thirds of the set, then elbow flare returned under fatigue."
        : "No reps captured yet — start a capture session to generate a breakdown.",
      reportStats: [
        {
          label: "CLEAN REPS",
          value:
            reps.filter((r) => r.score >= this.bar).length +
            "/" +
            (reps.length || 0),
          note: "Inside full tolerance band",
        },
        {
          label: "BEST REP",
          value: reps.length ? Math.max(...reps.map((r) => r.score)) : "—",
          note: "Rep 3 · bar path deviation 0.8cm",
        },
        {
          label: "FATIGUE ONSET",
          value: reps.length > 5 ? "Rep 6" : "—",
          note: "First sustained score drop",
        },
      ],
      fixList: [
        {
          n: "01",
          title: "Lead with the scapula",
          body: "Three sets of 8 scap pulls before every pulling session. Target retraction 0.3s before elbow flexion.",
        },
        {
          n: "02",
          title: "Cap the set at the first flare",
          body: "Quality over volume: end the set the rep after deviation crosses 8°.",
        },
        {
          n: "03",
          title: "Tempo eccentrics",
          body: "Two 4-second lowers per set to rebuild control through the bottom third of the range.",
        },
      ],
    };
  }
}
