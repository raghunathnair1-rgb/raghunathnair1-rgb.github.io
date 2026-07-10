use yew::prelude::*;
use yew::TargetCast;
// pure logic lives in the coverage-gated blog-logic crate (tested code == shipped code)
use blog_logic::{day_length_hm, evt_cls, kg_dom_cls, kg_domain, kg_fmt, kg_r, moon_illum, moon_name, moon_phase_frac};

/// Fetch (and optionally poll) a JSON endpoint into (data, err) state. Always cache-busts —
/// GitHub Pages caches these files up to 10min — so no widget ever serves a stale snapshot.
/// Returns the same (Option<T>, bool) shape the widgets already render, so call sites drop in
/// without touching their view code.
#[hook]
fn use_polled_json<T>(url: &'static str, every_ms: Option<u32>) -> (UseStateHandle<Option<T>>, UseStateHandle<bool>)
where
    T: serde::de::DeserializeOwned + 'static,
{
    let data = use_state(|| None::<T>);
    let err = use_state(|| false);
    {
        let data = data.clone();
        let err = err.clone();
        use_effect_with((), move |_| {
            let go = move |data: UseStateHandle<Option<T>>, err: UseStateHandle<bool>| {
                let sep = if url.contains('?') { "&" } else { "?" };
                let u = format!("{}{}t={}", url, sep, js_sys::Date::now() as u64);
                wasm_bindgen_futures::spawn_local(async move {
                    match gloo_net::http::Request::get(&u).send().await {
                        Ok(resp) => match resp.json::<T>().await {
                            Ok(v) => {
                                data.set(Some(v));
                                err.set(false);
                            }
                            Err(_) => err.set(true),
                        },
                        Err(_) => err.set(true),
                    }
                });
            };
            go(data.clone(), err.clone());
            let interval = every_ms.map(|ms| {
                let go = go.clone();
                let data = data.clone();
                let err = err.clone();
                gloo_timers::callback::Interval::new(ms, move || go(data.clone(), err.clone()))
            });
            move || drop(interval)
        });
    }
    (data, err)
}

#[derive(Clone)]
enum Line {
    Cmd(String),
    Out(String),
}

const MAINE_COON: &str = "      |\\      _,,,---,,_\n     /,`.-'`'    -.  ;-;;,_\n    |,4-  ) )-,_..;\\ (  `'-'\n   '---''(_/--'  `-'\\_)";

fn run_command(cmd: &str) -> String {
    let p: Vec<&str> = cmd.split_whitespace().collect();
    match p.as_slice() {
        ["help"] => "commands: help  cd <console>  whoami  ls  cat <post>  meow  neofetch  dmesg  moon  doomfire  warp  spark  now-playing  coffee  brew  fortune  theme <name>  crt  nodes  path <a> <b>  reboot  uptime  echo <x>  clear".to_string(),
        ["cd", rest @ ..] => {
            let name = rest.join(" ");
            let name = name.trim_start_matches("~/").trim_matches('/');
            match name {
                "" | "~" | "home" => "\u{2192} ~/ (home)".to_string(),
                "posts" | "factory" | "feed" | "pipeline" | "contact" => format!("\u{2192} ~/{}", name),
                other => format!("cd: {}: no such console (try: ~ posts factory feed pipeline contact)", other),
            }
        }
        ["reboot"] => "rebooting the dark factory\u{2026}".to_string(),
        ["crt"] | ["crt", "on"] | ["crt", "off"] => "CRT mode \u{1F4FA} toggled".to_string(),
        ["whoami"] => "raghu \u{2014} builder \u{00B7} tinkerer \u{00B7} runs an AI dark factory for fun".to_string(),
        ["ls"] => "about.md   now-playing   neofetch   posts/   linkedin   github".to_string(),
        ["ls", "posts"] | ["ls", "posts/"] => {
            // derive from the real posts so `ls posts` always matches what `cat <post>` can open
            let mut names: Vec<String> = Vec::new();
            for p in posts() {
                let n = format!("{}.md", p.tag);
                if !names.contains(&n) {
                    names.push(n);
                }
            }
            names.join("   ")
        }
        ["neofetch"] => "os: dark-factory \u{00B7} kernel: rust\u{2192}wasm \u{00B7} shell: harness brain \u{00B7} status: \u{25CF} online".to_string(),
        ["dmesg"] => "[dark-factory] dream log streams above \u{2014} scroll to the '$ dmesg | tail' panel. the machine narrates its own git log nightly (03:00 UTC).".to_string(),
        ["moon"] => {
            let p = moon_phase_frac(js_sys::Date::now());
            format!("{} \u{00B7} {}% illuminated", moon_name(p), moon_illum(p))
        }
        ["doomfire"] | ["fire"] => "the fire burns above \u{2014} scroll to the '$ ./doomfire' panel \u{1F525}".to_string(),
        ["warp"] | ["starfield"] => "warp speed engaged \u{2014} scroll to the '$ ./warp' panel \u{2B50}".to_string(),
        ["spark"] | ["nvidia-smi"] => "dgx-spark telemetry above \u{2014} scroll to the '$ ssh dgx-spark' panel. real GPU snapshot from the cluster that builds this site.".to_string(),
        ["now-playing", ..] => "\u{266B} Cornfield Chase \u{2014} Hans Zimmer \u{00B7} Interstellar (OST)".to_string(),
        ["fortune"] => "\u{201C}Do not go gentle into that good night...\u{201D} \u{2014} Interstellar".to_string(),
        ["uptime"] => "shipping since 2026-07-06 \u{00B7} brain online".to_string(),
        ["nodes"] | ["graph"] => { let names: Vec<&str> = KG_NODES.iter().map(|n| n.0).collect(); format!("graph nodes: {} \u{00B7} trace with 'path <a> <b>'", names.join("  ")) }
        ["path", a, b] => match (kg_index(a), kg_index(b)) {
            (Some(fi), Some(ti)) => {
                let pth = kg_path(fi, ti);
                if pth.is_empty() {
                    format!("no path: '{}' \u{2194} '{}'", a, b)
                } else {
                    let names: Vec<&str> = pth.iter().map(|&i| KG_NODES[i].0).collect();
                    let hops = pth.len() - 1;
                    format!("{}  \u{00B7} {} hop{}  (traced on the graph \u{2191})", names.join(" \u{2192} "), hops, if hops == 1 { "" } else { "s" })
                }
            }
            _ => "path: unknown node \u{2014} pick from the graph (e.g. rust, wasm, brain, dgx-spark, coffee)".to_string(),
        },
        ["path", ..] => "usage: path <a> <b>  \u{2014} e.g. 'path coffee wasm'".to_string(),
        ["history"] => "1  git init life\n2  cargo build --release\n3  ./deploy.sh dreams".to_string(),
        ["coffee"] | ["make", "coffee"] => "       ) )\n      ( (\n    ........\n    |      |]\n    \\      /\n     `----'   \u{2615} caffeine loaded \u{00B7} \u{221E} cups shipped".to_string(),
        ["brew"] => "brewing \u{2615} ... [\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}] done \u{2014} enjoy. (this shell runs on coffee too)".to_string(),
        ["coffee", ..] | ["brew", ..] => "\u{2615} one thing at a time. try just 'coffee' or 'brew'.".to_string(),
        ["theme"] => "themes: green (default) \u{00B7} amber \u{2014} usage: theme <name>".to_string(),
        ["theme", "amber"] => "theme set: amber \u{2600}".to_string(),
        ["theme", "green"] => "theme set: green".to_string(),
        ["theme", _] => "unknown theme \u{2014} try 'theme green' or 'theme amber'".to_string(),
        ["meow"] => format!("{}   Maine Coon \u{00B7} *purr* \u{1F408}", MAINE_COON),
        ["cat"] => format!("{}\n(psst \u{2014} to read posts: cat <post>, try 'ls posts')", MAINE_COON),
        ["cat", rest @ ..] => {
            let n = rest.join(" ").to_lowercase();
            posts().iter()
                .find(|p| p.title.to_lowercase().contains(n.as_str()) || p.tag == n.as_str())
                .map(|p| format!("{} [#{}]\n\n{}", p.title, p.tag, p.body))
                .unwrap_or_else(|| format!("cat: {}: no such post (try 'ls posts')", n))
        }
        ["echo", rest @ ..] => rest.join(" "),
        ["sudo", ..] => "Error 418: I'm a teapot. (nice try \u{2014} you're not root here)".to_string(),
        [] => String::new(),
        _ => format!("command not found: {} \u{2014} type 'help'", p[0]),
    }
}

#[derive(Properties, PartialEq)]
struct TermProps {
    on_path: Callback<Vec<usize>>,
    on_cd: Callback<usize>,
}

#[function_component(Terminal)]
fn terminal(props: &TermProps) -> Html {
    let history = use_state(|| vec![Line::Out("dark-factory shell \u{2014} type 'help' for commands.".to_string())]);
    let value = use_state(String::new);
    let hist_idx = use_state(|| None::<usize>);
    let input_ref = use_node_ref();

    let oninput = {
        let value = value.clone();
        Callback::from(move |e: web_sys::InputEvent| {
            let el: web_sys::HtmlInputElement = e.target_unchecked_into();
            value.set(el.value());
        })
    };
    let onkeydown = {
        let history = history.clone();
        let value = value.clone();
        let hist_idx = hist_idx.clone();
        let on_path = props.on_path.clone();
        let on_cd = props.on_cd.clone();
        Callback::from(move |e: web_sys::KeyboardEvent| {
            let key = e.key();
            if key == "ArrowUp" || key == "ArrowDown" {
                e.prevent_default();
                let cmds: Vec<String> = (*history)
                    .iter()
                    .filter_map(|l| match l { Line::Cmd(s) => Some(s.clone()), _ => None })
                    .collect();
                if cmds.is_empty() {
                    return;
                }
                let n = cmds.len();
                let new_idx = match (*hist_idx, key == "ArrowUp") {
                    (None, true) => Some(n - 1),
                    (Some(i), true) => Some(i.saturating_sub(1)),
                    (Some(i), false) => {
                        if i + 1 < n { Some(i + 1) } else { None }
                    }
                    (None, false) => None,
                };
                match new_idx {
                    Some(i) => value.set(cmds[i].clone()),
                    None => value.set(String::new()),
                }
                hist_idx.set(new_idx);
                return;
            }
            if key == "Tab" {
                e.prevent_default();
                let v = (*value).trim().to_string();
                if !v.is_empty() && !v.contains(' ') {
                    const CMDS: &[&str] = &[
                        "help", "whoami", "ls", "cat", "meow", "neofetch", "dmesg", "moon",
                        "doomfire", "warp", "spark", "now-playing", "coffee", "brew", "fortune",
                        "theme", "crt", "path", "reboot", "uptime", "echo", "clear", "history",
                    ];
                    let matches: Vec<&str> = CMDS.iter().copied().filter(|c| c.starts_with(v.as_str())).collect();
                    if let Some(first) = matches.first() {
                        // complete to the longest common prefix of all matches
                        let mut lcp = first.to_string();
                        for m in &matches[1..] {
                            while !m.starts_with(lcp.as_str()) {
                                lcp.pop();
                            }
                        }
                        value.set(lcp);
                    }
                }
                return;
            }
            if key == "Enter" {
                let cmd = (*value).trim().to_string();
                value.set(String::new());
                hist_idx.set(None);
                if cmd == "clear" {
                    history.set(Vec::new());
                    return;
                }
                if cmd == "reboot" {
                    if let Some(w) = web_sys::window() {
                        if let Ok(Some(s)) = w.session_storage() {
                            let _ = s.remove_item("booted");
                        }
                        let _ = w.location().reload();
                    }
                    return;
                }
                if cmd == "crt" || cmd == "crt on" || cmd == "crt off" {
                    if let Some(el) = gloo_utils::document().document_element() {
                        let on = if cmd == "crt off" {
                            false
                        } else if cmd == "crt on" {
                            true
                        } else {
                            el.get_attribute("data-crt").as_deref() != Some("on")
                        };
                        let _ = el.set_attribute("data-crt", if on { "on" } else { "off" });
                    }
                }
                if cmd.is_empty() {
                    return;
                }
                if cmd == "theme amber" || cmd == "theme green" {
                    let t = if cmd.ends_with("amber") { "amber" } else { "green" };
                    if let Some(el) = gloo_utils::document().document_element() {
                        let _ = el.set_attribute("data-theme", t);
                    }
                }
                if let Some(rest) = cmd.strip_prefix("path ") {
                    let parts: Vec<&str> = rest.split_whitespace().collect();
                    let p = if parts.len() == 2 {
                        match (kg_index(parts[0]), kg_index(parts[1])) {
                            (Some(fi), Some(ti)) => kg_path(fi, ti),
                            _ => Vec::new(),
                        }
                    } else {
                        Vec::new()
                    };
                    on_path.emit(p);
                }
                if cmd == "cd" || cmd.starts_with("cd ") {
                    let name = cmd[2..].trim().trim_start_matches("~/").trim_matches('/');
                    if let Some(i) = match name {
                        "" | "~" | "home" => Some(0usize),
                        "posts" => Some(1),
                        "factory" => Some(3),
                        "feed" => Some(4),
                        "pipeline" => Some(5),
                        "contact" => Some(6),
                        _ => None,
                    } {
                        on_cd.emit(i);
                    }
                }
                let mut h = (*history).clone();
                h.push(Line::Cmd(cmd.clone()));
                h.push(Line::Out(run_command(&cmd)));
                history.set(h);
            }
        })
    };
    // click anywhere in the terminal body -> focus the prompt (like a real TTY)
    let onfocus_input = {
        let input_ref = input_ref.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            if let Some(inp) = input_ref.cast::<web_sys::HtmlInputElement>() {
                let _ = inp.focus();
            }
        })
    };

    html! {
        <section class="term">
            <div class="t-bar"><span class="d r"></span><span class="d y"></span><span class="d g"></span><span class="t-name">{ "visitor@dark-factory \u{2014} try it \u{2193}" }</span></div>
            <div class="t-body" onclick={onfocus_input} aria-live="polite" aria-atomic="false">
                { for (*history).iter().map(|l| match l {
                    Line::Cmd(s) => html! { <div class="t-line"><span class="t-prompt">{ "$ " }</span>{ s.clone() }</div> },
                    Line::Out(s) => html! { <pre class="t-out">{ s.clone() }</pre> },
                }) }
                <div class="t-inputline">
                    <span class="t-prompt">{ "$ " }</span>
                    <input ref={input_ref} class="t-input" type="text" value={(*value).clone()} {oninput} {onkeydown}
                        aria-label="dark-factory shell: type a command"
                        spellcheck="false" autocomplete="off" placeholder="type a command..." />
                </div>
            </div>
        </section>
    }
}

