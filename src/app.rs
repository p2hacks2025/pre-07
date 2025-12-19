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
        <div class="box" id="side-space-left">
            <div class="tag-function">
                <input class="tag-search-window" type="text" placeholder="タグを検索"/>
                <div class="tag-predict">
                    <p>"タグ候補"</p>
                    <div class="tag-object">
                        <TagSelect tag="test_tag".to_string()/>
                    </div>
                </div>
            </div>
        </div>
        <div class="outer">
                <div class="post-function">
                    <input class="title-space" type="text" placeholder="タイトルを入力"/> <br/>
                    <div class="tag-space">
                        <div class="tag-object">//タグひとまとまり
                            <div class="tag-name">
                                <TagSelect tag="test1_tag".to_string()/>
                            </div>
                            <div class="tag-status">
                            <form>
                                <label>
                                    <select>
                                        <option>"A"</option>
                                        <option>"B"</option>
                                        <option>"C"</option>
                                    </select>
                                </label>
                                </form>
                            </div>
                            <div class="tag-cancel">
                            "×"
                            </div>
                        </div>
                    </div>
                    <div class="text-area-space">
                        <textarea class="text-space" placeholder="内容を入力"/> <br/>
                        <div class="text-function">
                            <div class="picture-button">
                                <img src="/images/seal_certificate_line72.png"/>
                            </div>
                        </div>
                    </div>
                    <div class="post-button">
                        <img src="/images/mailing_fill72.png"/>
                    </div>
                </div>
        </div>
        <div class="form-check">
            <input class="form-check-input" type="radio" name="flexRadioDefault" id="flexRadioDefault1" checked></input>
            <label class="form-check-label" for="flexRadioDefault1">
                "初心者"
            </label>
            <input class="form-check-input" type="radio" name="flexRadioDefault" id="flexRadioDefault2"></input>
            <label class="form-check-label" for="flexRadioDefault2">
                "経験者"
            </label>
        </div>
    }
}

#[component]
fn TagSelect(tag: String) -> impl IntoView{
    view!{
        <p> {tag} </p>
    }
}

#[component]
fn TagPreview(tag: String) -> impl IntoView{
    view!{
        <p> {tag} </p>
    }
}