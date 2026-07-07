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
        ["help"] => "commands: help  whoami  ls  cat <post>  meow  neofetch  now-playing  coffee  brew  fortune  theme <name>  crt  reboot  uptime  echo <x>  clear".to_string(),
        ["reboot"] => "rebooting the dark factory\u{2026}".to_string(),
        ["crt"] | ["crt", "on"] | ["crt", "off"] => "CRT mode \u{1F4FA} toggled".to_string(),
        ["whoami"] => "raghu \u{2014} builder \u{00B7} tinkerer \u{00B7} runs an AI dark factory for fun".to_string(),
        ["ls"] => "about.md   now-playing   neofetch   posts/   linkedin   github".to_string(),
        ["ls", "posts"] | ["ls", "posts/"] => "hello-world.md   anatomy-of-a-dark-factory.md   why-webassembly.md".to_string(),
        ["neofetch"] => "os: dark-factory \u{00B7} kernel: rust\u{2192}wasm \u{00B7} shell: harness brain \u{00B7} status: \u{25CF} online".to_string(),
        ["now-playing", ..] => "\u{266B} Cornfield Chase \u{2014} Hans Zimmer \u{00B7} Interstellar (OST)".to_string(),
        ["fortune"] => "\u{201C}Do not go gentle into that good night...\u{201D} \u{2014} Interstellar".to_string(),
        ["uptime"] => "shipping since 2026-07-06 \u{00B7} brain online".to_string(),
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

#[function_component(Terminal)]
fn terminal() -> Html {
    let history = use_state(|| vec![Line::Out("dark-factory shell \u{2014} type 'help' for commands.".to_string())]);
    let value = use_state(String::new);

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
        Callback::from(move |e: web_sys::KeyboardEvent| {
            if e.key() == "Enter" {
                let cmd = (*value).trim().to_string();
                value.set(String::new());
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
                if let Ok(resp) = gloo_net::http::Request::get("https://wttr.in/?format=j1").send().await {
                    if let Ok(d) = resp.json::<Wttr>().await {
                        if let (Some(c), Some(a)) = (d.current_condition.first(), d.nearest_area.first()) {
                            let city = a.area_name.first().map(|v| v.value.as_str()).unwrap_or("somewhere");
                            let country = a.country.first().map(|v| v.value.as_str()).unwrap_or("");
                            let desc = c.desc.first().map(|v| v.value.as_str()).unwrap_or("");
                            let code = cc(country);
                            wx.set(Some(format!(
                                "{} {}, {}: {} \u{00B7} {}\u{00B0}C \u{00B7} wind {}km/h \u{00B7} humidity {}%",
                                flag(&code), city, code, desc, c.temp_c, c.wind, c.humidity
                            )));
                        }
                    }
                }
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
    let tick = use_state(|| 0u64);
    {
        let st = st.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = gloo_net::http::Request::get("/status.json").send().await {
                    if let Ok(b) = resp.json::<BrainStatus>().await {
                        st.set(Some(b));
                    }
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
                match &*st {
                    Some(b) => {
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
                    None => html! { <div class="brain-loading">{ "querying the harness brain\u{2026}" }</div> },
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
    let f = use_state(|| 0u64);
    {
        let f = f.clone();
        use_effect_with((), move |_| {
            let iv = if reduced_motion() {
                None
            } else {
                Some(gloo_timers::callback::Interval::new(50, move || f.set(0)))
            };
            move || drop(iv)
        });
    }
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
    let f = use_state(|| 0u64);
    {
        let f = f.clone();
        use_effect_with((), move |_| {
            let iv = if reduced_motion() {
                None
            } else {
                Some(gloo_timers::callback::Interval::new(60, move || f.set(0)))
            };
            move || drop(iv)
        });
    }
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

#[function_component(App)]
fn app() -> Html {
    let selected = use_state(|| None::<usize>);
    let list = posts();

    let view = match *selected {
        Some(i) => {
            let p = &list[i];
            let back = {
                let s = selected.clone();
                Callback::from(move |_| s.set(None))
            };
            html! {
                <article>
                    <a class="back" onclick={back}>{"‹ back to log"}</a>
                    <h2>{ p.title }</h2>
                    <div class="meta"><span class="tag">{ format!("#{}", p.tag) }</span>{ " · " }<time>{ p.date }</time></div>
                    <p>{ p.body }</p>
                </article>
            }
        }
        None => html! {
            <>
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
            <RustBadge />
            <WeatherCard />
            <AsciiClock />
            <BrainCard />
            <Orrery />
            <SpinningDonut />
            <ul class="log">
                { for list.iter().enumerate().map(|(i, p)| {
                    let s = selected.clone();
                    let open = Callback::from(move |_| s.set(Some(i)));
                    html! {
                        <li onclick={open}>
                            <span class="prompt">{ "›" }</span>
                            <span class="title">{ p.title }</span>
                            <span class="tag">{ format!("#{}", p.tag) }</span>
                            <time>{ p.date }</time>
                        </li>
                    }
                }) }
            </ul>
            <Terminal />
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