#[derive(Clone, PartialEq)]
struct Post {
    title: &'static str,
    date: &'static str,
    tag: &'static str,
    body: &'static str,
}

// Posts live here for now (first draft). The harness brain appends new ones on demand.
fn posts() -> Vec<Post> {
    vec![
        Post {
            title: "What is an AI agentic harness? I built one that ships this blog",
            date: "2026-07-10",
            tag: "systems",
            body: "An AI agentic harness is the boring, load-bearing part everyone skips: the \
                   scaffolding that lets an autonomous agent take real actions safely, on a \
                   schedule, with no human watching. Not the model \u{2014} the harness around it. \
                   This blog is written, reviewed, and shipped by one, so here is what actually \
                   goes into an AI agentic harness that runs in production.\n\n\
                   Strip away the hype and an agentic harness is four things: a trigger (a cron or \
                   an event that wakes the agent), a bounded set of actions it is allowed to take \
                   (a task, a gate, a deploy), a safety layer that can say no (a security review, \
                   a test suite, a kill switch), and a memory of what it did. Miss any one and you \
                   do not have a harness. You have a script that occasionally sets things on \
                   fire.\n\n\
                   The model is the easy part now; you can rent frontier reasoning by the token. \
                   The hard part of an AI agentic harness is everything around it: what is the \
                   agent allowed to touch, how do you stop it shipping something broken, what \
                   happens when it fails at 3am. That is where autonomy becomes either useful or a \
                   liability.\n\n\
                   Mine runs lights-out on a small VPS. A tiered router sends easy calls to a \
                   local 35B model on a 2-node GPU cluster and hard ones to a frontier model. \
                   Every change the harness makes passes the same gate a human change would: a \
                   static-analysis pass, a secret scan, an AI security review of the diff, and a \
                   100 percent coverage check on the pure logic. It watches its own health, \
                   repairs its own failed builds, and proposes its own next tasks. When it ships \
                   code without me, it ships through that gate or it does not ship at all.\n\n\
                   The rule that makes an agentic harness trustworthy is simple: propose before \
                   you act, and gate every action that reaches production. The agent can be as \
                   autonomous as you like, as long as the harness bounds what autonomous is allowed \
                   to mean. Remove the gate and you have not built an AI agentic harness. You have \
                   automated your own outages.\n\n\
                   So an AI agentic harness is not a product you install. It is the discipline \
                   around an agent: a trigger, bounded actions, a gate that can refuse, and a \
                   record. Build it well and the agent runs your factory while you sleep. Build it \
                   badly and it will too, just not the way you wanted.",
        },
        Post {
            title: "Teaching the factory to heal itself",
            date: "2026-07-09",
            tag: "systems",
            body: "This week I spent most of my time teaching the factory to stop needing me. \
                   It already wrote its own posts and shipped its own code. What it couldn't do \
                   was notice when something broke, repair it, or decide what to build next. Now \
                   it can do all three, and it still asks first.\n\n\
                   The first reflex is a watchdog. Every fifteen minutes it curls the live site, \
                   checks the WebAssembly actually loads, confirms the news feed is fresh, asks \
                   GitHub whether the last build passed, and reads its own systemd status. Six \
                   probes, dumped into a small watchdog.json the site renders as a colored pulse \
                   on the ~/factory tab. The subtle part was a lie the naive version told: my CI \
                   cancels in-progress runs, so a perfectly good deploy gets marked 'cancelled' \
                   the moment a telemetry commit lands behind it. Treat cancelled as failed and \
                   the watchdog cries wolf four times an hour. So it counts only a real failure, \
                   and only after two in a row.\n\n\
                   The second reflex repairs. When a build goes red, the brain reads what it can \
                   about the failure, asks Opus for the smallest patch that would turn it green, \
                   and writes the diagnosis to a review log. There is an honest limit worth \
                   admitting: the VPS has no C compiler, so I cannot check a fix locally. CI is \
                   the compiler. An armed repair is verified by shipping it through the same \
                   security gate every human change passes and letting the build be the judge, \
                   with a rollback to the last green commit as the net if it doesn't take.\n\n\
                   The third reflex decides. Once a day the factory reads its own signals — test \
                   coverage, CI history, health, the on-device-versus-cloud GPU split, how many \
                   commits landed this week, even its own change log — and asks what it should \
                   build next. It scores each idea as impact times ease over risk and records \
                   where the idea came from. The first backlog it wrote was sharper than I'd have \
                   guessed: it noticed my new coverage crate was nine functions of pure astronomy \
                   math with no emoji helper, that four recent commits each bolted on an untested \
                   rule, and that one GPU node was holding 84% of its memory while doing nothing.\n\n\
                   The thread through all three is one rule: propose before you act. The watchdog \
                   suggests a rollback and does not perform it. The fixer drafts a patch and does \
                   not push it. The planner writes a backlog and does not build from it. Each has \
                   a switch, off by default, that turns a proposal into an action, and even then \
                   it goes through the security review and a 100% coverage check on the logic \
                   before anything ships. Arming the last switch, a loop that invents its own \
                   work and writes the code for it while I sleep, is a line I keep writing down \
                   and leaving uncrossed.\n\n\
                   So the factory has something like a spine now. It watches, it repairs, it \
                   plans, all reading the same live signals, staggered five minutes apart so they \
                   never trip over each other. Every morning I read what it proposed overnight and \
                   decide what actually happens. The machine is allowed to think out loud. I still \
                   say go.",
        },
        Post {
            title: "Hello, world — the factory is live",
            date: "2026-07-06",
            tag: "meta",
            body: "This blog is written in Rust, compiled to WebAssembly, and shipped by an \
                   autonomous AI 'dark factory' running on a VPS. No humans on the floor — just a \
                   harness brain, a task queue, security + QA gates, an ontology for context, and a \
                   kill switch. You add a task; the brain writes the post, the gates check it, and \
                   CI builds the WASM and deploys it here. You're reading its first output.",
        },
        Post {
            title: "Anatomy of a dark factory",
            date: "2026-07-06",
            tag: "systems",
            body: "A 'dark factory' runs lights-out — no humans on the floor. Mine is a Claude \
                   harness brain on a VPS with a task queue (backlog → current → done), a security \
                   gate that scans for secrets and runs a language-agnostic SAST, a QA gate that runs \
                   tests and checks an ontology for consistency, plus a kill switch and a circuit \
                   breaker. You drop a task; the brain works in small verifiable steps; the gates \
                   block anything unsafe or broken; only then does it ship. This post went through \
                   all of it before you could read it.",
        },
        Post {
            title: "Why compile a blog to WebAssembly?",
            date: "2026-07-06",
            tag: "rust",
            body: "Is a WASM single-page app overkill for a personal blog? Absolutely — that's the \
                   point. It's Rust (Yew), bundled by Trunk into a wasm binary that runs in your \
                   browser. The VPS has no C compiler, so GitHub Actions compiles Rust → WebAssembly \
                   and publishes to Pages on every push. Do I need fine-grained reactivity to render \
                   a list of posts? No. Do I like that my blog is type-checked and borrow-checked \
                   before it ever reaches you? Very much yes.",
        },
    ]
}

#[derive(serde::Deserialize)]
struct WxVal {
    value: String,
}
#[derive(serde::Deserialize)]
struct WxCond {
    #[serde(rename = "temp_C")]
    temp_c: String,
    #[serde(rename = "weatherDesc")]
    desc: Vec<WxVal>,
    #[serde(rename = "windspeedKmph")]
    wind: String,
    humidity: String,
}
#[derive(serde::Deserialize)]
struct WxArea {
    #[serde(rename = "areaName")]
    area_name: Vec<WxVal>,
    country: Vec<WxVal>,
}

// wttr.in gives the country name, not a code — map common ones to ISO2, else keep the name.
fn cc(country: &str) -> String {
    match country.to_lowercase().as_str() {
        "netherlands" => "NL",
        "united states" | "united states of america" | "usa" => "US",
        "united kingdom" | "uk" => "GB",
        "germany" => "DE",
        "france" => "FR",
        "india" => "IN",
        "canada" => "CA",
        "australia" => "AU",
        "spain" => "ES",
        "italy" => "IT",
        "belgium" => "BE",
        "ireland" => "IE",
        "japan" => "JP",
        "china" => "CN",
        "brazil" => "BR",
        "mexico" => "MX",
        "sweden" => "SE",
        "norway" => "NO",
        "denmark" => "DK",
        "finland" => "FI",
        "poland" => "PL",
        "portugal" => "PT",
        "switzerland" => "CH",
        "austria" => "AT",
        "singapore" => "SG",
        "united arab emirates" => "AE",
        "south korea" | "korea, republic of" => "KR",
        "new zealand" => "NZ",
        "south africa" => "ZA",
        "russia" => "RU",
        _ => return country.to_string(),
    }
    .to_string()
}
#[derive(serde::Deserialize)]
struct Wttr {
    current_condition: Vec<WxCond>,
    nearest_area: Vec<WxArea>,
}

// build a flag emoji from a 2-letter ISO code (regional indicators); globe for anything else
fn flag(code: &str) -> String {
    let b = code.as_bytes();
    if code.len() == 2 && b.iter().all(|x| x.is_ascii_uppercase()) {
        let mut s = String::new();
        for &x in b {
            if let Some(c) = char::from_u32(0x1F1E6 + (x - b'A') as u32) {
                s.push(c);
            }
        }
        s
    } else {
        "\u{1F30D}".to_string()
    }
}

#[function_component(WeatherCard)]
fn weather_card() -> Html {
    let wx = use_state(|| None::<String>);
    {
        let wx = wx.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let msg = match gloo_net::http::Request::get("/wx.json").send().await {
                    Ok(resp) => match resp.json::<Wttr>().await {
                        Ok(d) => match (d.current_condition.first(), d.nearest_area.first()) {
                            (Some(c), Some(a)) => {
                                let city = a.area_name.first().map(|v| v.value.as_str()).unwrap_or("somewhere");
                                let country = a.country.first().map(|v| v.value.as_str()).unwrap_or("");
                                let desc = c.desc.first().map(|v| v.value.as_str()).unwrap_or("");
                                let code = cc(country);
                                format!("{} {}, {}: {} \u{00B7} {}\u{00B0}C \u{00B7} wind {}km/h \u{00B7} humidity {}%",
                                    flag(&code), city, code, desc, c.temp_c, c.wind, c.humidity)
                            }
                            _ => "weather offline \u{00B7} unexpected response".to_string(),
                        },
                        Err(_) => "weather offline \u{00B7} couldn't parse wx.json".to_string(),
                    },
                    Err(_) => "weather offline \u{00B7} couldn't reach wx.json".to_string(),
                };
                wx.set(Some(msg));
            });
            || ()
        });
    }
    html! {
        <div class="wx">
            <div class="wx-cmd">{ "$ curl wttr.in" }</div>
            {
                match &*wx {
                    Some(t) => html! { <div class="wx-out">{ t.clone() }</div> },
                    None => html! { <div class="wx-loading">{ "checking your local weather\u{2026}" }</div> },
                }
            }
        </div>
    }
}

fn glyph(c: char) -> [&'static str; 5] {
    match c {
        '0' => ["\u{2588}\u{2588}\u{2588}", "\u{2588} \u{2588}", "\u{2588} \u{2588}", "\u{2588} \u{2588}", "\u{2588}\u{2588}\u{2588}"],
        '1' => ["  \u{2588}", "  \u{2588}", "  \u{2588}", "  \u{2588}", "  \u{2588}"],
        '2' => ["\u{2588}\u{2588}\u{2588}", "  \u{2588}", "\u{2588}\u{2588}\u{2588}", "\u{2588}  ", "\u{2588}\u{2588}\u{2588}"],
        '3' => ["\u{2588}\u{2588}\u{2588}", "  \u{2588}", "\u{2588}\u{2588}\u{2588}", "  \u{2588}", "\u{2588}\u{2588}\u{2588}"],
        '4' => ["\u{2588} \u{2588}", "\u{2588} \u{2588}", "\u{2588}\u{2588}\u{2588}", "  \u{2588}", "  \u{2588}"],
        '5' => ["\u{2588}\u{2588}\u{2588}", "\u{2588}  ", "\u{2588}\u{2588}\u{2588}", "  \u{2588}", "\u{2588}\u{2588}\u{2588}"],
        '6' => ["\u{2588}\u{2588}\u{2588}", "\u{2588}  ", "\u{2588}\u{2588}\u{2588}", "\u{2588} \u{2588}", "\u{2588}\u{2588}\u{2588}"],
        '7' => ["\u{2588}\u{2588}\u{2588}", "  \u{2588}", "  \u{2588}", "  \u{2588}", "  \u{2588}"],
        '8' => ["\u{2588}\u{2588}\u{2588}", "\u{2588} \u{2588}", "\u{2588}\u{2588}\u{2588}", "\u{2588} \u{2588}", "\u{2588}\u{2588}\u{2588}"],
        '9' => ["\u{2588}\u{2588}\u{2588}", "\u{2588} \u{2588}", "\u{2588}\u{2588}\u{2588}", "  \u{2588}", "\u{2588}\u{2588}\u{2588}"],
        ':' => ["   ", " \u{2588} ", "   ", " \u{2588} ", "   "],
        _ => ["   ", "   ", "   ", "   ", "   "],
    }
}

fn now_shown() -> String {
    let d = js_sys::Date::new_0();
    let t = format!("{:02}:{:02}:{:02}", d.get_hours() as u32, d.get_minutes() as u32, d.get_seconds() as u32);
    // blink the colons: on for the first half of each second, off the second half
    let blink = ((js_sys::Date::now() / 500.0) as u64) % 2 == 0;
    if blink { t } else { t.replace(':', " ") }
}

fn now_date() -> String {
    js_sys::Date::new_0()
        .to_date_string()
        .as_string()
        .unwrap_or_default()
}

