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
            <label for="sidemenu" style="margin-left: 10px">
                <img src="./images/menu_line72.png" alt="メニュー" height="40px"/>
            </label>
            <div class="divider"></div>
            <img src="./images/tabicon.JPG" alt="アイコン" class="logo" height="40px"/> 
            <input type="text" class="searchbar" placeholder="🔎タグ検索"></input>
            <img src="./images/beru.png" alt="アイコン" class="beru" height="40px"/> 
            <img src="./images/kariicon.jpg" alt="アイコン" class="kariicon" height="40px"/> 
        </header> 
        <input type="checkbox" id="sidemenu" hidden/> 
        <nav class="sidebar"> 
            <a>"ホーム"<br/></a>
            <a>"投稿"<br/></a>
            <a>"プロフ"<br/></a> 
        </nav>
    }
}
