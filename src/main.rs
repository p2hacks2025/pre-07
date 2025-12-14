use console_error_panic_hook;
use leptos::{prelude::*};

fn main() {
    leptos::mount::mount_to_body(App);
    console_error_panic_hook::set_once();
}

#[component]
fn App() -> impl IntoView{
    view!{
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