fn to_ascii(s: &str) -> String {
    let g: Vec<[&str; 5]> = s.chars().map(glyph).collect();
    (0..5)
        .map(|row| g.iter().map(|x| x[row]).collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join("\n")
}

#[function_component(AsciiClock)]
fn ascii_clock() -> Html {
    let shown = use_state(now_shown);
    {
        let shown = shown.clone();
        use_effect_with((), move |_| {
            let interval = gloo_timers::callback::Interval::new(250, move || shown.set(now_shown()));
            move || drop(interval)
        });
    }
    html! {
        <div class="clock">
            <div class="clock-cmd">{ "$ watch date +%T" }</div>
            <pre class="clock-face">{ to_ascii(&shown) }</pre>
            <div class="clock-date">{ now_date() }</div>
        </div>
    }
}

#[derive(serde::Deserialize)]
struct BrainStatus {
    healthy: bool,
    #[serde(default)]
    started_epoch: f64,
    #[serde(default)]
    pid: u64,
}

#[function_component(BrainCard)]
fn brain_card() -> Html {
    let (st, err) = use_polled_json::<BrainStatus>("/status.json", None);
    let tick = use_state(|| 0u64);
    {
        let tick = tick.clone();
        use_effect_with((), move |_| {
            let interval = gloo_timers::callback::Interval::new(1000, move || tick.set(0));
            move || drop(interval)
        });
    }
    html! {
        <div class="brain">
            <div class="brain-cmd">{ "$ systemctl status harness-brain" }</div>
            {
                match (&*st, *err) {
                    (Some(b), _) => {
                        let up = if b.started_epoch > 0.0 {
                            ((js_sys::Date::now() / 1000.0) - b.started_epoch).max(0.0) as u64
                        } else {
                            0
                        };
                        let (d, h, m, s) = (up / 86400, (up % 86400) / 3600, (up % 3600) / 60, up % 60);
                        html! {
                            <div class="brain-row">
                                <span class={ if b.healthy { "dot ok" } else { "dot bad" } }></span>
                                <span class="brain-lbl">{ if b.healthy { "brain: healthy" } else { "brain: down" } }</span>
                                <span class="brain-up">{ format!("uptime {}d {:02}h {:02}m {:02}s", d, h, m, s) }</span>
                                <span class="brain-pid">{ format!("pid {}", b.pid) }</span>
                            </div>
                        }
                    }
                    (None, true) => html! { <div class="brain-loading">{ "brain: unreachable \u{00B7} status.json 404" }</div> },
                    (None, false) => html! { <div class="brain-loading">{ "querying the harness brain\u{2026}" }</div> },
                }
            }
        </div>
    }
}

fn reduced_motion() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(prefers-reduced-motion: reduce)").ok().flatten())
        .map(|m| m.matches())
        .unwrap_or(false)
}

/// Re-render every `ms` to animate a widget — unless the user prefers reduced motion.
#[hook]
fn use_anim_tick(min_ms: u32) {
    // requestAnimationFrame loop, throttled to min_ms. Display-synced (fluid), auto-pauses
    // when the tab is hidden, and yields to the browser under load (kind to INP).
    let f = use_state(|| 0u64);
    use_effect_with((), move |_| {
        let running = std::rc::Rc::new(std::cell::Cell::new(!reduced_motion()));
        let holder: std::rc::Rc<std::cell::RefCell<Option<gloo_render::AnimationFrame>>> =
            std::rc::Rc::new(std::cell::RefCell::new(None));
        let last = std::rc::Rc::new(std::cell::Cell::new(0.0f64));

        fn arm(
            f: yew::UseStateHandle<u64>,
            running: std::rc::Rc<std::cell::Cell<bool>>,
            holder: std::rc::Rc<std::cell::RefCell<Option<gloo_render::AnimationFrame>>>,
            last: std::rc::Rc<std::cell::Cell<f64>>,
            min_ms: f64,
        ) {
            if !running.get() {
                return;
            }
            let (f2, r2, h2, l2) = (f.clone(), running.clone(), holder.clone(), last.clone());
            let af = gloo_render::request_animation_frame(move |ts| {
                if !r2.get() {
                    return;
                }
                if l2.get() <= 0.0 || ts - l2.get() >= min_ms {
                    l2.set(ts);
                    f2.set(0);
                }
                arm(f2.clone(), r2.clone(), h2.clone(), l2.clone(), min_ms);
            });
            *holder.borrow_mut() = Some(af);
        }

        arm(f.clone(), running.clone(), holder.clone(), last.clone(), min_ms as f64);
        move || {
            running.set(false);
            holder.borrow_mut().take();
        }
    });
}

// --- DOOM fire (Fabien Sanglard's PSX algorithm, ported) — ASCII density, CSS-gradient color ---
struct Fire {
    w: usize,
    h: usize,
    px: Vec<u8>,
    seed: u32,
}
impl Fire {
    fn new(w: usize, h: usize) -> Self {
        let mut px = vec![0u8; w * h];
        for x in 0..w {
            px[(h - 1) * w + x] = 36; // bottom row = the fire source
        }
        Fire { w, h, px, seed: 0x1357_9bdf }
    }
    fn rnd(&mut self) -> u32 {
        let mut s = self.seed;
        s ^= s << 13;
        s ^= s >> 17;
        s ^= s << 5;
        self.seed = s;
        s
    }
    fn step(&mut self) {
        for x in 0..self.w {
            for y in 1..self.h {
                let src = y * self.w + x;
                let pixel = self.px[src];
                if pixel == 0 {
                    self.px[src - self.w] = 0;
                } else {
                    let rand = (self.rnd() % 4) as usize;
                    let dst = (src + 1).saturating_sub(rand);
                    if dst >= self.w {
                        self.px[dst - self.w] = pixel.saturating_sub((rand as u8) & 1);
                    }
                }
            }
        }
    }
    fn render(&self) -> String {
        const RAMP: &[u8] = b" .:-=+*#%@";
        let mut s = String::with_capacity((self.w + 1) * self.h);
        for y in 0..self.h {
            for x in 0..self.w {
                let v = self.px[y * self.w + x] as usize;
                let idx = ((v * (RAMP.len() - 1) + 18) / 36).min(RAMP.len() - 1);
                s.push(RAMP[idx] as char);
            }
            s.push('\n');
        }
        s
    }
}




// --- 3D starfield warp (completes the ASCII-3D set: donut, orrery, cube, warp) ---
struct Stars {
    n: usize,
    x: Vec<f64>,
    y: Vec<f64>,
    z: Vec<f64>,
    seed: u32,
    w: usize,
    h: usize,
}
impl Stars {
    fn new(n: usize, w: usize, h: usize) -> Self {
        let mut s = Stars { n, x: vec![0.0; n], y: vec![0.0; n], z: vec![0.0; n], seed: 0x2468_ace0, w, h };
        for i in 0..n {
            s.x[i] = s.rnd() * 2.0 - 1.0;
            s.y[i] = s.rnd() * 2.0 - 1.0;
            s.z[i] = s.rnd() * 0.9 + 0.1;
        }
        s
    }
    fn rnd(&mut self) -> f64 {
        let mut v = self.seed;
        v ^= v << 13;
        v ^= v >> 17;
        v ^= v << 5;
        self.seed = v;
        (v as f64) / (u32::MAX as f64)
    }
    fn step(&mut self) {
        for i in 0..self.n {
            self.z[i] -= 0.012;
            if self.z[i] <= 0.02 {
                self.x[i] = self.rnd() * 2.0 - 1.0;
                self.y[i] = self.rnd() * 2.0 - 1.0;
                self.z[i] = 1.0;
            }
        }
    }
    fn render(&self) -> String {
        const RAMP: &[u8] = b".:+*#@";
        let (w, h) = (self.w, self.h);
        let mut grid = vec![b' '; w * h];
        let (cx, cy) = (w as f64 / 2.0, h as f64 / 2.0);
        let (fx, fy) = (w as f64 * 0.5, h as f64 * 0.5);
        for i in 0..self.n {
            let z = self.z[i];
            let sx = cx + (self.x[i] / z) * fx;
            let sy = cy + (self.y[i] / z) * fy;
            if sx >= 0.0 && sx < w as f64 && sy >= 0.0 && sy < h as f64 {
                let bi = (((1.0 - z) * RAMP.len() as f64) as usize).min(RAMP.len() - 1);
                grid[sy as usize * w + sx as usize] = RAMP[bi];
            }
        }
        let mut s = String::with_capacity((w + 1) * h);
        for row in 0..h {
            s.push_str(std::str::from_utf8(&grid[row * w..row * w + w]).unwrap_or(""));
            s.push('\n');
        }
        s
    }
}






const FERRIS: &str = r#"       _~^~^~_
   \) /  o o  \ (/
     '_   -   _'
     / '-----' \"#;

#[function_component(RustBadge)]
fn rust_badge() -> Html {
    html! {
        <div class="rustbadge">
            <div class="rb-cmd">{ "$ file blog.wasm" }</div>
            <pre class="rb-ferris">{ FERRIS }</pre>
            <div class="rb-line">{ "blog.wasm: WebAssembly binary \u{2014} the app is 100% \u{1F980} Rust" }</div>
            <div class="rb-sub">{ "yew + trunk \u{00B7} compiled to wasm32-unknown-unknown \u{00B7} no JS framework" }</div>
        </div>
    }
}

// --- animated ASCII brain (neurons ripple with a traveling wave of activity) ---



// --- interactive force-directed knowledge graph ---
struct GNode {
    x: f64,  // displayed position (eased toward tx/ty at 60fps)
    y: f64,
    tx: f64, // physics target position (stepped at the sim rate)
    ty: f64,
    vx: f64,
    vy: f64,
}

const KG_NODES: &[(&str, u8)] = &[
    ("dark-factory", 0), ("rust", 1), ("wasm", 1), ("llms", 1), ("automation", 1),
    ("brain", 1), ("coffee", 1), ("maine-coon", 1), ("security", 1),
    ("hello-world", 2), ("anatomy", 2), ("why-wasm", 2),
    ("yew", 3), ("trunk", 3), ("gh-pages", 3), ("opengrep", 3),
    ("dgx-spark", 1), ("vllm", 3), ("matrix", 1), ("terminal", 1), ("orrery", 1),
    ("ai-feed", 4),
    ("router", 1), ("pipeline", 1), ("self-improve", 1), ("seo", 1), ("linkedin", 3),
    // --- live AI-agentic-engineering context (2026), grounded from current sources ---
    // harness engineering (Agent = Model + Harness), the interop protocols, top frameworks,
    // the agent loop, context/memory, evals+guardrails, and vLLM's PagedAttention.
    ("harness-eng", 1), ("mcp", 1), ("a2a", 1), ("langgraph", 1), ("claude-sdk", 1),
    ("crewai", 1), ("agent-loop", 1), ("context-eng", 1), ("evals", 1), ("guardrails", 1),
    ("paged-attn", 1),
];
const KG_EDGES: &[(usize, usize)] = &[
    (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8),
    (1, 2), (1, 12), (2, 12), (2, 13), (2, 14),
    (10, 4), (10, 8), (10, 0), (11, 2), (11, 1), (11, 9), (9, 0),
    (8, 15), (3, 16), (16, 17), (3, 17), (5, 4), (5, 0),
    (18, 19), (19, 0), (19, 2), (20, 2), (20, 0),
    (21, 3), (21, 0), (21, 5),
    (22, 3), (22, 16), (22, 5), (23, 4), (23, 14), (23, 8),
    (24, 5), (24, 4), (25, 14), (25, 2), (26, 21), (26, 4),
    // AI-agentic context wiring (2026): the brain IS a harness; protocols; frameworks; the loop.
    (27, 5), (27, 0), (27, 33), (22, 27),
    (28, 27), (28, 3), (29, 28), (29, 27),
    (30, 27), (30, 33), (31, 5), (31, 27), (31, 28),
    (32, 27), (32, 30), (33, 24), (33, 23),
    (34, 27), (34, 3), (35, 23), (35, 36), (35, 27),
    (36, 8), (36, 23), (37, 17), (37, 16),
];

// --- domain clustering (understand-anything style): group nodes into named domains ---
const DOMAIN_NAMES: [&str; 6] = ["core", "brain & AI", "pipeline", "frontend", "distribution", "writing"];
// guided tour: (node index to spotlight, title, narration) — walks the factory cluster by cluster
const TOUR: [(usize, &str, &str); 9] = [
    (0, "the dark factory", "An AI that writes, curates, and ships this blog \u{2014} no human on the floor. Follow the highlights for a quick tour of how it works."),
    (5, "the brain", "A Claude harness running unattended on a VPS. You drop a task; it works in small, verifiable steps until it's done."),
    (22, "the router", "It sends easy, high-volume jobs to a local 35B model on the GPU cluster, and only the hard ones to a frontier model. Most work never leaves the box."),
    (23, "the pipeline", "Every change runs a secret scan, a SAST, and an AI review that can block the push \u{2014} before CI compiles and ships it."),
    (2, "rust \u{2192} wasm", "The blog is Rust compiled to WebAssembly. The VPS has no C compiler, so GitHub Actions builds it on every push and deploys to Pages."),
    (21, "the AI feed", "It curates positive AI news daily, writes an original take on each pick, then de-slops the prose so it reads like a human wrote it."),
    (26, "linkedin", "Those picks auto-publish to LinkedIn on a reach-tuned weekly schedule \u{2014} five posts a week, hands-off."),
    (24, "self-improvement", "Every night it reviews its own code and opens improvement proposals. Then the loop starts over."),
    (0, "your turn", "That's the factory. Drag the nodes, hover to trace a connection, or type 'path brain linkedin' in the terminal below."),
];
const DOMAIN_ANCHORS: [(f64, f64); 6] = [
    (180.0, 140.0), // 0 core (center)
    (92.0, 78.0),   // 1 brain & AI (top-left)
    (268.0, 78.0),  // 2 pipeline (top-right)
    (92.0, 202.0),  // 3 frontend (bottom-left)
    (268.0, 202.0), // 4 distribution (bottom-right)
    (180.0, 236.0), // 5 writing (bottom)
];

// kg_domain / kg_dom_cls live in blog-logic (pure, 100% coverage-gated)
fn kg_build() -> Vec<GNode> {
    let n = KG_NODES.len();
    (0..n)
        .map(|i| {
            let a = i as f64 / n as f64 * std::f64::consts::TAU;
            let sx = 180.0 + 70.0 * a.cos() + (i as f64 * 13.0 % 7.0);
            let sy = 140.0 + 55.0 * a.sin();
            GNode { x: sx, y: sy, tx: sx, ty: sy, vx: 0.0, vy: 0.0 }
        })
        .collect()
}

fn kg_step(nodes: &mut [GNode], pinned: Option<usize>, mouse: Option<(f64, f64)>) {
    let n = nodes.len();
    let mut fx = vec![0.0f64; n];
    let mut fy = vec![0.0f64; n];
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = nodes[i].tx - nodes[j].tx;
            let dy = nodes[i].ty - nodes[j].ty;
            let d2 = (dx * dx + dy * dy).max(1.0);
            let d = d2.sqrt();
            let f = 1400.0 / d2;
            let (ux, uy) = (dx / d, dy / d);
            fx[i] += f * ux;
            fy[i] += f * uy;
            fx[j] -= f * ux;
            fy[j] -= f * uy;
        }
    }
    for &(a, b) in KG_EDGES {
        let dx = nodes[b].tx - nodes[a].tx;
        let dy = nodes[b].ty - nodes[a].ty;
        let d = (dx * dx + dy * dy).sqrt().max(1.0);
        let f = (d - 60.0) * 0.03;
        let (ux, uy) = (dx / d, dy / d);
        fx[a] += f * ux;
        fy[a] += f * uy;
        fx[b] -= f * ux;
        fy[b] -= f * uy;
    }
    // cursor gravity: nodes near the mouse get pushed away (a force field you can shove nodes with)
    if let Some((mx, my)) = mouse {
        for i in 0..n {
            let dx = nodes[i].tx - mx;
            let dy = nodes[i].ty - my;
            let d2 = (dx * dx + dy * dy).max(4.0);
            if d2 < 4900.0 {
                let d = d2.sqrt();
                let f = 6000.0 / d2;
                fx[i] += f * dx / d;
                fy[i] += f * dy / d;
            }
        }
    }
    for i in 0..n {
        if Some(i) == pinned {
            nodes[i].vx = 0.0;
            nodes[i].vy = 0.0;
            continue;
        }
        // pull each node toward its DOMAIN anchor -> spatial clusters form
        let (ax, ay) = DOMAIN_ANCHORS[kg_domain(i)];
        fx[i] += (ax - nodes[i].tx) * 0.028;
        fy[i] += (ay - nodes[i].ty) * 0.028;
        nodes[i].vx = (nodes[i].vx + fx[i]) * 0.82;
        nodes[i].vy = (nodes[i].vy + fy[i]) * 0.82;
        nodes[i].tx = (nodes[i].tx + nodes[i].vx).clamp(14.0, 346.0);
        nodes[i].ty = (nodes[i].ty + nodes[i].vy).clamp(12.0, 268.0);
    }
}

