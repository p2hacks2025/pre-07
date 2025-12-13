use console_error_panic_hook;
use leptos::{prelude::*};

fn main() {
    leptos::mount::mount_to_body(App);
    console_error_panic_hook::set_once();
}

#[component]
fn App() -> impl IntoView{
    view!{
    }
}
