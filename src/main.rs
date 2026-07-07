use yew::prelude::*;
use yew::TargetCast;

#[derive(Clone)]
enum Line {
    Cmd(String),
    Out(String),
}

const MAINE_COON: &str = "      |\\      _,,,---,,_\n     /,`.-'`'    -.  ;-;;,_\n    |,4-  ) )-,_..;\\ (  `'-'\n   '---''(_/--'  `-'\\_)";

fn run_command(cmd: &str) -> String {
    let p: Vec<&str> = cmd.split_whitespace().collect();
    match p.as_slice() {
        ["help"] => "commands: help  whoami  ls  cat <post>  meow  neofetch  dmesg  moon  doomfire  warp  spark  now-playing  coffee  brew  fortune  theme <name>  crt  path <a> <b>  reboot  uptime  echo <x>  clear".to_string(),
        ["reboot"] => "rebooting the dark factory\u{2026}".to_string(),
        ["crt"] | ["crt", "on"] | ["crt", "off"] => "CRT mode \u{1F4FA} toggled".to_string(),
        ["whoami"] => "raghu \u{2014} builder \u{00B7} tinkerer \u{00B7} runs an AI dark factory for fun".to_string(),
        ["ls"] => "about.md   now-playing   neofetch   posts/   linkedin   github".to_string(),
        ["ls", "posts"] | ["ls", "posts/"] => "hello-world.md   anatomy-of-a-dark-factory.md   why-webassembly.md".to_string(),
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
}

#[function_component(Terminal)]
fn terminal(props: &TermProps) -> Html {
    let history = use_state(|| vec![Line::Out("dark-factory shell \u{2014} type 'help' for commands.".to_string())]);
    let value = use_state(String::new);
    let hist_idx = use_state(|| None::<usize>);

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
                let mut h = (*history).clone();
                h.push(Line::Cmd(cmd.clone()));
                h.push(Line::Out(run_command(&cmd)));
                history.set(h);
            }
        })
    };

    html! {
        <section class="term">
            <div class="t-bar"><span class="d r"></span><span class="d y"></span><span class="d g"></span><span class="t-name">{ "visitor@dark-factory \u{2014} try it \u{2193}" }</span></div>
            <div class="t-body">
                { for (*history).iter().map(|l| match l {
                    Line::Cmd(s) => html! { <div class="t-line"><span class="t-prompt">{ "$ " }</span>{ s.clone() }</div> },
                    Line::Out(s) => html! { <pre class="t-out">{ s.clone() }</pre> },
                }) }
                <div class="t-inputline">
                    <span class="t-prompt">{ "$ " }</span>
                    <input class="t-input" type="text" value={(*value).clone()} {oninput} {onkeydown}
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
                let msg = match gloo_net::http::Request::get("https://wttr.in/?format=j1").send().await {
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
                        Err(_) => "weather offline \u{00B7} couldn't parse wttr.in".to_string(),
                    },
                    Err(_) => "weather offline \u{00B7} couldn't reach wttr.in".to_string(),
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
    let st = use_state(|| None::<BrainStatus>);
    let err = use_state(|| false);
    let tick = use_state(|| 0u64);
    {
        let st = st.clone();
        let err = err.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match gloo_net::http::Request::get("/status.json").send().await {
                    Ok(resp) => match resp.json::<BrainStatus>().await {
                        Ok(b) => st.set(Some(b)),
                        Err(_) => err.set(true),
                    },
                    Err(_) => err.set(true),
                }
            });
            || ()
        });
    }
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
fn use_anim_tick(ms: u32) {
    let f = use_state(|| 0u64);
    use_effect_with((), move |_| {
        let iv = if reduced_motion() {
            None
        } else {
            Some(gloo_timers::callback::Interval::new(ms, move || f.set(0)))
        };
        move || drop(iv)
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

#[function_component(DoomFire)]
fn doom_fire() -> Html {
    use_anim_tick(60);
    let fire = use_mut_ref(|| Fire::new(60, 20));
    let frame = {
        let mut f = fire.borrow_mut();
        f.step();
        f.render()
    };
    html! {
        <div class="ascii-art">
            <div class="ascii-cmd">{ "$ ./doomfire" }</div>
            <pre class="ascii-face doom-fire">{ frame }</pre>
        </div>
    }
}

// --- moon phase (computed from the current date) ---
fn moon_phase_frac(now_ms: f64) -> f64 {
    let synodic = 29.530_588_853 * 86_400_000.0;
    let diff = now_ms - 947_182_440_000.0; // 2000-01-06 18:14 UTC — a known new moon
    let mut p = (diff / synodic).fract();
    if p < 0.0 {
        p += 1.0;
    }
    p
}
fn moon_name(p: f64) -> &'static str {
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
fn moon_illum(p: f64) -> u32 {
    ((1.0 - (2.0 * std::f64::consts::PI * p).cos()) / 2.0 * 100.0).round() as u32
}
fn moon_art(p: f64) -> String {
    let c = (2.0 * std::f64::consts::PI * p).cos();
    let waxing = p <= 0.5;
    let (rows, cols) = (11i32, 22i32);
    let mut s = String::new();
    for ry in 0..rows {
        let ny = (ry as f64 - (rows as f64 - 1.0) / 2.0) / ((rows as f64 - 1.0) / 2.0);
        for rx in 0..cols {
            let nx = (rx as f64 - (cols as f64 - 1.0) / 2.0) / ((cols as f64 - 1.0) / 2.0);
            if nx * nx + ny * ny <= 1.0 {
                let lit = if waxing { nx >= c } else { nx <= -c };
                s.push(if lit { '@' } else { '.' });
            } else {
                s.push(' ');
            }
        }
        s.push('\n');
    }
    s
}

#[function_component(MoonPhase)]
fn moon_phase() -> Html {
    let p = moon_phase_frac(js_sys::Date::now());
    html! {
        <div class="ascii-art">
            <div class="ascii-cmd">{ "$ moon" }</div>
            <pre class="ascii-face moon-face">{ moon_art(p) }</pre>
            <div class="moon-info">{ format!("{} \u{00B7} {}% illuminated", moon_name(p), moon_illum(p)) }</div>
        </div>
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

#[function_component(Starfield)]
fn starfield() -> Html {
    use_anim_tick(50);
    let stars = use_mut_ref(|| Stars::new(150, 60, 22));
    let frame = {
        let mut s = stars.borrow_mut();
        s.step();
        s.render()
    };
    html! {
        <div class="ascii-art">
            <div class="ascii-cmd">{ "$ ./warp" }</div>
            <pre class="ascii-face star-face">{ frame }</pre>
        </div>
    }
}

// --- spinning ASCII donut (a1k0n donut.c, ported) ---
fn donut_frame(a: f64, b: f64) -> String {
    const W: usize = 44;
    const H: usize = 22;
    let k1 = W as f64 * 5.0 / 12.0;
    let chars = b".,-~:;=!*#$@";
    let mut out = [b' '; W * H];
    let mut zb = [0f64; W * H];
    let (ca, sa) = (a.cos(), a.sin());
    let (cb, sb) = (b.cos(), b.sin());
    let mut th = 0.0f64;
    while th < std::f64::consts::TAU {
        let (ct, st) = (th.cos(), th.sin());
        let mut ph = 0.0f64;
        while ph < std::f64::consts::TAU {
            let (cp, sp) = (ph.cos(), ph.sin());
            let cx = 2.0 + ct;
            let cy = st;
            let x = cx * (cb * cp + sa * sb * sp) - cy * ca * sb;
            let y = cx * (sb * cp - sa * cb * sp) + cy * ca * cb;
            let zz = 5.0 + ca * cx * sp + cy * sa;
            let ooz = 1.0 / zz;
            let xp = (W as f64 / 2.0 + k1 * ooz * x) as isize;
            let yp = (H as f64 / 2.0 - k1 * ooz * y * 0.5) as isize;
            let l = cp * ct * sb - ca * ct * sp - sa * st + cb * (ca * st - ct * sa * sp);
            if l > 0.0 && xp >= 0 && (xp as usize) < W && yp >= 0 && (yp as usize) < H {
                let idx = xp as usize + yp as usize * W;
                if ooz > zb[idx] {
                    zb[idx] = ooz;
                    out[idx] = chars[((l * 8.0) as usize).min(11)];
                }
            }
            ph += 0.02;
        }
        th += 0.07;
    }
    let mut s = String::with_capacity((W + 1) * H);
    for r in 0..H {
        for c in 0..W {
            s.push(out[r * W + c] as char);
        }
        s.push('\n');
    }
    s
}

#[function_component(SpinningDonut)]
fn spinning_donut() -> Html {
    use_anim_tick(50);
    let t = js_sys::Date::now() / 1000.0;
    html! {
        <div class="ascii-art">
            <div class="ascii-cmd">{ "$ ./donut" }</div>
            <pre class="ascii-face">{ donut_frame(t, t * 0.5) }</pre>
        </div>
    }
}

// --- solar-system orrery (concentric orbits + planets, distance-field rings) ---
fn orrery_frame(t: f64) -> String {
    const W: usize = 54;
    const H: usize = 27;
    let cx = (W as f64 - 1.0) / 2.0;
    let cy = (H as f64 - 1.0) / 2.0;
    let ax = 1.0f64;
    let ay = 0.5f64;
    // (orbit radius, symbol, angular speed)
    let planets: [(f64, u8, f64); 8] = [
        (4.5, b'o', 1.70),
        (7.5, b'O', 1.25),
        (10.5, b'o', 1.00),
        (13.5, b'O', 0.82),
        (17.5, b'#', 0.50),
        (21.0, b'%', 0.38),
        (24.0, b'o', 0.30),
        (26.5, b'o', 0.24),
    ];
    let mut buf = [b' '; W * H];
    for y in 0..H {
        for x in 0..W {
            let dx = (x as f64 - cx) / ax;
            let dy = (y as f64 - cy) / ay;
            let rr = (dx * dx + dy * dy).sqrt();
            if planets.iter().any(|p| (rr - p.0).abs() < 0.55) {
                buf[y * W + x] = b'.';
            }
        }
    }
    buf[cy.round() as usize * W + cx.round() as usize] = b'@';
    for p in planets.iter() {
        let a = t * p.2;
        let x = (cx + p.0 * ax * a.cos()).round() as isize;
        let y = (cy + p.0 * ay * a.sin()).round() as isize;
        if x >= 0 && (x as usize) < W && y >= 0 && (y as usize) < H {
            buf[y as usize * W + x as usize] = p.1;
        }
    }
    let mut s = String::with_capacity((W + 1) * H);
    for r in 0..H {
        for c in 0..W {
            s.push(buf[r * W + c] as char);
        }
        s.push('\n');
    }
    s
}

#[function_component(Orrery)]
fn orrery() -> Html {
    use_anim_tick(60);
    let t = js_sys::Date::now() / 1000.0;
    html! {
        <div class="ascii-art">
            <div class="ascii-cmd">{ "$ ./orrery" }</div>
            <pre class="ascii-face orrery-face">{ orrery_frame(t) }</pre>
        </div>
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
const BRAIN: &str = r#"       .-~~~~~~~-.
     .~ o  o  o o ~.
   .~ o .-~~~-. o o ~.
  / o  / o  o o \  o  \
 | o  | o  o o o | o o |
 |o o | o o  o o | o  o|
 | o  | o  o o o | o o |
  \ o  \ o o  o /  o  /
   ~. o  ~-.-~  o o .~
     ~. o  o  o  o.~
       ~-._____.-~"#;

fn brain_frame(t: f64) -> String {
    let ramp = b" .:-+ioO@";
    let front = (t / 2.6).fract() * 14.0; // bright spike sweeping across every 2.6s
    let mut s = String::with_capacity(BRAIN.len() + 16);
    for (r, line) in BRAIN.lines().enumerate() {
        for (c, ch) in line.bytes().enumerate() {
            if ch == b'o' {
                let phase = r as f64 * 0.7 + c as f64 * 0.32;
                let base = 0.5 + 0.5 * (t * 3.5 - phase).sin();
                let d = phase - front;
                let boost = (-(d * d) / 2.42).exp(); // gaussian spike front (2*1.1^2)
                let b = base.max(boost);
                let idx = ((b * (ramp.len() as f64 - 1.0)) as usize).min(ramp.len() - 1);
                s.push(ramp[idx] as char);
            } else {
                s.push(ch as char);
            }
        }
        s.push('\n');
    }
    s
}

#[function_component(BrainViz)]
fn brain_viz() -> Html {
    use_anim_tick(80);
    let t = js_sys::Date::now() / 1000.0;
    let frame = brain_frame(t);
    let active = frame.bytes().filter(|&b| b == b'O' || b == b'@').count();
    let bright = frame.bytes().filter(|&b| b == b'@').count();
    let hz = 7.0 + 1.6 * (t * 0.5).sin();
    let sub = if bright > 1 {
        format!("\u{03B8} {:.1} Hz \u{00B7} {} firing \u{00B7} \u{21AF} spike", hz, active)
    } else {
        format!("\u{03B8} {:.1} Hz \u{00B7} {} firing", hz, active)
    };
    html! {
        <div class="ascii-art">
            <div class="ascii-cmd">{ "$ ./brain --live" }</div>
            <pre class="ascii-face brain-face">{ frame }</pre>
            <div class="brain-sub">{ sub }</div>
        </div>
    }
}

// --- interactive force-directed knowledge graph ---
struct GNode {
    x: f64,
    y: f64,
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
];
const KG_EDGES: &[(usize, usize)] = &[
    (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8),
    (1, 2), (1, 12), (2, 12), (2, 13), (2, 14),
    (10, 4), (10, 8), (10, 0), (11, 2), (11, 1), (11, 9), (9, 0),
    (8, 15), (3, 16), (16, 17), (3, 17), (5, 4), (5, 0),
    (18, 19), (19, 0), (19, 2), (20, 2), (20, 0),
    (21, 3), (21, 0), (21, 5),
];

fn kg_build() -> Vec<GNode> {
    let n = KG_NODES.len();
    (0..n)
        .map(|i| {
            let a = i as f64 / n as f64 * std::f64::consts::TAU;
            GNode {
                x: 180.0 + 70.0 * a.cos() + (i as f64 * 13.0 % 7.0),
                y: 140.0 + 55.0 * a.sin(),
                vx: 0.0,
                vy: 0.0,
            }
        })
        .collect()
}

fn kg_step(nodes: &mut [GNode], pinned: Option<usize>) {
    let n = nodes.len();
    let mut fx = vec![0.0f64; n];
    let mut fy = vec![0.0f64; n];
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = nodes[i].x - nodes[j].x;
            let dy = nodes[i].y - nodes[j].y;
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
        let dx = nodes[b].x - nodes[a].x;
        let dy = nodes[b].y - nodes[a].y;
        let d = (dx * dx + dy * dy).sqrt().max(1.0);
        let f = (d - 60.0) * 0.03;
        let (ux, uy) = (dx / d, dy / d);
        fx[a] += f * ux;
        fy[a] += f * uy;
        fx[b] -= f * ux;
        fy[b] -= f * uy;
    }
    for i in 0..n {
        if Some(i) == pinned {
            nodes[i].vx = 0.0;
            nodes[i].vy = 0.0;
            continue;
        }
        fx[i] += (180.0 - nodes[i].x) * 0.02;
        fy[i] += (140.0 - nodes[i].y) * 0.02;
        nodes[i].vx = (nodes[i].vx + fx[i]) * 0.82;
        nodes[i].vy = (nodes[i].vy + fy[i]) * 0.82;
        nodes[i].x = (nodes[i].x + nodes[i].vx).clamp(14.0, 346.0);
        nodes[i].y = (nodes[i].y + nodes[i].vy).clamp(12.0, 268.0);
    }
}

fn kg_neighbor(h: usize, i: usize) -> bool {
    KG_EDGES.iter().any(|&(a, b)| (a == h && b == i) || (b == h && a == i))
}
fn kg_r(kind: u8) -> f64 {
    match kind {
        0 => 8.0,
        3 => 4.5,
        4 => 7.0,
        _ => 6.0,
    }
}
fn kg_cls(kind: u8) -> &'static str {
    match kind {
        0 => "kg-root",
        2 => "kg-post",
        3 => "kg-tool",
        4 => "kg-feed",
        _ => "kg-concept",
    }
}
fn kg_fmt(v: f64) -> String {
    format!("{:.1}", v)
}
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
    let tick = use_state(|| 0u64);
    let hovered = use_state(|| None::<usize>);
    let sel_node = use_state(|| None::<usize>);
    let feed_count = use_state(|| None::<usize>);
    let svg_ref = use_node_ref();
    {
        let feed_count = feed_count.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = gloo_net::http::Request::get("/news.json").send().await {
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
        let tick = tick.clone();
        use_effect_with((), move |_| {
            let iv = if reduced_motion() {
                None
            } else {
                Some(gloo_timers::callback::Interval::new(33, move || {
                    let pinned = *drag.borrow();
                    {
                        let mut b = sim.borrow_mut();
                        kg_step(&mut b, pinned);
                    }
                    tick.set(0);
                }))
            };
            move || drop(iv)
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
        let svg_ref = svg_ref.clone();
        Callback::from(move |e: web_sys::MouseEvent| {
            if let Some(i) = *drag.borrow() {
                *moved.borrow_mut() = true;
                if let Some(el) = svg_ref.cast::<web_sys::Element>() {
                    let rect = el.get_bounding_client_rect();
                    if rect.width() > 0.0 && rect.height() > 0.0 {
                        let sx = (e.client_x() as f64 - rect.left()) / rect.width() * 360.0;
                        let sy = (e.client_y() as f64 - rect.top()) / rect.height() * 280.0;
                        let mut b = sim.borrow_mut();
                        b[i].x = sx.clamp(14.0, 346.0);
                        b[i].y = sy.clamp(12.0, 268.0);
                        b[i].vx = 0.0;
                        b[i].vy = 0.0;
                    }
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
        Callback::from(move |_: web_sys::MouseEvent| {
            *drag.borrow_mut() = None;
        })
    };
    let nodes = sim.borrow();
    let hv = *hovered;
    let sn = *sel_node;
    let focus = hv.or(sn);
    let t = js_sys::Date::now() / 1000.0;
    let path = props.path.clone();
    let pmode = !path.is_empty();
    html! {
        <div class="kg-wrap">
            <div class="ascii-cmd">{ "$ graph --knowledge  \u{00B7} hover \u{00B7} drag \u{00B7} click \u{00B7} try 'path a b'" }</div>
            <svg class="kg" ref={svg_ref.clone()} viewBox="0 0 360 280" preserveAspectRatio="xMidYMid meet"
                 onmousemove={onmove} onmouseup={onup} onmouseleave={onleave}>
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
                           onmousedown={ let d = drag.clone(); let m = moved.clone(); Callback::from(move |e: web_sys::MouseEvent| { e.prevent_default(); *d.borrow_mut() = Some(i); *m.borrow_mut() = false; }) }>
                            { if ringed { html! { <circle cx={kg_fmt(nd.x)} cy={kg_fmt(nd.y)} r={kg_fmt(r + 3.0)} class="kg-ring" /> } } else { html! {} } }
                            <circle cx={kg_fmt(nd.x)} cy={kg_fmt(nd.y)} r={kg_fmt(r)} class={kg_cls(kind)} />
                            <text x={kg_fmt(nd.x + r + 2.0)} y={kg_fmt(nd.y + 2.5)}>{ label }</text>
                        </g>
                    }
                }) }
            </svg>
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

// --- rotating wireframe cube (3D projection + line raster) ---
fn cube_frame(a: f64, b: f64) -> String {
    const W: usize = 44;
    const H: usize = 22;
    let (cx, cy) = (W as f64 / 2.0, H as f64 / 2.0);
    let verts = [
        (-1.0, -1.0, -1.0), (1.0, -1.0, -1.0), (1.0, 1.0, -1.0), (-1.0, 1.0, -1.0),
        (-1.0, -1.0, 1.0), (1.0, -1.0, 1.0), (1.0, 1.0, 1.0), (-1.0, 1.0, 1.0),
    ];
    let edges = [
        (0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 6), (6, 7), (7, 4),
        (0, 4), (1, 5), (2, 6), (3, 7),
    ];
    let (ca, sa) = (a.cos(), a.sin());
    let (cb, sb) = (b.cos(), b.sin());
    let mut proj = [(0.0f64, 0.0f64); 8];
    for (i, &(x, y, z)) in verts.iter().enumerate() {
        let x1 = x * cb + z * sb;
        let z1 = -x * sb + z * cb;
        let y2 = y * ca - z1 * sa;
        let z2 = y * sa + z1 * ca;
        let sc = 32.0 / (3.2 + z2);
        proj[i] = (cx + x1 * sc, cy - y2 * sc * 0.5);
    }
    let mut buf = vec![b' '; W * H];
    let mut plot = |px: f64, py: f64, ch: u8, buf: &mut [u8]| {
        let (x, y) = (px.round() as isize, py.round() as isize);
        if x >= 0 && (x as usize) < W && y >= 0 && (y as usize) < H {
            buf[y as usize * W + x as usize] = ch;
        }
    };
    for &(u, v) in edges.iter() {
        let (x0, y0) = proj[u];
        let (x1, y1) = proj[v];
        let (dx, dy) = (x1 - x0, y1 - y0);
        let steps = dx.abs().max(dy.abs()).max(1.0) as usize;
        for s in 0..=steps {
            let t = s as f64 / steps as f64;
            plot(x0 + dx * t, y0 + dy * t, b'#', &mut buf);
        }
    }
    for &(px, py) in proj.iter() {
        plot(px, py, b'@', &mut buf);
    }
    let mut out = String::with_capacity((W + 1) * H);
    for r in 0..H {
        for c in 0..W {
            out.push(buf[r * W + c] as char);
        }
        out.push('\n');
    }
    out
}

#[function_component(CubeWireframe)]
fn cube_wireframe() -> Html {
    use_anim_tick(60);
    let t = js_sys::Date::now() / 1000.0;
    html! {
        <div class="ascii-art">
            <div class="ascii-cmd">{ "$ ./cube" }</div>
            <pre class="ascii-face cube-face">{ cube_frame(t * 0.7, t * 0.9) }</pre>
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
    let items = use_state(|| None::<Vec<NewsItem>>);
    let page = use_state(|| 0usize);
    let err = use_state(|| false);
    {
        let items = items.clone();
        let err = err.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match gloo_net::http::Request::get("/news.json").send().await {
                    Ok(resp) => match resp.json::<Vec<NewsItem>>().await {
                        Ok(v) => items.set(Some(v)),
                        Err(_) => err.set(true),
                    },
                    Err(_) => err.set(true),
                }
            });
            || ()
        });
    }
    let body = match (&*items, *err) {
        (None, true) => html! { <div class="news-loading">{ "ai-feed offline \u{2014} couldn't load news.json" }</div> },
        (None, false) => html! { <div class="news-loading">{ "fetching the AI feed\u{2026}" }</div> },
        (Some(v), _) if v.is_empty() => html! { <div class="news-loading">{ "feed warming up \u{2014} the factory posts fresh AI / agentic / LLM stories here every day \u{1F5DE}\u{FE0F}" }</div> },
        (Some(v), _) => {
            let total = v.len();
            let pages = ((total + NEWS_PER_PAGE - 1) / NEWS_PER_PAGE).max(1);
            let cur = (*page).min(pages - 1);
            let start = cur * NEWS_PER_PAGE;
            let end = (start + NEWS_PER_PAGE).min(total);
            let prev = { let p = page.clone(); Callback::from(move |_: web_sys::MouseEvent| { if *p > 0 { p.set(*p - 1); } }) };
            let next = { let p = page.clone(); Callback::from(move |_: web_sys::MouseEvent| { if *p + 1 < pages { p.set(*p + 1); } }) };
            html! { <>
                <ul class="news-list">
                    { for v[start..end].iter().map(news_item) }
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
    let lines = use_state(|| None::<Vec<DreamLine>>);
    let err = use_state(|| false);
    {
        let lines = lines.clone();
        let err = err.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match gloo_net::http::Request::get("/dmesg.json").send().await {
                    Ok(resp) => match resp.json::<Vec<DreamLine>>().await {
                        Ok(v) => lines.set(Some(v)),
                        Err(_) => err.set(true),
                    },
                    Err(_) => err.set(true),
                }
            });
            || ()
        });
    }
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

#[function_component(SparkMonitor)]
fn spark_monitor() -> Html {
    let data = use_state(|| None::<SparkData>);
    let err = use_state(|| false);
    {
        let data = data.clone();
        let err = err.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match gloo_net::http::Request::get("/spark.json").send().await {
                    Ok(resp) => match resp.json::<SparkData>().await {
                        Ok(v) => data.set(Some(v)),
                        Err(_) => err.set(true),
                    },
                    Err(_) => err.set(true),
                }
            });
            || ()
        });
    }
    let body = match (&*data, *err) {
        (None, true) => html! { <div class="dj-loading">{ "dgx-spark monitor offline \u{2014} spark.json unreachable" }</div> },
        (None, false) => html! { <div class="dj-loading">{ "polling dgx-spark\u{2026}" }</div> },
        (Some(d), _) => html! { <pre class="ascii-face spark-face">{ spark_text(d) }</pre> },
    };
    html! {
        <div class="ascii-art">
            <div class="ascii-cmd">{ "$ ssh dgx-spark nvidia-smi  \u{00B7}  the machine that builds this" }</div>
            { body }
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let selected = use_state(|| None::<usize>);
    let path_hl = use_state(|| Vec::<usize>::new());
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
                    <p>{ p.body }</p>
                </article>
            }
        }
        None => html! {
            <>
            <RustBadge />
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
            <AsciiClock />
            <BrainCard />
            <DreamJournal />
            <SparkMonitor />
            <BrainViz />
            <Orrery />
            <SpinningDonut />
            <CubeWireframe />
            <DoomFire />
            <MoonPhase />
            <Starfield />
            <KnowledgeGraph on_open={ let s = selected.clone(); Callback::from(move |i: usize| s.set(Some(i))) } path={(*path_hl).clone()} />
            <NewsFeed />
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
                        </li>
                    }
                }) }
            </ul>
            <Terminal on_path={ let p = path_hl.clone(); Callback::from(move |pv: Vec<usize>| p.set(pv)) } />
            </>
        },
    };

    html! {
        <>
            <header>
                <div class="logo">{ "raghu" }<span class="cursor">{ "\u{2588}" }</span></div>
                <p class="boot">{ "// dark-factory online · brain healthy · shipping from wasm" }</p>
            </header>
            <main>{ view }</main>
            <footer>
                <div class="social">
                    <a href="https://www.linkedin.com/in/rgnair">{ "linkedin" }</a>
                    { " · " }
                    <a href="https://github.com/raghunathnair1-rgb">{ "github" }</a>
                </div>
                { "built in Rust \u{2192} WebAssembly · shipped by an AI harness brain" }
            </footer>
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