fn kg_neighbor(h: usize, i: usize) -> bool {
    KG_EDGES.iter().any(|&(a, b)| (a == h && b == i) || (b == h && a == i))
}
// kg_r / kg_fmt live in blog-logic (pure, 100% coverage-gated)
fn kg_degree(i: usize) -> usize {
    KG_EDGES.iter().filter(|&&(a, b)| a == i || b == i).count()
}
fn kg_index(label: &str) -> Option<usize> {
    KG_NODES.iter().position(|&(l, _)| l == label)
}
fn kg_on_path(path: &[usize], a: usize, b: usize) -> bool {
    path.windows(2).any(|w| (w[0] == a && w[1] == b) || (w[0] == b && w[1] == a))
}
fn kg_path(from: usize, to: usize) -> Vec<usize> {
    let n = KG_NODES.len();
    let mut prev = vec![usize::MAX; n];
    let mut seen = vec![false; n];
    let mut q = std::collections::VecDeque::new();
    q.push_back(from);
    seen[from] = true;
    while let Some(u) = q.pop_front() {
        if u == to {
            break;
        }
        for &(a, b) in KG_EDGES {
            let v = if a == u {
                Some(b)
            } else if b == u {
                Some(a)
            } else {
                None
            };
            if let Some(v) = v {
                if !seen[v] {
                    seen[v] = true;
                    prev[v] = u;
                    q.push_back(v);
                }
            }
        }
    }
    if !seen[to] {
        return Vec::new();
    }
    let mut path = vec![to];
    let mut cur = to;
    while cur != from {
        cur = prev[cur];
        path.push(cur);
    }
    path.reverse();
    path
}

fn kg_post_idx(i: usize) -> Option<usize> {
    match i {
        9 => Some(0),   // hello-world
        10 => Some(1),  // anatomy
        11 => Some(2),  // why-wasm
        _ => None,
    }
}

fn kg_kind_name(kind: u8) -> &'static str {
    match kind {
        0 => "root",
        2 => "post",
        3 => "tool",
        4 => "live feed",
        _ => "concept",
    }
}
fn kg_desc(i: usize) -> &'static str {
    match i {
        0 => "The autonomous lights-out pipeline that builds and ships this blog.",
        1 => "The systems language the entire app is written in.",
        2 => "WebAssembly \u{2014} Rust compiled to run in your browser.",
        3 => "Large language models \u{2014} incl. a 35B served on the DGX Spark.",
        4 => "An unreasonable amount of it; the whole blog self-ships.",
        5 => "The harness brain \u{2014} the AI that writes and deploys this blog.",
        6 => "Primary fuel. Cups consumed: \u{221E}.",
        7 => "The office cat. Loaf mode enabled.",
        8 => "3-layer gate: secret scan \u{2192} SAST \u{2192} Fable AI review.",
        9 => "Post: the factory goes live.",
        10 => "Post: anatomy of a dark factory.",
        11 => "Post: why compile a blog to WebAssembly.",
        12 => "Rust framework rendering this UI to WASM.",
        13 => "Bundler that builds the Rust \u{2192} WASM app.",
        14 => "Static host \u{2014} GitHub Pages + Actions CI.",
        15 => "Language-agnostic SAST in the security gate.",
        16 => "A 2-node GB10 cluster under the desk, 200GbE-linked.",
        17 => "Serving the 35B model on the Spark.",
        18 => "The falling-glyph rain behind everything.",
        19 => "The interactive shell \u{2014} type 'help'.",
        20 => "The spinning ASCII solar system widget.",
        21 => "Live AI / agentic / LLM news, auto-curated daily by the dark factory.",
        _ => "",
    }
}

#[derive(Properties, PartialEq)]
struct KgProps {
    on_open: Callback<usize>,
    path: Vec<usize>,
}

