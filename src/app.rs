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
fn Login() -> impl IntoView {
    let (visible, set_visible) = signal(true);
    let change_visible = move || if visible.get() { "password" } else { "input" };

    fn check_password_length(password: String) -> bool {
        password.chars().count() >= 8
    }

    let (password, set_password) = signal(String::new());
    let (name, set_name) = signal(String::new());

    let login = move || {
        task::spawn_local(async move {
            log!(
                "{}",
                server::log_in(name.get_untracked(), password.get_untracked())
                    .await
                    .unwrap()
                    .unwrap()
            );
        });
    };
    let signup = move || {
        task::spawn_local(async move {
            log!(
                "{}",
                server::sign_up(name.get_untracked(), password.get_untracked())
                    .await
                    .unwrap()
                    .unwrap()
            );
        });
    };

    view! {
        <img class="backpicture" src="./images/IMG_0257.JPG" alt="Background Image"/>
            <div class="login-board">
            <input type="text" class="user-name" autocomplete="username" placeholder="ユーザーネーム" on:input:target=move |ev| set_password.set(ev.target().value())/>
            <div class="password-wrap">
                <input type={change_visible} class="password" placeholder="パスワード" on:input:target=move |ev| set_password.set(ev.target().value())/>
                <img src="./images/eye_transparent.png" class="eye-icon"
                    on:click={move |_| *set_visible.write() = !visible.get()}/>
            </div>
            <button class="loginbtn" on:click={move |_| login()}>"ログイン"</button>
            <p class="wrongpassword">"passwordが間違っています"</p>
            <button class="signupbtn" on:click={move |_| signup()}>"新規登録"</button>
            </div>
    }
}
