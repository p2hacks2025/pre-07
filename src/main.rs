use console_error_panic_hook;
use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(App);
    console_error_panic_hook::set_once();
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Header/>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="header">
            <label for="sidemenu" class="menu-btn">"☰"</label>
            <span class="title">biestar</span>
            <input type="checkbox" id="sidemenu" hidden />
            <nav class="sidebar">
                <a href="#">"ホーム"<br/></a>
                <a href="#">"投稿"<br/></a>
                <a href="#">"プロフ"<br/></a>
            </nav>
        </header>
    }
}