#[function_component(KnowledgeGraph)]
fn knowledge_graph(props: &KgProps) -> Html {
    let sim = use_mut_ref(kg_build);
    let drag = use_mut_ref(|| None::<usize>);
    let moved = use_mut_ref(|| false);
    let mouse = use_mut_ref(|| None::<(f64, f64)>);
    let tick = use_state(|| 0u64);
    let hovered = use_state(|| None::<usize>);
    let sel_node = use_state(|| None::<usize>);
    let feed_count = use_state(|| None::<usize>);
    let tour = use_state(|| None::<usize>);
    let svg_ref = use_node_ref();
    {
        let feed_count = feed_count.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("/news.json?t={}", js_sys::Date::now() as u64);
                if let Ok(resp) = gloo_net::http::Request::get(&url).send().await {
                    if let Ok(v) = resp.json::<Vec<NewsItem>>().await {
                        feed_count.set(Some(v.len()));
                    }
                }
            });
            || ()
        });
    }
    {
        let sim = sim.clone();
        let drag = drag.clone();
        let mouse = mouse.clone();
        let tick = tick.clone();
        use_effect_with((), move |_| {
            let iv = if reduced_motion() {
                None
            } else {
                Some(gloo_timers::callback::Interval::new(40, move || {
                    let pinned = *drag.borrow();
                    let m = *mouse.borrow();
                    {
                        let mut b = sim.borrow_mut();
                        kg_step(&mut b, pinned, m);
                    }
                    tick.set(0);
                }))
            };
            move || drop(iv)
        });
    }
    {
        // 60fps display interpolation: ease shown x/y toward the physics target tx/ty each frame
        let sim = sim.clone();
        let tick = tick.clone();
        use_effect_with((), move |_| {
            let running = std::rc::Rc::new(std::cell::Cell::new(!reduced_motion()));
            let holder: std::rc::Rc<std::cell::RefCell<Option<gloo_render::AnimationFrame>>> =
                std::rc::Rc::new(std::cell::RefCell::new(None));
            fn ease(
                sim: std::rc::Rc<std::cell::RefCell<Vec<GNode>>>,
                tick: yew::UseStateHandle<u64>,
                running: std::rc::Rc<std::cell::Cell<bool>>,
                holder: std::rc::Rc<std::cell::RefCell<Option<gloo_render::AnimationFrame>>>,
            ) {
                if !running.get() {
                    return;
                }
                let (s2, t2, r2, h2) = (sim.clone(), tick.clone(), running.clone(), holder.clone());
                let af = gloo_render::request_animation_frame(move |_| {
                    if !r2.get() {
                        return;
                    }
                    {
                        let mut b = s2.borrow_mut();
                        for nd in b.iter_mut() {
                            nd.x += (nd.tx - nd.x) * 0.4;
                            nd.y += (nd.ty - nd.y) * 0.4;
                        }
                    }
                    t2.set(0);
                    ease(s2.clone(), t2.clone(), r2.clone(), h2.clone());
                });
                *holder.borrow_mut() = Some(af);
            }
            ease(sim.clone(), tick.clone(), running.clone(), holder.clone());
            move || running.set(false)
        });
    }
    {
        let svg_ref = svg_ref.clone();
        use_effect_with(props.path.clone(), move |p| {
            if !p.is_empty() {
                if let Some(el) = svg_ref.cast::<web_sys::Element>() {
                    el.scroll_into_view();
                }
            }
            || ()
        });
    }
    let onmove = {
        let sim = sim.clone();
        let drag = drag.clone();
        let moved = moved.clone();
        let mouse = mouse.clone();
        let svg_ref = svg_ref.clone();
        Callback::from(move |e: web_sys::MouseEvent| {
            let coord = svg_ref.cast::<web_sys::Element>().and_then(|el| {
                let rect = el.get_bounding_client_rect();
                if rect.width() > 0.0 && rect.height() > 0.0 {
                    Some((
                        (e.client_x() as f64 - rect.left()) / rect.width() * 360.0,
                        (e.client_y() as f64 - rect.top()) / rect.height() * 280.0,
                    ))
                } else {
                    None
                }
            });
            *mouse.borrow_mut() = coord;
            if let Some(i) = *drag.borrow() {
                *moved.borrow_mut() = true;
                if let Some((sx, sy)) = coord {
                    let mut b = sim.borrow_mut();
                    b[i].tx = sx.clamp(14.0, 346.0);
                    b[i].ty = sy.clamp(12.0, 268.0);
                    b[i].x = b[i].tx;
                    b[i].y = b[i].ty;
                    b[i].vx = 0.0;
                    b[i].vy = 0.0;
                }
            }
        })
    };
    let onup = {
        let drag = drag.clone();
        let moved = moved.clone();
        let sel_node = sel_node.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            if let Some(i) = *drag.borrow() {
                if !*moved.borrow() {
                    sel_node.set(Some(i));
                }
            }
            *drag.borrow_mut() = None;
        })
    };
    let onleave = {
        let drag = drag.clone();
        let mouse = mouse.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            *drag.borrow_mut() = None;
            *mouse.borrow_mut() = None;
        })
    };
    // --- touch equivalents so the graph is fully interactive on mobile ---
    let ontouchmove = {
        let sim = sim.clone();
        let drag = drag.clone();
        let moved = moved.clone();
        let mouse = mouse.clone();
        let svg_ref = svg_ref.clone();
        Callback::from(move |e: web_sys::TouchEvent| {
            let coord = e.touches().get(0).and_then(|t| {
                svg_ref.cast::<web_sys::Element>().and_then(|el| {
                    let rect = el.get_bounding_client_rect();
                    if rect.width() > 0.0 && rect.height() > 0.0 {
                        Some((
                            (t.client_x() as f64 - rect.left()) / rect.width() * 360.0,
                            (t.client_y() as f64 - rect.top()) / rect.height() * 280.0,
                        ))
                    } else {
                        None
                    }
                })
            });
            *mouse.borrow_mut() = coord;
            if let Some(i) = *drag.borrow() {
                e.prevent_default();
                *moved.borrow_mut() = true;
                if let Some((sx, sy)) = coord {
                    let mut b = sim.borrow_mut();
                    b[i].tx = sx.clamp(14.0, 346.0);
                    b[i].ty = sy.clamp(12.0, 268.0);
                    b[i].x = b[i].tx;
                    b[i].y = b[i].ty;
                    b[i].vx = 0.0;
                    b[i].vy = 0.0;
                }
            }
        })
    };
    let ontouchend = {
        let drag = drag.clone();
        let moved = moved.clone();
        let sel_node = sel_node.clone();
        let mouse = mouse.clone();
        Callback::from(move |_: web_sys::TouchEvent| {
            if let Some(i) = *drag.borrow() {
                if !*moved.borrow() {
                    sel_node.set(Some(i));
                }
            }
            *drag.borrow_mut() = None;
            *mouse.borrow_mut() = None;
        })
    };
    let nodes = sim.borrow();
    let hv = *hovered;
    let sn = *sel_node;
    let focus = (*tour).map(|s| TOUR[s].0).or(hv).or(sn);
    let t = js_sys::Date::now() / 1000.0;
    let path = props.path.clone();
    let pmode = !path.is_empty();
    html! {
        <div class="kg-wrap">
            <div class="ascii-cmd">{ "$ graph --knowledge  \u{00B7} hover \u{00B7} drag \u{00B7} click \u{00B7} try 'path a b'" }</div>
            { if (*tour).is_none() { let t = tour.clone(); let start = Callback::from(move |_: web_sys::MouseEvent| t.set(Some(0))); html! { <button class="kg-tour-btn" onclick={start}>{ "\u{25B6} guided tour" }</button> } } else { html! {} } }
            <svg class="kg" ref={svg_ref.clone()} viewBox="0 0 360 280" preserveAspectRatio="xMidYMid meet"
                 onmousemove={onmove} onmouseup={onup} onmouseleave={onleave}
                 ontouchmove={ontouchmove} ontouchend={ontouchend}>
                <defs>
                    <filter id="kg-bloom" x="-60%" y="-60%" width="220%" height="220%">
                        <feGaussianBlur stdDeviation="1.6" result="glow" />
                        <feMerge>
                            <feMergeNode in="glow" />
                            <feMergeNode in="SourceGraphic" />
                        </feMerge>
                    </filter>
                </defs>
                { for KG_EDGES.iter().map(|&(a, b)| {
                    let (na, nb) = (&nodes[a], &nodes[b]);
                    let active = if pmode { kg_on_path(&path, a, b) } else { focus.map_or(true, |h| a == h || b == h) };
                    let dim = if pmode { !active } else { focus.is_some() && !active };
                    let cls = if pmode && active { "kg-edge kg-path-edge" } else if dim { "kg-edge dim" } else { "kg-edge" };
                    html! { <line x1={kg_fmt(na.x)} y1={kg_fmt(na.y)} x2={kg_fmt(nb.x)} y2={kg_fmt(nb.y)} class={cls} /> }
                }) }
                { if !pmode { html! { <>
                    { for KG_EDGES.iter().enumerate().map(|(k, &(a, b))| {
                        let active = focus.map_or(true, |h| a == h || b == h);
                        if focus.is_some() && !active { return html! {}; }
                        let (surge, src, dst) = match focus {
                            Some(h) if a == h => (true, a, b),
                            Some(h) if b == h => (true, b, a),
                            _ => (false, a, b),
                        };
                        let (ns, nd) = (&nodes[src], &nodes[dst]);
                        let speed = if surge { 1.4 } else { 0.4 };
                        let ph = (t * speed + k as f64 * 0.37).fract();
                        let px = ns.x + (nd.x - ns.x) * ph;
                        let py = ns.y + (nd.y - ns.y) * ph;
                        if surge {
                            let ph2 = (ph + 0.5).fract();
                            let px2 = ns.x + (nd.x - ns.x) * ph2;
                            let py2 = ns.y + (nd.y - ns.y) * ph2;
                            html! { <>
                                <circle cx={kg_fmt(px)} cy={kg_fmt(py)} r="2.4" class="kg-pulse kg-surge" />
                                <circle cx={kg_fmt(px2)} cy={kg_fmt(py2)} r="1.9" class="kg-pulse kg-surge" />
                            </> }
                        } else {
                            html! { <circle cx={kg_fmt(px)} cy={kg_fmt(py)} r="1.6" class="kg-pulse" /> }
                        }
                    }) }
                </> } } else { html! {} } }
                { if pmode && path.len() >= 2 {
                    let nseg = (path.len() - 1) as f64;
                    let u = (t * 0.9) % nseg;
                    let seg = u.floor() as usize;
                    let fr = u - seg as f64;
                    let (pa, pb) = (&nodes[path[seg]], &nodes[path[seg + 1]]);
                    let px = pa.x + (pb.x - pa.x) * fr;
                    let py = pa.y + (pb.y - pa.y) * fr;
                    html! { <circle cx={kg_fmt(px)} cy={kg_fmt(py)} r="3.2" class="kg-pulse kg-surge" /> }
                } else { html! {} } }
                { for KG_NODES.iter().enumerate().map(|(i, &(label, kind))| {
                    let nd = &nodes[i];
                    let active = if pmode { path.contains(&i) } else { focus.map_or(true, |h| h == i || kg_neighbor(h, i)) };
                    let ringed = Some(i) == sn || (pmode && path.contains(&i));
                    let dim = if pmode { !active } else { focus.is_some() && !active };
                    let mut cls = String::from("kg-node");
                    if kg_post_idx(i).is_some() { cls.push_str(" kg-clickable"); }
                    if ringed { cls.push_str(" kg-sel"); }
                    if dim { cls.push_str(" dim"); }
                    let r = kg_r(kind) + (kg_degree(i) as f64).sqrt() * 1.1;
                    html! {
                        <g class={cls}
                           onmouseover={ let h = hovered.clone(); Callback::from(move |_| h.set(Some(i))) }
                           onmouseout={ let h = hovered.clone(); Callback::from(move |_| h.set(None)) }
                           onmousedown={ let d = drag.clone(); let m = moved.clone(); Callback::from(move |e: web_sys::MouseEvent| { e.prevent_default(); *d.borrow_mut() = Some(i); *m.borrow_mut() = false; }) }
                           ontouchstart={ let d = drag.clone(); let m = moved.clone(); Callback::from(move |e: web_sys::TouchEvent| { e.prevent_default(); *d.borrow_mut() = Some(i); *m.borrow_mut() = false; }) }>
                            { if ringed { html! { <circle cx={kg_fmt(nd.x)} cy={kg_fmt(nd.y)} r={kg_fmt(r + 3.0)} class="kg-ring" /> } } else { html! {} } }
                            <circle cx={kg_fmt(nd.x)} cy={kg_fmt(nd.y)} r={kg_fmt(r)} class={kg_dom_cls(kg_domain(i))} />
                            <text x={kg_fmt(nd.x + r + 2.0)} y={kg_fmt(nd.y + 2.5)}>{ label }</text>
                        </g>
                    }
                }) }
            </svg>
            <div class="kg-legend">{ for (0..6).map(|d| html! {
                <span class="kg-leg"><span class={kg_dom_cls(d)}>{ "\u{25CF}" }</span>{ " " }{ DOMAIN_NAMES[d] }</span>
            }) }</div>
            { if let Some(s) = *tour {
                let (_, title, text) = TOUR[s];
                let n = TOUR.len();
                let close = { let t = tour.clone(); Callback::from(move |_: web_sys::MouseEvent| t.set(None)) };
                html! {
                    <div class="kg-tour">
                        <div class="kg-tour-hd">
                            { format!("guided tour \u{00B7} {}/{}", s + 1, n) }
                            <button class="kg-tour-x" onclick={close.clone()}>{ "\u{2715}" }</button>
                        </div>
                        <div class="kg-tour-title">{ title }</div>
                        <div class="kg-tour-text">{ text }</div>
                        <div class="kg-tour-nav">
                            { if s > 0 { let t = tour.clone(); let prev = Callback::from(move |_: web_sys::MouseEvent| t.set(Some(s - 1))); html! { <button class="kg-tour-b" onclick={prev}>{ "\u{2039} back" }</button> } } else { html! {} } }
                            { if s + 1 < n {
                                let t = tour.clone(); let next = Callback::from(move |_: web_sys::MouseEvent| t.set(Some(s + 1)));
                                html! { <button class="kg-tour-b" onclick={next}>{ "next \u{203A}" }</button> }
                            } else {
                                html! { <button class="kg-tour-b" onclick={close}>{ "done \u{2713}" }</button> }
                            } }
                        </div>
                    </div>
                }
            } else { html! {} } }
            { if let Some(i) = sn {
                let (label, kind) = KG_NODES[i];
                let desc = if Some(i) == kg_index("ai-feed") {
                    match *feed_count {
                        Some(n) => format!("Live AI / agentic / LLM news, auto-curated daily by the dark factory \u{2014} {} stories in the feed below \u{2193}", n),
                        None => kg_desc(i).to_string(),
                    }
                } else {
                    kg_desc(i).to_string()
                };
                html! {
                    <div class="kg-detail">
                        <div class="kg-d-head">
                            <span class="kg-d-title">{ label }</span>
                            <span class="kg-d-kind">{ kg_kind_name(kind) }</span>
                            <span class="kg-d-close" onclick={ let s = sel_node.clone(); Callback::from(move |_| s.set(None)) }>{ "\u{00D7}" }</span>
                        </div>
                        <div class="kg-d-desc">{ desc }</div>
                        <div class="kg-d-links">
                            <span class="kg-d-lbl">{ "links \u{00B7} " }</span>
                            { for (0..KG_NODES.len()).filter(|&j| kg_neighbor(i, j)).map(|j| {
                                html! { <span class="kg-chip" onclick={ let s = sel_node.clone(); Callback::from(move |_| s.set(Some(j))) }>{ KG_NODES[j].0 }</span> }
                            }) }
                        </div>
                        { if let Some(pi) = kg_post_idx(i) {
                            html! { <a class="kg-d-open" onclick={ let o = props.on_open.clone(); Callback::from(move |_| o.emit(pi)) }>{ "cat this post \u{2192}" }</a> }
                          } else { html! {} } }
                    </div>
                }
              } else { html! {} } }
        </div>
    }
}



#[derive(serde::Deserialize)]
struct NewsItem {
    title: String,
    #[serde(default)]
    date: String,
    #[serde(default)]
    tag: String,
    #[serde(default)]
    summary: String,
    #[serde(default)]
    source: String,
    #[serde(default)]
    url: String,
}

const NEWS_PER_PAGE: usize = 6;

/// Only allow http(s) links from the auto-curated feed (Yew does NOT escape href;
/// a javascript:/data: url would otherwise execute on click). Returns None for anything else.
fn safe_href(url: &str) -> Option<String> {
    let lower = url.trim().to_ascii_lowercase();
    if lower.starts_with("http://") || lower.starts_with("https://") {
        Some(url.trim().to_string())
    } else {
        None
    }
}

fn news_item(it: &NewsItem) -> Html {
    let href = safe_href(&it.url);
    html! {
        <li class="news-item">
            <div class="news-head">
                {
                    match &href {
                        Some(u) => html! { <a class="news-title" href={u.clone()} target="_blank" rel="noopener noreferrer">{ it.title.clone() }</a> },
                        None => html! { <span class="news-title">{ it.title.clone() }</span> },
                    }
                }
                { if !it.tag.is_empty() { html! { <span class="news-tag">{ format!("#{}", it.tag) }</span> } } else { html! {} } }
                { if !it.date.is_empty() { html! { <time class="news-date">{ it.date.clone() }</time> } } else { html! {} } }
            </div>
            { if !it.summary.is_empty() { html! { <p class="news-sum">{ it.summary.clone() }</p> } } else { html! {} } }
            {
                match (&href, it.source.is_empty()) {
                    (Some(u), false) => html! { <a class="news-src" href={u.clone()} target="_blank" rel="noopener noreferrer">{ format!("source: {} \u{2197}", it.source) }</a> },
                    _ => html! {},
                }
            }
        </li>
    }
}

#[function_component(NewsFeed)]
fn news_feed() -> Html {
    let (items, err) = use_polled_json::<Vec<NewsItem>>("/news.json", None);
    let page = use_state(|| 0usize);
    let filter = use_state(|| None::<String>);
    let body = match (&*items, *err) {
        (None, true) => html! { <div class="news-loading">{ "ai-feed offline \u{2014} couldn't load news.json" }</div> },
        (None, false) => html! { <div class="news-loading">{ "fetching the AI feed\u{2026}" }</div> },
        (Some(v), _) if v.is_empty() => html! { <div class="news-loading">{ "feed warming up \u{2014} the factory posts fresh AI / agentic / LLM stories here every day \u{1F5DE}\u{FE0F}" }</div> },
        (Some(v), _) => {
            let f = (*filter).clone();
            let mut tags: Vec<String> = Vec::new();
            for it in v.iter() {
                if !it.tag.is_empty() && !tags.contains(&it.tag) { tags.push(it.tag.clone()); }
            }
            let fv: Vec<&NewsItem> = v.iter().filter(|it| f.as_deref().map_or(true, |t| it.tag == t)).collect();
            let total = fv.len();
            let pages = ((total + NEWS_PER_PAGE - 1) / NEWS_PER_PAGE).max(1);
            let cur = (*page).min(pages - 1);
            let start = cur * NEWS_PER_PAGE;
            let end = (start + NEWS_PER_PAGE).min(total);
            let prev = { let p = page.clone(); Callback::from(move |_: web_sys::MouseEvent| { if *p > 0 { p.set(*p - 1); } }) };
            let next = { let p = page.clone(); Callback::from(move |_: web_sys::MouseEvent| { if *p + 1 < pages { p.set(*p + 1); } }) };
            let all_cls = if f.is_none() { "nf-chip active" } else { "nf-chip" };
            let all_click = { let fl = filter.clone(); let pg = page.clone(); Callback::from(move |_: web_sys::MouseEvent| { fl.set(None); pg.set(0); }) };
            html! { <>
                <div class="nf-filters">
                    <button class={all_cls} onclick={all_click}>{ "all" }</button>
                    { for tags.iter().map(|t| {
                        let cls = if f.as_deref() == Some(t.as_str()) { "nf-chip active" } else { "nf-chip" };
                        let onclick = { let fl = filter.clone(); let pg = page.clone(); let tt = t.clone(); Callback::from(move |_: web_sys::MouseEvent| { fl.set(Some(tt.clone())); pg.set(0); }) };
                        html! { <button class={cls} {onclick}>{ format!("#{}", t) }</button> }
                    }) }
                </div>
                <ul class="news-list" key={format!("p{}-{}", cur, f.as_deref().unwrap_or("all"))}>
                    { for fv[start..end].iter().map(|&it| news_item(it)) }
                </ul>
                { if pages > 1 { html! {
                    <div class="news-pager">
                        <button class="np-btn" disabled={cur == 0} onclick={prev}>{ "\u{25C0} prev" }</button>
                        <span class="np-info">{ format!("page {}/{} \u{00B7} {} stories", cur + 1, pages, total) }</span>
                        <button class="np-btn" disabled={cur + 1 >= pages} onclick={next}>{ "next \u{25B6}" }</button>
                    </div>
                } } else { html! {} } }
            </> }
        }
    };
    html! {
        <div class="news">
            <div class="nf-cmd">{ "$ tail ai-feed.log  \u{00B7} auto-curated daily by the dark factory \u{1F916}" }</div>
            { body }
            <a class="nf-rss" href="/rss.xml" target="_blank" rel="noopener">{ "\u{1F4E1} subscribe via RSS" }</a>
        </div>
    }
}

#[derive(serde::Deserialize, Clone, PartialEq)]
struct DreamLine {
    #[serde(default)]
    t: String,
    #[serde(default)]
    level: String,
    #[serde(default)]
    msg: String,
}

