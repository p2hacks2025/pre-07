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
        <Login/>
    }
}

#[component]
fn Login() -> impl IntoView{
    let (visible, set_visible) = signal(true);
    let password = move || if visible.get() {"password"} else {"input"}; 

    view!{
        <img class="backpicture" src="./images/IMG_0257.JPG" alt="Background Image"/>
            <div class="login-board">
            <input type="email" class="mail" autocomplete="username" placeholder="メールアドレス"/>
            <div class="password-wrap">
                <input type={password} class="password" placeholder="パスワード"/>
                <img src="./images/eye_transparent.png" class="eye-icon"
                    on:click={move |_| *set_visible.write() = !visible.get()}/>
            </div>
            <button class="loginbtn">"ログイン"</button>
            <p class="wrongpassword">"passwordが間違っています"</p>
            <button class="signupbtn">"新規登録"</button>
            </div>
    }
}