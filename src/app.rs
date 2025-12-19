use leptos::{logging::log, prelude::*, task};
use leptos_router::{components::*, path};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use serde::{Deserialize, Serialize};

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

#[derive(Clone)]
struct User {
    jwt: String,
    name: String,
}


#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (user, user_write) = signal(None as Option<User>);
    provide_context(user);
    provide_context(user_write);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/pre-07.css"/>

        // sets the document title
        //<Title text="Welcome to Leptos"/>

        <Title text="Biestar"/>
        <Show when=move || {
            match user.get(){
                None => false,
                Some(_) => true
            }
        } fallback=Login>

        <Header/>

        <Router>
            <Routes fallback=|| "NotFound">
                <Route path=path!("/test") view=Test/>
            </Routes>
        </Router>

        </Show>

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

#[component]
fn Test() -> impl IntoView {
    // 後で消してください
    view! {
        <h1> "TEST" </h1>
    }
}

//ログイン画面

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum LoginScreenState {
    Ok,
    InvalidAccount,
    TooShortPassword,
    NameExists,
    Logining,
    SigningUp,
}

#[component]
fn Login() -> impl IntoView {
    let (visible, set_visible) = signal(true);
    let change_visible = move || if visible.get() { "password" } else { "input" };
    let (login_state, set_login_state) = signal(LoginScreenState::Ok);

    let (name, set_name) = signal(String::new());
    let (password, set_password) = signal(String::new());

    let login = move |(name, password): (String, String)| {
        set_login_state.set(LoginScreenState::Logining);
        task::spawn_local(async move {
            let api = server::log_in(name.clone(), password).await.unwrap();
            match api {
                Ok(token) => {
                    set_login_state.set(LoginScreenState::Ok);
                    let user_write = use_context::<WriteSignal<Option<User>>>().unwrap();
                    user_write.set(Some(User { jwt: token, name }))
                }
                Err(state) => set_login_state.set(state),
            }
        });
    };
    let signup = move |(name, password): (String, String)| {
        set_login_state.set(LoginScreenState::SigningUp);
        task::spawn_local(async move {
            let api = server::sign_up(name.clone(), password).await.unwrap();
            match api {
                Ok(token) => {
                    set_login_state.set(LoginScreenState::Ok);
                    let user_write = use_context::<WriteSignal<Option<User>>>().unwrap();
                    user_write.set(Some(User { jwt: token, name }))
                }
                Err(state) => set_login_state.set(state),
            }
        });
    };

    view! {
        <img class="backpicture" src="./images/IMG_0257.JPG" alt="Background Image"/>
            <div class="login-board">
            <input type="text" class="user-name" autocomplete="username" placeholder="ユーザーネーム" on:input:target=move |ev| set_name.set(ev.target().value())/>
            <div class="password-wrap">
                <input type={change_visible} class="password" placeholder="パスワード" on:input:target=move |ev| set_password.set(ev.target().value())/>
                <img src="./images/eye_transparent.png" class="eye-icon"
                    on:click={move |_| *set_visible.write() = !visible.get()}/>
            </div>
            <button class="loginbtn" on:click={move |_| login((name.get(), password.get()))}>"ログイン"</button>
            <Show
                when=move || login_state.get() != LoginScreenState::Ok> <p class="wrongpassword">{move || {
                    match login_state.get(){
                        LoginScreenState::Ok => unreachable!(),
                        LoginScreenState::InvalidAccount => "パスワードかユーザーネームが間違っています",
                        LoginScreenState::Logining => "ログイン中です",
                        LoginScreenState::SigningUp => "登録中です",
                        LoginScreenState::NameExists => "その名前は存在しています",
                        LoginScreenState::TooShortPassword => "パスワードは8文字以上にしてください",
                    }
                }
            }</p>
            </Show>
            <button class="signupbtn" on:click={move |_| signup((name.get(), password.get()))}>"新規登録"</button>
            </div>
    }
}
