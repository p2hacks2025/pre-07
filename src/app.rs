use leptos::{logging::log, prelude::*, task};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{components::*, path};
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

#[derive(Clone, Debug)]
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

        <Router>
            <Header/>
            <Routes fallback=|| "NotFound">
                <Route path=path!("/") view=MainScreen/>
                <Route path=path!("/post") view=PostScreen/>
            </Routes>
        </Router>

        </Show>
    }
}

#[component]
fn PostScreen() -> impl IntoView {
    let (search_tag, set_search_tag) = signal(Vec::<String>::new());
    let (select_tag, set_select_tag) = signal(Vec::<String>::new());

    let (search_string, set_search_string) = signal(String::new());

    let (title, set_title) = signal(String::new());
    let (body, set_body) = signal(String::new());
    let (advanced, set_advanced) = signal(false);

    let post = |title, body, tag, is_advanced, user:Option<User>| {
        task::spawn_local(async move {
            if let Some(u) = user {
                let x = server::do_post(u.name, u.jwt, title, body, Some(tag), is_advanced)
                    .await
                    .unwrap();
            }
        })
    };

    Resource::new(
        move || search_string.get(),
        move |s| async move {
            if !s.is_empty() {
                task::spawn_local(async move {
                    set_search_tag.set(server::search_tag_with_prefix(s, 3).await.unwrap());
                });
            } else {
                set_search_tag.set(vec![]);
            }
        },
    );

    view! {
        <div class="box" id="side-space-left">
            <div class="tag-function">
                <input class="tag-search-window" type="text" placeholder="タグを検索" on:input:target=move |ev| {set_search_string.set(ev.target().value())}/>
                <div class="tag-predict">
                    <p>"タグ候補"</p>
                    <For
                        each=move || search_tag.get()
                        key=|tag| tag.clone()
                        let(tag)
                    >
                        <TagSelect tag=tag set_select_tag=set_select_tag/>
                    </For>
                </div>
            </div>
        </div>
        <div class="outer">
                <div class="post-function">
                    <input class="title-space" type="text" placeholder="タイトル" on:input:target=move |ev| {set_title.set(ev.target().value())}/> <br/>
                    <div class="tag-space">
                        <For
                            each=move || select_tag.get()
                            key=|tag| tag.clone()
                            let(tag)
                        >
                            <TagSearch tag=tag set_select_tag=set_select_tag/>
                        </For>
                    </div>
                        <textarea class="text-area-space" placeholder="内容を入力" on:input:target=move |ev| {set_body.set(ev.target().value())}/>
                    <div class="post-button">
                        <img src="/images/mailing_fill72.png" on:click=move |_| {post(title.get(), body.get(), select_tag.get(), advanced.get(), use_context::<ReadSignal<Option<User>>>().unwrap().get())}/>
                    </div>
                </div>
        </div>
        <div class="form-check">
            <input class="form-check-input" type="radio"
                prop:checked=move || !advanced.get()
                on:change=move |_| set_advanced.set(false)
                ></input>
            <label class="form-check-label" on:click=move |_| set_advanced.set(false)>
                "初心者"
            </label>
            <input class="form-check-input" type="radio"
                prop:checked=move || advanced.get()
                on:change=move |_| set_advanced.set(true)
            ></input>
            <label class="form-check-label" on:click=move |_| set_advanced.set(true)>
                "経験者"
            </label>
        </div>
    }
}

#[component]
fn TagSelect(tag: String, set_select_tag: WriteSignal<Vec<String>>) -> impl IntoView {
    view! {
        <div class="tag-object" on:click=move |_| {
            let mut l = set_select_tag.write();
            if !l.contains(&tag) {
                l.push(tag.clone());
            }
            log!("{:?}", *l);
        }>
            <p> {tag.clone()} </p>
        </div>
    }
}

#[component]
fn TagSearch(tag: String, set_select_tag: WriteSignal<Vec<String>>) -> impl IntoView {
    view! {
        <div class="tag-object">//タグひとまとまり
            <div class="tag-name">
                <p> {tag.clone()} </p>
            </div>
            <div class="tag-cancel" on:click=move |_| {
                set_select_tag.write().retain(|x| *x != tag);
            }>
                "×"
            </div>
        </div>
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
                <input type="text" class="searchbar" placeholder="タグ検索"/>
            </div>
            <img src="./images/beru.png" alt="アイコン" class="beru" height="40px"/>
            <img src="./images/kariicon.jpg" alt="アイコン" class="kariicon" height="40px"/>
        </header>
        <input type="checkbox" id="sidemenu" hidden/>
        <label for="sidemenu" class="overlay"></label>

        <nav class="sidebar">
            <A href="/">"ホーム"</A>
            <A href="/post">"投稿"</A>
            <img src="./images/bear.png" alt="熊" width="150px"/>
        </nav>
    }
}