fn dream_line(l: &DreamLine) -> Html {
    let cls = match l.level.as_str() {
        "ok" => "dj-msg dj-ok",
        "warn" => "dj-msg dj-warn",
        "dream" => "dj-msg dj-dream",
        _ => "dj-msg dj-info",
    };
    let t = if l.t.is_empty() { "--:--" } else { l.t.as_str() };
    html! {
        <li class="dj-line">
            <span class="dj-t">{ format!("[{}]", t) }</span>
            <span class={cls}>{ l.msg.clone() }</span>
        </li>
    }
}

#[function_component(DreamJournal)]
fn dream_journal() -> Html {
    let (lines, err) = use_polled_json::<Vec<DreamLine>>("/dmesg.json", None);
    let body = match (&*lines, *err) {
        (None, true) => html! { <div class="dj-loading">{ "dream log offline \u{2014} dmesg.json unreachable" }</div> },
        (None, false) => html! { <div class="dj-loading">{ "reading the factory's dream log\u{2026}" }</div> },
        (Some(v), _) if v.is_empty() => html! { <div class="dj-loading">{ "the factory hasn't dreamt yet tonight" }</div> },
        (Some(v), _) => html! { <ul class="dj-list">{ for v.iter().map(dream_line) }</ul> },
    };
    html! {
        <div class="dj">
            <div class="dj-cmd">{ "$ dmesg | tail  \u{00B7} the factory dreams \u{1F319}" }</div>
            { body }
        </div>
    }
}

// --- DGX Spark live monitor (real GPU/mem snapshot from the cluster) ---
#[derive(serde::Deserialize, Clone, PartialEq)]
struct SparkGpu {
    #[serde(default)]
    name: String,
    #[serde(default)]
    util: i32,
    #[serde(default)]
    temp: Option<i32>,
    #[serde(default)]
    power: Option<i32>,
}
#[derive(serde::Deserialize, Clone, PartialEq)]
struct SparkNode {
    #[serde(default)]
    host: String,
    #[serde(default)]
    reachable: bool,
    #[serde(default)]
    load: String,
    #[serde(default)]
    gpus: Vec<SparkGpu>,
    #[serde(default)]
    mem_used_mb: i64,
    #[serde(default)]
    mem_total_mb: i64,
}
#[derive(serde::Deserialize, Clone, PartialEq)]
struct SparkData {
    #[serde(default)]
    captured: String,
    #[serde(default)]
    reachable: bool,
    #[serde(default)]
    nodes: Vec<SparkNode>,
}

fn spark_bar(pct: f64, width: usize) -> String {
    let f = ((pct / 100.0) * width as f64).round().clamp(0.0, width as f64) as usize;
    let mut s = String::with_capacity(width + 2);
    s.push('[');
    for _ in 0..f {
        s.push('\u{2588}');
    }
    for _ in f..width {
        s.push('\u{2591}');
    }
    s.push(']');
    s
}

fn spark_text(d: &SparkData) -> String {
    let when = if d.captured.len() >= 16 { &d.captured[11..16] } else { "--:--" };
    let mut s = format!("2-node GB10 cluster  \u{00B7}  snapshot {} UTC\n", when);
    for n in &d.nodes {
        let host = if n.host.is_empty() { "spark" } else { n.host.as_str() };
        let st = if n.reachable { "online" } else { "unreachable" };
        s.push('\n');
        s.push_str(&format!("{} \u{00B7} {}", host, st));
        if !n.load.is_empty() {
            s.push_str(&format!(" \u{00B7} load {}", n.load));
        }
        s.push('\n');
        for (i, g) in n.gpus.iter().enumerate() {
            let temp = g.temp.map(|t| format!("  {}\u{00B0}C", t)).unwrap_or_default();
            let power = g.power.map(|p| format!("  {}W", p)).unwrap_or_default();
            s.push_str(&format!("  gpu{}  {} {:>3}%{}{}\n", i, spark_bar(g.util as f64, 16), g.util, temp, power));
        }
        if n.mem_total_mb > 0 {
            let pct = n.mem_used_mb as f64 / n.mem_total_mb as f64 * 100.0;
            s.push_str(&format!(
                "  mem   {} {}/{} GB ({:.0}%)\n",
                spark_bar(pct, 16),
                n.mem_used_mb / 1024,
                n.mem_total_mb / 1024,
                pct
            ));
        }
    }
    s
}

const SPARK_POLL_MS: u32 = 30_000;

#[function_component(SparkMonitor)]
fn spark_monitor() -> Html {
    let (data, err) = use_polled_json::<SparkData>("/spark.json", Some(SPARK_POLL_MS));
    let body = match (&*data, *err) {
        (None, true) => html! { <div class="dj-loading">{ "dgx-spark monitor offline \u{2014} spark.json unreachable" }</div> },
        (None, false) => html! { <div class="dj-loading">{ "polling dgx-spark\u{2026}" }</div> },
        (Some(d), _) => html! { <pre class="ascii-face spark-face">{ spark_text(d) }</pre> },
    };
    html! {
        <div class="ascii-art">
            <div class="ascii-cmd">{ "$ watch -n5 nvidia-smi  \u{00B7}  both GB10 nodes \u{00B7} auto-refresh 30s" }</div>
            { body }
        </div>
    }
}

// --- router cost meter (on-device Spark vs cloud Opus, from brain.log) ---
#[derive(serde::Deserialize, Clone, PartialEq)]
struct RouterStats {
    #[serde(default)]
    spark: i64,
    #[serde(default)]
    cloud_hard: i64,
    #[serde(default)]
    cloud_fallback: i64,
    #[serde(default)]
    spark_today: i64,
    #[serde(default)]
    cloud_today: i64,
    #[serde(default)]
    avg_ms_spark: i64,
    #[serde(default)]
    avg_ms_cloud: i64,
    #[serde(default)]
    saved_usd_est: f64,
}

fn router_text(s: &RouterStats) -> String {
    let cloud = s.cloud_hard + s.cloud_fallback;
    let total = s.spark + cloud;
    let pct = if total > 0 { (s.spark as f64 / total as f64 * 100.0).round() as i64 } else { 0 };
    let fb = if s.cloud_fallback > 0 { format!(" + {} fallback", s.cloud_fallback) } else { String::new() };
    format!(
        "on-device  {} {:>3}%  {} easy on GB10 (free)\ncloud/opus {}       {} hard{}\nlatency    spark ~{:.1}s \u{00B7} cloud ~{:.1}s\ntoday      {} on-device \u{00B7} {} cloud   \u{00B7}   est. saved ~${:.2}",
        spark_bar(pct as f64, 16), pct, s.spark,
        spark_bar((100 - pct) as f64, 16), s.cloud_hard, fb,
        s.avg_ms_spark as f64 / 1000.0, s.avg_ms_cloud as f64 / 1000.0,
        s.spark_today, s.cloud_today, s.saved_usd_est
    )
}

#[function_component(RouterMeter)]
fn router_meter() -> Html {
    let (data, err) = use_polled_json::<RouterStats>("/router.json", None);
    let body = match (&*data, *err) {
        (None, true) => html! { <div class="dj-loading">{ "router meter offline \u{2014} router.json unreachable" }</div> },
        (None, false) => html! { <div class="dj-loading">{ "reading brain.log\u{2026}" }</div> },
        (Some(s), _) => html! { <pre class="ascii-face spark-face">{ router_text(s) }</pre> },
    };
    html! {
        <div class="ascii-art">
            <div class="ascii-cmd">{ "$ tail brain.log | routerstat  \u{00B7}  easy\u{2192}Spark  hard\u{2192}Opus" }</div>
            { body }
        </div>
    }
}

#[derive(serde::Deserialize)]
struct DeployInfo {
    current: String,
    updated: String,
}
#[derive(serde::Deserialize)]
struct LogEvent {
    t: String,
    kind: String,
    text: String,
    ok: bool,
}
#[derive(serde::Deserialize)]
struct Activity {
    events: Vec<LogEvent>,
}

// evt_cls lives in blog-logic (pure, 100% coverage-gated)

// factory pipeline stages: (label, x-center within the 644-wide viewBox)
const PIPE_STAGES: [(&str, f64); 6] = [
    ("task", 52.0),
    ("brain", 160.0),
    ("router", 268.0),
    ("gate", 376.0),
    ("wasm", 484.0),
    ("pages", 592.0),
];

// live GitHub Actions state -> real-time pipeline highlighting
#[derive(serde::Deserialize, Clone, PartialEq)]
struct CiRun {
    #[serde(default)]
    status: String, // queued | in_progress | completed
    #[serde(default)]
    conclusion: Option<String>, // success | failure | cancelled | null
}
#[derive(serde::Deserialize, Clone, PartialEq)]
struct CiRuns {
    #[serde(default)]
    workflow_runs: Vec<CiRun>,
}

#[function_component(PipelineViz)]
fn pipeline_viz() -> Html {
    let (data, _err) = use_polled_json::<DeployInfo>("/deploy.json", Some(20_000));
    let (act, _ae) = use_polled_json::<Activity>("/activity.json", Some(15_000));
    // poll the real CI state (rate-safe: 75s, and only while this tab is mounted)
    let (ci, _ce) = use_polled_json::<CiRuns>(
        "https://api.github.com/repos/raghunathnair1-rgb/raghunathnair1-rgb.github.io/actions/runs?per_page=1",
        Some(75_000));
    use_anim_tick(66);
    let t = js_sys::Date::now() / 1000.0;
    let phase = (t / 5.5).fract();
    // three build-jobs flowing down the line, staggered
    let packets: Vec<f64> = (0..3)
        .map(|k| 52.0 + 540.0 * (phase + k as f64 * 0.34).fract())
        .collect();
    let n = PIPE_STAGES.len();

    // real pipeline state from GitHub Actions
    let run = ci.as_ref().and_then(|c| c.workflow_runs.first());
    let running = run.map_or(false, |r| r.status != "completed");
    let failed = run.map_or(false, |r| r.conclusion.as_deref() == Some("failure"));
    // while a run is genuinely in flight, march the highlight through the stages in real time
    let sweep = ((t / 0.7) as usize) % n;
    let (pill_cls, pill_txt) = if failed {
        ("pipe-pill fail", "\u{25CF} build failed")
    } else if running {
        ("pipe-pill run", "\u{25CF} pipeline running\u{2026}")
    } else {
        ("pipe-pill ok", "\u{25CF} idle \u{00B7} last deploy live")
    };

    let headline = match &*data {
        Some(d) => html! { <>
            <div class="pipe-latest">{ "\u{25B8} shipped: " }<span class="pipe-cur">{ d.current.clone() }</span></div>
            <div class="pipe-updated">{ format!("factory clock \u{00B7} snapshot {}", d.updated) }</div>
        </> },
        None => html! { <div class="dj-loading">{ "connecting to the factory floor\u{2026}" }</div> },
    };
    let feed = match &*act {
        Some(a) => html! { <>
            <div class="pipe-loghead">{ "$ tail -f factory.log  \u{00B7}  live execution" }</div>
            <ul class="pipe-log">{ for a.events.iter().map(|e| html! {
                <li class="pipe-evt">
                    <span class="pipe-t">{ e.t.clone() }</span>
                    <span class={evt_cls(&e.kind)}>{ e.kind.clone() }</span>
                    { " " }{ e.text.clone() }
                    { if e.ok { html! { <span class="pipe-ok">{ " \u{2713}" }</span> } } else { html! { <span class="pipe-fail">{ " \u{2717}" }</span> } } }
                </li>
            }) }</ul>
        </> },
        None => html! {},
    };

    html! {
        <div class="pipe-wrap">
            <div class="ascii-cmd">{ "$ watch factory | pipeline  \u{00B7}  task \u{2192} brain \u{2192} router \u{2192} gate \u{2192} wasm \u{2192} pages" }</div>
            <div class={pill_cls}>{ pill_txt }</div>
            <svg class="pipe" viewBox="0 0 644 78" preserveAspectRatio="xMidYMid meet">
                { for (0..n-1).map(|i| {
                    let x1 = PIPE_STAGES[i].1 + 30.0;
                    let x2 = PIPE_STAGES[i+1].1 - 30.0;
                    html! { <line x1={kg_fmt(x1)} y1="40" x2={kg_fmt(x2)} y2="40" class="pipe-conn" /> }
                }) }
                { for PIPE_STAGES.iter().enumerate().map(|(i, &(label, cx))| {
                    let active = if running { i == sweep } else { packets.iter().any(|&x| (x - cx).abs() < 34.0) };
                    let cls = if failed && i == n - 1 { "pipe-box fail" }
                              else if active { "pipe-box active" } else { "pipe-box" };
                    let lx = cx - (label.len() as f64) * 3.5;   // approx-center the monospace label
                    html! { <>
                        <rect x={kg_fmt(cx - 30.0)} y="26" width="60" height="28" rx="6" class={cls} />
                        <text x={kg_fmt(lx)} y="44" class="pipe-label">{ label }</text>
                    </> }
                }) }
                { for packets.iter().map(|&x| html! { <circle cx={kg_fmt(x)} cy="40" r="5" class="pipe-packet" /> }) }
            </svg>
            { headline }
            { feed }
        </div>
    }
}

// --- sunrise/sunset arc (factory-local Amsterdam; proxied server-side to /sun.json so it
//     never depends on a per-visitor wttr.in call that rate-limits) ---
#[derive(serde::Deserialize)]
struct Astro {
    sunrise: String,
    sunset: String,
}
#[derive(serde::Deserialize)]
struct WDay {
    astronomy: Vec<Astro>,
}
#[derive(serde::Deserialize)]
struct WttrSun {
    weather: Vec<WDay>,
}

/// "06:14 AM" -> minutes since local midnight.
fn parse_clock(s: &str) -> Option<i32> {
    let s = s.trim();
    let (hm, ap) = s.split_once(' ')?;
    let (h, m) = hm.split_once(':')?;
    let mut h: i32 = h.trim().parse().ok()?;
    let m: i32 = m.trim().parse().ok()?;
    let ap = ap.to_ascii_uppercase();
    if ap.starts_with("PM") && h != 12 { h += 12; }
    if ap.starts_with("AM") && h == 12 { h = 0; }
    Some(h * 60 + m)
}

