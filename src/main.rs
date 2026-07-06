use yew::prelude::*;

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
                    <div class="nf-line"><span class="k">{ "status" }</span><span class="nf-ok">{ "\u{25CF} online" }</span></div>
                </div>
            </div>
            <div class="fortune">
                <div class="nf-cmd">{ "$ fortune" }</div>
                <blockquote>{ "\u{201C}Do not go gentle into that good night; rage, rage against the dying of the light.\u{201D} \u{2014} Interstellar" }</blockquote>
            </div>
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
