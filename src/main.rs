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
        ["help"] => "commands: help  whoami  ls  cat <post>  meow  neofetch  now-playing  coffee  brew  fortune  theme <name>  uptime  echo <x>  clear".to_string(),
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
}
#[derive(serde::Deserialize)]
struct Wttr {
    current_condition: Vec<WxCond>,
    nearest_area: Vec<WxArea>,
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
                            let desc = c.desc.first().map(|v| v.value.as_str()).unwrap_or("");
                            wx.set(Some(format!(
                                "{}: {} \u{00B7} {}\u{00B0}C \u{00B7} wind {}km/h \u{00B7} humidity {}%",
                                city, desc, c.temp_c, c.wind, c.humidity
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
    // blink the colons: shown first half of each second, blank the second half
    if d.get_milliseconds() < 500.0 { t } else { t.replace(':', " ") }
}

fn now_date() -> String {
    let d = js_sys::Date::new_0();
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    format!(
        "{} {} {:02} {}",
        days[d.get_day() as usize],
        months[d.get_month() as usize],
        d.get_date() as u32,
        d.get_full_year() as u32
    )
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
            <WeatherCard />
            <AsciiClock />
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
