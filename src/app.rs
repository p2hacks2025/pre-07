use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet};

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

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/pre-07.css"/>

        // sets the document title
        //<Title text="Welcome to Leptos"/>
        <PostScreen/>
    }
}

#[component]
fn PostScreen() -> impl IntoView{
    view!{
        <div class="flex"></div>
        <div class="box" id="side-space-left">One</div>
        <div class="outer">
                <div class="post-function">
                    <input class="title-space" type="text" placeholder="タイトルを入力"/> <br/>
                    <div class="tag-space">
                        <p>"asfd"</p>
                    </div>
                    <div class="text-area-space">
                        <textarea class="text-space" placeholder="内容を入力"/> <br/>
                        <div class="text-function">
                            <button type="button" class="picture">
                                <img src="../../images/seal_certificate_line72.png"/>
                            </button>
                        </div>
                    </div>
                    <div class="button-container">
                        <button type="button" class="post-button">
                            <img src="../../images/mailing_fill72.png"/>
                        </button>
                    </div>
                </div>
        </div>
        <div class="form-check">
            <input class="form-check-input" type="radio" name="flexRadioDefault" id="flexRadioDefault1"></input>
            <label class="form-check-label" for="flexRadioDefault1">
                初心者
            </label>
            <input class="form-check-input" type="radio" name="flexRadioDefault" id="flexRadioDefault2" checked></input>
            <label class="form-check-label" for="flexRadioDefault2">
                上級者
            </label>
        </div>
    }
}