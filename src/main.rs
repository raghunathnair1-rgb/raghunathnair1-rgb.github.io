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
    vec![Post {
        title: "Hello, world — the factory is live",
        date: "2026-07-06",
        tag: "meta",
        body: "This blog is written in Rust, compiled to WebAssembly, and shipped by an \
               autonomous AI 'dark factory' running on a VPS. No humans on the floor — just a \
               harness brain, a task queue, security + QA gates, an ontology for context, and a \
               kill switch. You add a task; the brain writes the post, the gates check it, and \
               CI builds the WASM and deploys it here. You're reading its first output.",
    }]
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
                { "built in Rust \u{2192} WebAssembly · shipped by an AI harness brain · " }
                <a href="https://github.com/raghunathnair1-rgb">{ "github" }</a>
            </footer>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