#[component]
fn MainScreen() -> impl IntoView{
    let (posts, set_posts) = signal(vec![server::Post{title: "最強の推し".to_string(),id: "id".to_string(), name: "ルビス".to_string(), body: "最近はまっているのはツクリちゃん！\nツクリちゃんの歌うロミオとシンデレラを初めて聞いたときは脳を打ち抜かれました…！\nマルチクリエイティブVtuberということもあり、作曲、歌唱、MIX、動画制作などすべてできるものすごいお方！\n落ち着いた声もかっこいい歌声も最高なので１度聞いてみてほしいです！".to_string(), tags: vec!["推し活".to_string(), "ミリプロ".to_string()], is_advanced: true, comment:vec![]}]);
    view!{
        <div class="main-layout">
            <For
                each=move || posts.get()
                key=|post| post.id.clone()
                let(post)
            >
                <MainScreenPost post=post/>
            </For>
            <div class="post-right">
                <div class="post">
                    <div class="post-icon"><img src="./images/kariicon.jpg" alt="アイコン" class="kariicon" height="40px"/></div>

                    <div class="post-content">
                        <div class="post-header">
                            <span class="post-title">最高の推し</span>
                            <span class="post-username">ルビス</span>
                            <span class="post-attribute">初心者</span>/*経験者の時post-attribute-experience*/
                        </div>
                        <div class="post-text">
                        "最推しはあくたん！なんといっても彼女の魅力はそのかわいらしい声とゲームのうまさ！
その歌声は万物をいやし、落ち込んだ心を救済すること間違いなし！
また、得意とするAPEXでは常人では目の追いつかないほどの速度で敵を打ち倒す！
その強さを表現する語彙力がないことが実に口惜しい…！
まさに銀河１のアイドルはあくたんしかいないと思っています！"
                        </div>
                        <div class="post-actions">
                            <span class="post-tag">"推し活"</span>
                            <span class="post-tag">"hololive"</span>
                        </div>
                        <div class="post-footer">
                            <span class="check-btn">返信</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
}
}

#[component]
fn MainScreenPost(post: server::Post) -> impl IntoView {
    let tags = post
        .tags
        .iter()
        .map(|t| view! {<span class="post-tag"> {t.to_string()} </span>})
        .collect_view();

    view! {
        <div class="main-layout">
            <div class="timeline">
                <div class="post">
                    <div class="post-icon"><img src="./images/kariicon.jpg" alt="アイコン" class="kariicon" height="40px"/></div>

                    <div class="post-content">
                        <div class="post-header">
                            <span class="post-title"> {post.title}</span>
                            <span class="post-username"> {post.name} </span>
                            <span class="post-attribute" class:post-attribute-experience=post.is_advanced> {
                                if post.is_advanced{
                                    "経験者"
                                } else {
                                    "初心者"
                                }
                            }</span>/*経験者の時post-attribute-experience*/
                        </div>

                        <div class="post-text-preview">
                            {post.body}
                        </div>
                        <div class="post-actions">
                            {
                                tags.collect_view()
                            }
                        </div>
                        <div class="post-footer">
                            <span class="check-btn">全文表示</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
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

#[component]
fn ResponceScreen() -> impl IntoView{
    view!{
        <div class="box" id="side-space-left">
        //仮　かいとの投稿文プレビュー
        <div class="post-right">
                <div class="post">
                    <div class="post-icon"><img src="./images/kariicon.jpg" alt="アイコン" class="kariicon" height="40px"/></div>
                    <div class="post-content">
                        <div class="post-header">
                            <span class="post-title">最高の推し</span>
                            <span class="post-username">ルビス</span>
                            <span class="post-attribute">初心者</span>/*経験者の時post-attribute-experience*/
                        </div>
                        <div class="post-text">
                        "最推しはあくたん！なんといっても彼女の魅力はそのかわいらしい声とゲームのうまさ！
その歌声は万物をいやし、落ち込んだ心を救済すること間違いなし！
また、得意とするAPEXでは常人では目の追いつかないほどの速度で敵を打ち倒す！
その強さを表現する語彙力がないことが実に口惜しい…！
まさに銀河１のアイドルはあくたんしかいないと思っています！"
                        </div>
                        <div class="post-actions">
                            <span class="post-tag">"推し活"</span>
                            <span class="post-tag">"hololive"</span>
                        </div>
                        <div class="post-footer">
                            <span class="check-btn">返信</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
            //仮　かいとの投稿文プレビュー　ここまで
        <div class="outer">
                <div class="text-area-space">
                    <textarea class="text-space" placeholder="内容を入力"/> <br/>
                </div>
                <div class="post-button">
                    <img src="/images/mailing_fill72.png"/>
                </div>
        </div>
        <div class="form-check">
            <input class="form-check-input" type="radio" name="flexRadioDefault" id="flexRadioDefault1"></input>
            <label class="form-check-label" for="flexRadioDefault1">
                "初心者"
            </label>
            <input class="form-check-input" type="radio" name="flexRadioDefault" id="flexRadioDefault2" checked></input>
            <label class="form-check-label" for="flexRadioDefault2">
                "経験者"
            </label>
        </div>

    }
}