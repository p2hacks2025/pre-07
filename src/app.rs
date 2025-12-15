use leptos::{logging::log, prelude::*, task};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet};

use crate::server;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="ja">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (text, set_text) = signal(String::new());
    let click = move |_| {
        task::spawn_local(async move {
            let s = server::test().await.unwrap();
            log!("{}", s);

            set_text.set(s);
        })
    };

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/pre-07.css"/>

        // sets the document title
        //<Title text="Welcome to Leptos"/>
        <p> {move || text.get()} </p>
        <button on:click={click}> "test" </button>
    }
}