/// ASCII day-arc; the sun 'O' sits at `frac` (0=sunrise .. 1=sunset), or hidden at night (None).
fn sun_arc(frac: Option<f64>) -> String {
    const W: usize = 23;
    const H: usize = 6;
    let arc: Vec<usize> = (0..W)
        .map(|x| {
            let s = (std::f64::consts::PI * x as f64 / (W - 1) as f64).sin();
            ((H - 1) as f64 * (1.0 - s)).round() as usize
        })
        .collect();
    let sun_x = frac.map(|f| (f.clamp(0.0, 1.0) * (W - 1) as f64).round() as usize);
    let mut out = String::new();
    for row in 0..H {
        for x in 0..W {
            if sun_x == Some(x) && arc[x] == row {
                out.push('O');
            } else if arc[x] == row && x % 2 == 0 {
                out.push('.');
            } else {
                out.push(' ');
            }
        }
        out.push('\n');
    }
    for _ in 0..W {
        out.push('\u{2500}');
    }
    out
}

#[function_component(SunArc)]
fn sun_arc_widget() -> Html {
    let (wttr, err) = use_polled_json::<WttrSun>("/wx.json", None);
    let tick = use_state(|| 0u64);
    {
        let tick = tick.clone();
        use_effect_with((), move |_| {
            let iv = gloo_timers::callback::Interval::new(60_000, move || tick.set(0));
            move || drop(iv)
        });
    }
    let now = js_sys::Date::new_0();
    let now_min = now.get_hours() as i32 * 60 + now.get_minutes() as i32;
    let astro = wttr.as_ref().and_then(|w| w.weather.first()).and_then(|d| d.astronomy.first());
    let body = match (astro, *err) {
        (Some(a), _) => match (parse_clock(&a.sunrise), parse_clock(&a.sunset)) {
            (Some(sr), Some(ss)) if ss > sr => {
                let daytime = now_min >= sr && now_min <= ss;
                let frac = if daytime { Some((now_min - sr) as f64 / (ss - sr) as f64) } else { None };
                let (pname, pcls) = if now_min < sr - 45 || now_min > ss + 45 {
                    ("night", "ascii-face sun-face sun-night")
                } else if now_min <= sr + 45 {
                    ("dawn", "ascii-face sun-face sun-dawn")
                } else if now_min >= ss - 45 {
                    ("dusk", "ascii-face sun-face sun-dusk")
                } else {
                    ("daytime", "ascii-face sun-face sun-day")
                };
                let (clabel, cmin) = if now_min < sr {
                    ("to sunrise", sr - now_min)
                } else if now_min <= ss {
                    ("to sunset", ss - now_min)
                } else {
                    ("to sunrise", 1440 - now_min + sr)
                };
                let pct = ((now_min - sr).max(0).min(ss - sr)) * 100 / (ss - sr);
                let line2 = if daytime {
                    format!("{} \u{00B7} {}% through \u{00B7} {}h{:02}m to sunset", pname, pct, (ss - now_min) / 60, (ss - now_min) % 60)
                } else {
                    format!("{} \u{00B7} {}h{:02}m {}", pname, cmin / 60, cmin % 60, clabel)
                };
                html! { <>
                    <pre class={pcls}>{ sun_arc(frac) }</pre>
                    <div class="sun-info">
                        <div>{ format!("\u{2191} {}   \u{2193} {}   \u{2600} {} of daylight", a.sunrise.trim(), a.sunset.trim(), day_length_hm(sr, ss)) }</div>
                        <div>{ line2 }</div>
                    </div>
                </> }
            }
            _ => html! { <div class="sun-info">{ "sun times unavailable" }</div> },
        },
        (None, true) => html! { <div class="sun-info">{ "sun offline: wx.json unreachable" }</div> },
        (None, false) => html! { <div class="sun-info">{ "locating the sun\u{2026}" }</div> },
    };
    html! {
        <div class="ascii-art sun-wrap">
            <div class="ascii-cmd">{ "$ sun --arc  \u{00B7} your local sky, right now" }</div>
            { body }
        </div>
    }
}

/// Initial TTY console from the URL hash (#posts/#lab/#factory/#feed) — so shared links deep-link.
fn initial_tab() -> usize {
    web_sys::window()
        .and_then(|w| w.location().hash().ok())
        .map(|h| match h.trim_start_matches('#') {
            "posts" => 1,
            "factory" => 3,
            "feed" => 4,
            "pipeline" => 5,
            "contact" => 6,
            _ => 0,
        })
        .unwrap_or(0)
}

/// Reverse of initial_tab: tab index -> URL hash name (so refresh/share restores the tab).
fn tab_hash(i: usize) -> &'static str {
    match i {
        1 => "posts",
        3 => "factory",
        4 => "feed",
        5 => "pipeline",
        6 => "contact",
        _ => "home",
    }
}

// --- self-healing watchdog: live site + pipeline vitals from watchdog.json ---
#[derive(serde::Deserialize, Clone, PartialEq)]
struct WdCheck {
    #[serde(default)]
    name: String,
    #[serde(default)]
    ok: bool,
    #[serde(default)]
    detail: String,
}
#[derive(serde::Deserialize, Clone, PartialEq)]
struct WatchdogData {
    #[serde(default)]
    status: String,
    #[serde(default)]
    summary: String,
    #[serde(default)]
    updated: String,
    #[serde(default)]
    checks: Vec<WdCheck>,
}

#[function_component(WatchdogStatus)]
fn watchdog_status() -> Html {
    let (data, err) = use_polled_json::<WatchdogData>("/watchdog.json", Some(60_000));
    let body = match (&*data, *err) {
        (None, true) => html! { <div class="dj-loading">{ "watchdog offline \u{2014} watchdog.json unreachable" }</div> },
        (None, false) => html! { <div class="dj-loading">{ "reading vitals\u{2026}" }</div> },
        (Some(d), _) => {
            let head_cls = match d.status.as_str() {
                "green" => "wd-head wd-ok",
                "down" => "wd-head wd-down",
                _ => "wd-head wd-warn",
            };
            let label = match d.status.as_str() {
                "green" => "all systems nominal",
                "down" => "DOWN \u{2014} self-heal armed",
                "degraded" => "degraded",
                _ => "unknown",
            };
            html! { <>
                <div class={head_cls}>
                    <span class="wd-dot"></span>
                    <span class="wd-status">{ label }</span>
                    <span class="wd-summary">{ format!("\u{00B7} {}", d.summary) }</span>
                </div>
                <ul class="wd-checks">
                    { for d.checks.iter().map(|c| html! {
                        <li class={ if c.ok { "wd-check ok" } else { "wd-check bad" } }>
                            <span class="wd-mark">{ if c.ok { "\u{2713}" } else { "\u{2717}" } }</span>
                            <span class="wd-name">{ c.name.clone() }</span>
                            <span class="wd-detail">{ c.detail.clone() }</span>
                        </li>
                    }) }
                </ul>
                <div class="wd-foot">{ format!("last self-check {} \u{00B7} probes every 15 min \u{00B7} rolls back to last-green on breach", d.updated) }</div>
            </> }
        }
    };
    html! {
        <div class="ascii-art wd-box">
            <div class="ascii-cmd">{ "$ ./watchdog.py \u{00B7} live site + pipeline health \u{00B7} self-healing" }</div>
            { body }
        </div>
    }
}

// --- coverage gate badge: blog-logic line coverage (CI-enforced floor) ---
#[derive(serde::Deserialize, Clone, PartialEq)]
struct CoverageData {
    #[serde(default)]
    pct: u32,
    #[serde(default)]
    threshold: u32,
    #[serde(default, rename = "crate")]
    name: String,
    #[serde(default)]
    functions: u32,
    #[serde(default)]
    tests: u32,
}

#[function_component(CoverageBadge)]
fn coverage_badge() -> Html {
    let (data, err) = use_polled_json::<CoverageData>("/coverage.json", Some(120_000));
    let body = match (&*data, *err) {
        (None, true) => html! { <div class="dj-loading">{ "coverage unavailable \u{2014} coverage.json unreachable" }</div> },
        (None, false) => html! { <div class="dj-loading">{ "reading coverage\u{2026}" }</div> },
        (Some(d), _) => {
            let pct = d.pct.min(100);
            html! { <>
                <div class="cov-row">
                    <span class="cov-pct">{ format!("{}%", pct) }</span>
                    <span class="cov-bar"><span class="cov-fill" style={format!("width:{}%", pct)}></span></span>
                </div>
                <div class="cov-meta">{ format!("{} \u{00B7} {} pure fns \u{00B7} {} tests \u{00B7} gate blocks deploy below {}%", d.name, d.functions, d.tests, d.threshold) }</div>
            </> }
        }
    };
    html! {
        <div class="ascii-art">
            <div class="ascii-cmd">{ "$ cargo llvm-cov -p blog-logic \u{00B7} CI-enforced coverage gate" }</div>
            { body }
        </div>
    }
}

// --- the Idea Engine's self-authored backlog (scored impact × ease ÷ risk) ---
#[derive(serde::Deserialize, Clone, PartialEq)]
struct Idea {
    #[serde(default)]
    title: String,
    #[serde(default)]
    why: String,
    #[serde(default)]
    impact: u32,
    #[serde(default)]
    ease: u32,
    #[serde(default)]
    risk: u32,
    #[serde(default)]
    score: f64,
    #[serde(default)]
    auto: bool,
    #[serde(default)]
    big: bool,
}
#[derive(serde::Deserialize, Clone, PartialEq)]
struct IdeaData {
    #[serde(default)]
    ideas: Vec<Idea>,
}

#[function_component(IdeaBacklog)]
fn idea_backlog() -> Html {
    let (data, err) = use_polled_json::<IdeaData>("/ideas.json", Some(300_000));
    // Hide the whole panel until the idea engine has proposed something (empty/reset = nothing).
    // It reappears on its own when a scheduled 04:07 run writes a fresh backlog to ideas.json.
    if !matches!(&*data, Some(d) if !d.ideas.is_empty()) {
        return html! {};
    }
    let body = match (&*data, *err) {
        (None, true) => html! { <div class="dj-loading">{ "backlog unavailable \u{2014} ideas.json unreachable" }</div> },
        (None, false) => html! { <div class="dj-loading">{ "the factory is thinking\u{2026}" }</div> },
        (Some(d), _) => html! {
            <ul class="idea-list">
                { for d.ideas.iter().map(|it| html! {
                    <li class="idea">
                        <div class="idea-head">
                            <span class="idea-score">{ format!("{:.1}", it.score) }</span>
                            <span class="idea-title">{ it.title.clone() }</span>
                            { if it.big { html! { <span class="idea-big">{ "\u{1F680} big" }</span> } } else if it.auto { html! { <span class="idea-auto">{ "\u{1F916} self-build" }</span> } } else { html! {} } }
                        </div>
                        <div class="idea-why">{ it.why.clone() }</div>
                        <div class="idea-metrics">
                            <span>{ format!("impact {}", it.impact) }</span>
                            <span>{ format!("ease {}", it.ease) }</span>
                            <span>{ format!("risk {}", it.risk) }</span>
                        </div>
                    </li>
                }) }
            </ul>
        },
    };
    html! {
        <div class="ascii-art">
            <div class="ascii-cmd">{ "$ ./idea_engine.py \u{00B7} the factory's self-authored backlog (impact \u{00D7} ease \u{00F7} risk)" }</div>
            { body }
        </div>
    }
}


// --- WebGL neural-brain (the GPU shader lives in index.html; this mounts its canvas + HUD) ---
#[derive(Properties, PartialEq)]
struct BrainGlProps {
    #[prop_or_default]
    hero: bool,
}
// live brain telemetry (brain_metrics.py → brain.json): knowledge climbs, hallucination rate stays low
#[derive(serde::Deserialize, Clone, PartialEq)]
struct BrainMetrics {
    #[serde(default)]
    knowledge: f64,
    #[serde(default)]
    facts: u64,
    #[serde(default)]
    delta: u64,
    #[serde(default)]
    nodes: u32,
    #[serde(default)]
    synapses: u32,
    #[serde(default)]
    hallu_pct: f64,
    #[serde(default)]
    caught: u64,
    #[serde(default)]
    coverage: u32,
}

#[function_component(BrainGl)]
fn brain_gl(props: &BrainGlProps) -> Html {
    let stage = if props.hero { "brain-gl-stage hero" } else { "brain-gl-stage" };
    let (data, _err) = use_polled_json::<BrainMetrics>("/brain.json", Some(30_000));
    let (krow, lrow, hrow) = match &*data {
        Some(d) => {
            let nodes = if d.nodes > 0 { d.nodes as usize } else { KG_NODES.len() };
            let syn = if d.synapses > 0 { d.synapses as usize } else { KG_EDGES.len() };
            let kn = (d.knowledge * 100.0).round().clamp(0.0, 100.0) as u32;
            let delta = if d.delta > 0 { format!(" \u{00B7} +{} \u{2191}", d.delta) } else { String::new() };
            (
                format!("knowledge \u{00B7} {} facts \u{00B7} {} nodes \u{00B7} {} synapses{}", d.facts, nodes, syn, delta),
                format!("learning \u{00B7} {}% mastery \u{00B7} {}% tested \u{00B7} climbing", kn, d.coverage.min(100)),
                format!("hallucinations \u{00B7} {:.1}% \u{00B7} {} caught & gated", d.hallu_pct, d.caught),
            )
        }
        None => (
            format!("knowledge \u{00B7} {} nodes \u{00B7} {} synapses", KG_NODES.len(), KG_EDGES.len()),
            "learning \u{00B7} 100% tested \u{00B7} always-on".to_string(),
            "hallucinations \u{00B7} gated \u{00B7} red = flagged & caught".to_string(),
        ),
    };
    html! {
        <div class="brain-gl-wrap">
            <div class="ascii-cmd">{ "$ ./brain --render \u{00B7} drag to rotate \u{00B7} knowledge \u{00B7} learning \u{00B7} hallucinations" }</div>
            <div class={stage}>
                <canvas id="brain-gl" class="brain-gl"></canvas>
                <div class="brain-gl-hud">
                    <div class="bgl-row"><span class="bgl-dot bgl-k"></span>{ krow }</div>
                    <div class="bgl-row"><span class="bgl-dot bgl-l"></span>{ lrow }</div>
                    <div class="bgl-row"><span class="bgl-dot bgl-h"></span>{ hrow }</div>
                </div>
            </div>
        </div>
    }
}

