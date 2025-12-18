use leptos::{logging::log, prelude::*, task};
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
        <Header/>

    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="header">
            <label for="sidemenu" style="margin-left: 10px">
                <img src="./images/menu_line72.png" alt="メニュー" height="40px"/>
            </label>
            <div class="divider"></div>
            <img src="./images/tabicon.JPG" alt="アイコン" class="logo" height="40px"/>
            <div class="search-wrap">
                <img src="./images/search_fill48.png" class="search-icon" />
                <input type="text" class="searchbar" placeholder="タブ検索"/>
            </div>
            <img src="./images/beru.png" alt="アイコン" class="beru" height="40px"/> 
            <img src="./images/kariicon.jpg" alt="アイコン" class="kariicon" height="40px"/> 
        </header> 
        <input type="checkbox" id="sidemenu" hidden/> 
        <label for="sidemenu" class="overlay"></label>
        <nav class="sidebar"> 
            <a>"ホーム"</a>
            <a>"投稿"</a>
            <a>"プロフ"</a>
        </nav>
    }
}