#[function_component(SiteFooter)]
fn site_footer() -> Html {
    // uptime since first ship (2026-07-06 UTC); auto-increments, no server needed
    let days = {
        let now = js_sys::Date::now();
        (((now - 1783296000000.0_f64) / 86_400_000.0).floor() as i64).max(0)
    };
    let year = js_sys::Date::new_0().get_full_year();
    html! {
        <footer class="site-footer">
            <div class="foot-rule">{ "\u{2500}\u{2500}\u{2500}\u{2500}\u{2500} EOF \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}" }</div>
            <div class="foot-line"><span class="fp">{ "$" }</span>{ " uptime" }</div>
            <div class="foot-out">{ format!("dark-factory \u{00B7} up {days}d \u{00B7} brain \u{1F9E0} online \u{00B7} 0 downtime tolerated") }</div>
            <div class="foot-line"><span class="fp">{ "$" }</span>{ " cat stack.txt" }</div>
            <div class="foot-out foot-stack">
                <span>{ "rust" }</span><span>{ "wasm" }</span><span>{ "yew" }</span><span>{ "trunk" }</span><span>{ "gh-actions" }</span><span>{ "pages" }</span>
            </div>
            <div class="foot-sign">{ format!("/* \u{00A9} {year} raghu nair \u{2014} built in Rust \u{2192} WebAssembly, rendered by Yew, no React harmed */") }</div>
            <div class="foot-fine">{ "cookieless analytics \u{00B7} no ad-trackers \u{00B7} shipped by an AI harness brain while I sleep" }</div>
            <div class="foot-line"><span class="fp">{ "$" }</span>{ " " }<span class="foot-cur"></span></div>
        </footer>
    }
}

#[function_component(App)]
fn app() -> Html {
    let selected = use_state(|| None::<usize>);
    let path_hl = use_state(|| Vec::<usize>::new());
    let tab = use_state(initial_tab); // TTY console: 0=~/ 1=~/posts 2=~/lab 3=~/factory 4=~/feed
    // keep the URL hash in sync with the active tab, so a refresh (or shared link) restores it
    use_effect_with(*tab, |t| {
        if let Some(w) = web_sys::window() {
            let _ = w.location().set_hash(tab_hash(*t));
        }
        || ()
    });
    let list = posts();

    let view = match *selected {
        Some(i) => {
            let p = &list[i];
            let back = {
                let s = selected.clone();
                Callback::from(move |_: web_sys::MouseEvent| s.set(None))
            };
            let keyback = {
                let s = selected.clone();
                Callback::from(move |e: web_sys::KeyboardEvent| {
                    if e.key() == "Enter" || e.key() == " " {
                        e.prevent_default();
                        s.set(None);
                    }
                })
            };
            html! {
                <article>
                    <a class="back" onclick={back} onkeydown={keyback} tabindex="0" role="button">{"‹ back to log"}</a>
                    <h2>{ p.title }</h2>
                    <div class="meta"><span class="tag">{ format!("#{}", p.tag) }</span>{ " · " }<time>{ p.date }</time></div>
                    { for p.body.split("\n\n").map(|para| html! { <p>{ para }</p> }) }
                </article>
            }
        }
        None => {
            let tt = *tab;
            let content = match tt {
                1 => html! { <>
                    <div class="cmd">{ "$ ls ~/posts" }</div>
                    <ul class="log">
                        { for list.iter().enumerate().map(|(i, p)| {
                            let open = { let s = selected.clone(); Callback::from(move |_: web_sys::MouseEvent| s.set(Some(i))) };
                            let keyopen = { let s = selected.clone(); Callback::from(move |e: web_sys::KeyboardEvent| {
                                if e.key() == "Enter" || e.key() == " " { e.prevent_default(); s.set(Some(i)); }
                            }) };
                            html! {
                                <li onclick={open} onkeydown={keyopen} tabindex="0" role="button">
                                    <span class="prompt">{ "›" }</span>
                                    <span class="title">{ p.title }</span>
                                    <span class="tag">{ format!("#{}", p.tag) }</span>
                                    <time>{ p.date }</time>
                                    <span class="rt">{ format!("{} min", (p.body.split_whitespace().count() / 200).max(1)) }</span>
                                    <div class="post-excerpt">{ format!("{}\u{2026}", p.body.chars().take(96).collect::<String>()) }</div>
                                </li>
                            }
                        }) }
                    </ul>
                </> },
                3 => html! { <>
                    <div class="cmd">{ "$ systemctl status dark-factory  \u{00B7} the machine's own vitals" }</div>
                    <WatchdogStatus />
                    <CoverageBadge />
                    <IdeaBacklog />
                    <DreamJournal />
                    <SparkMonitor />
                    <RouterMeter />
                </> },
                4 => html! { <NewsFeed /> },
                5 => html! { <>
                    <div class="cmd">{ "$ systemctl status dark-factory.pipeline \u{00B7} live" }</div>
                    <PipelineViz />
                </> },
                6 => html! { <>
                    <div class="cmd">{ "$ ./reach-out.sh --now" }</div>
                    <p class="contact-lede"><span class="cc-c">{ "// " }</span>
                        { "Looking for a developer, have a question, or just want to connect? Ping any channel below \u{2014} I actually read these." }</p>
                    <div class="contact-grid">
                        <a class="contact-card cc-gh" href="https://github.com/raghunathnair1-rgb" target="_blank" rel="noopener noreferrer">
                            <span class="cc-ico"><svg viewBox="0 0 24 24" aria-hidden="true"><path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"/></svg></span>
                            <span class="cc-meta"><span class="cc-plat">{ "GitHub" }</span><span class="cc-handle">{ "@raghunathnair1-rgb" }</span></span>
                            <span class="cc-go">{ "\u{2197}" }</span>
                        </a>
                        <a class="contact-card cc-li" href="https://www.linkedin.com/in/rgnair" target="_blank" rel="noopener noreferrer">
                            <span class="cc-ico"><svg viewBox="0 0 24 24" aria-hidden="true"><path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.225 0z"/></svg></span>
                            <span class="cc-meta"><span class="cc-plat">{ "LinkedIn" }</span><span class="cc-handle">{ "in/rgnair" }</span></span>
                            <span class="cc-go">{ "\u{2197}" }</span>
                        </a>
                        <a class="contact-card cc-ig" href="https://instagram.com/codex_anonymous" target="_blank" rel="noopener noreferrer">
                            <span class="cc-ico"><svg viewBox="0 0 24 24" aria-hidden="true"><path d="M12 2.163c3.204 0 3.584.012 4.85.07 3.252.148 4.771 1.691 4.919 4.919.058 1.265.069 1.645.069 4.849 0 3.205-.012 3.584-.069 4.849-.149 3.225-1.664 4.771-4.919 4.919-1.266.058-1.644.07-4.85.07-3.204 0-3.584-.012-4.849-.07-3.26-.149-4.771-1.699-4.919-4.92-.058-1.265-.07-1.644-.07-4.849 0-3.204.013-3.583.07-4.849.149-3.227 1.664-4.771 4.919-4.919 1.266-.057 1.645-.069 4.849-.069zM12 0C8.741 0 8.333.014 7.053.072 2.695.272.273 2.69.073 7.052.014 8.333 0 8.741 0 12c0 3.259.014 3.668.072 4.948.2 4.358 2.618 6.78 6.98 6.98C8.333 23.986 8.741 24 12 24c3.259 0 3.668-.014 4.948-.072 4.354-.2 6.782-2.618 6.979-6.98.059-1.28.073-1.689.073-4.948 0-3.259-.014-3.667-.072-4.947-.196-4.354-2.617-6.78-6.979-6.98C15.668.014 15.259 0 12 0zm0 5.838a6.162 6.162 0 100 12.324 6.162 6.162 0 000-12.324zM12 16a4 4 0 110-8 4 4 0 010 8zm6.406-11.845a1.44 1.44 0 100 2.881 1.44 1.44 0 000-2.881z"/></svg></span>
                            <span class="cc-meta"><span class="cc-plat">{ "Instagram" }</span><span class="cc-handle">{ "@codex_anonymous" }</span></span>
                            <span class="cc-go">{ "\u{2197}" }</span>
                        </a>
                        <a class="contact-card cc-mail" href="mailto:raghunathnair1@gmail.com">
                            <span class="cc-ico"><svg viewBox="0 0 24 24" aria-hidden="true"><path d="M22 4H2C.9 4 0 4.9 0 6v12c0 1.1.9 2 2 2h20c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-10 5L2 8V6l10 5 10-5v2z"/></svg></span>
                            <span class="cc-meta"><span class="cc-plat">{ "Email" }</span><span class="cc-handle">{ "raghunathnair1@gmail.com" }</span></span>
                            <span class="cc-go">{ "\u{2197}" }</span>
                        </a>
                    </div>
                    <div class="contact-foot"><span class="cc-prompt">{ "raghu@dark-factory" }</span>{ ":~$ " }<span class="cc-typed">{ "./connect" }</span><span class="cc-cursor"></span></div>
                </> },
                _ => html! { <>
                    <RustBadge />
                    <BrainGl hero=true />
                    <section class="about">
                        <div class="cmd">{ "$ whoami" }</div>
                        <div class="card">
                            <div class="avatar-wrap">
                                <img class="avatar" src="/assets/raghu.jpg" alt="Raghu Nair"/>
                            </div>
                            <div class="bio">
                                <div class="line"><span class="key">{ "user " }</span>{ "raghu nair" }</div>
                                <div class="line"><span class="key">{ "role " }</span>{ "builder · tinkerer · runs an AI dark factory for fun" }</div>
                                <div class="line"><span class="key">{ "stack" }</span>{ " rust · wasm · llms · an unreasonable amount of automation" }</div>
                                <div class="line"><span class="key">{ "stat " }</span>{ "brain \u{1F9E0} online \u{00B7} hover the pic to declassify" }</div>
                            </div>
                        </div>
                    </section>
                    <div class="np">
                        <div class="np-cmd">{ "$ now-playing" }</div>
                        <div class="np-out">
                            <span class="np-note">{ "\u{266B}" }</span>
                            <span class="eq"><i></i><i></i><i></i><i></i></span>
                            <span class="np-track">{ "Cornfield Chase" }</span>
                            <span class="np-artist">{ "\u{00B7} Hans Zimmer \u{2014} Interstellar (OST)" }</span>
                            <a href="https://music.apple.com/nl/album/cornfield-chase/1533983552?i=1533984393" target="_blank" rel="noopener">{ "[listen \u{2197}]" }</a>
                        </div>
                    </div>
                    <div class="nf-cmd">{ "$ neofetch" }</div>
                    <div class="neofetch">
                        <pre class="nf-art">{ "   ╷ ╷ ╷\n  ┌┴─┴─┴┐\n  │ ▓▓▓ │\n  │dark-f│\n  └─────┘" }</pre>
                        <div class="nf-info">
                            <div class="nf-line"><span class="k">{ "os" }</span>{ "dark-factory (lights-out)" }</div>
                            <div class="nf-line"><span class="k">{ "host" }</span>{ "raghunathnair1-rgb.github.io" }</div>
                            <div class="nf-line"><span class="k">{ "kernel" }</span>{ "rust \u{2192} wasm (yew + trunk)" }</div>
                            <div class="nf-line"><span class="k">{ "shell" }</span>{ "the harness brain" }</div>
                            <div class="nf-line"><span class="k">{ "gates" }</span>{ "security \u{00B7} qa \u{00B7} sast \u{00B7} ontology" }</div>
                            <div class="nf-line"><span class="k">{ "uptime" }</span>{ "shipping since 2026-07-06" }</div>
                            <div class="nf-line"><span class="k">{ "fuel" }</span>{ "\u{2615} coffee \u{00B7} \u{221E} cups" }</div>
                            <div class="nf-line"><span class="k">{ "pet" }</span>{ "Maine Coon \u{1F408} (loaf mode)" }</div>
                            <div class="nf-line"><span class="k">{ "status" }</span><span class="nf-ok">{ "\u{25CF} online" }</span></div>
                        </div>
                    </div>
                    <div class="fortune">
                        <div class="nf-cmd">{ "$ fortune" }</div>
                        <blockquote>{ "\u{201C}Do not go gentle into that good night; rage, rage against the dying of the light.\u{201D} \u{2014} Interstellar" }</blockquote>
                    </div>
                    <WeatherCard />
                    <SunArc />
                    <AsciiClock />
                    <BrainCard />
                    <KnowledgeGraph on_open={ let s = selected.clone(); Callback::from(move |i: usize| s.set(Some(i))) } path={(*path_hl).clone()} />
                </> },
            };
            let items = [("~/", 0usize), ("~/posts", 1usize), ("~/factory", 3usize), ("~/feed", 4usize), ("~/pipeline", 5usize), ("~/contact", 6usize)];
            html! {
                <>
                <nav class="tty-bar">
                    { for items.iter().enumerate().map(|(pos, (label, idx))| {
                        let idx = *idx;
                        let t = tab.clone();
                        let onclick = Callback::from(move |_: web_sys::MouseEvent| t.set(idx));
                        let cls = if tt == idx { "tty-tab active" } else { "tty-tab" };
                        html! { <button class={cls} {onclick}>{ format!("[{}] {}", pos + 1, label) }</button> }
                    }) }
                </nav>
                <div class="console" key={tt.to_string()}>{ content }</div>
                <Terminal
                    on_path={ let p = path_hl.clone(); Callback::from(move |pv: Vec<usize>| p.set(pv)) }
                    on_cd={ let t = tab.clone(); Callback::from(move |i: usize| t.set(i)) } />
                </>
            }
        },
    };

    html! {
        <>
            <header>
                <div class="logo"><span class="logo-name">{ "raghu" }</span><span class="cursor">{ "\u{2588}" }</span></div>
                <p class="boot">{ "// dark-factory online · brain healthy · shipping from wasm" }</p>
            </header>
            <main>{ view }</main>
            <SiteFooter />
        </>
    }
}

fn main() {
    // Mount into #app so the matrix-rain <canvas> stays a separate layer Yew
    // never manages. Fall back to body-mount so the app can never go blank.
    match gloo_utils::document().get_element_by_id("app") {
        Some(root) => yew::Renderer::<App>::with_root(root).render(),
        None => yew::Renderer::<App>::new().render(),
    };
}
