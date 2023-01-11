use gloo::utils::document;
use log::info;
use std::borrow::BorrowMut;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};
use yew::prelude::*;
use yew::props;
use yew_hooks::prelude::*;

use crate::msg_ctx::MessageContext;

use yew::{function_component, html, Html, Properties};

#[function_component]
pub fn Producer() -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();
    let msg_ctx_clone = msg_ctx.clone();
    use_effect_once(|| {
        info!("This code only runs once after page loads");
        let is_native_dark = window()
            .unwrap()
            .match_media("(prefers-color-scheme: dark)")
            .ok()
            .unwrap()
            .unwrap();

        let button = document()
            .query_selector("#darkToggle")
            .ok()
            .unwrap()
            .unwrap()
            .dyn_into::<HtmlElement>();

        if is_native_dark.matches() {
            button.ok().unwrap().click();
        }

        || info!("Running clean-up of effect on unmount")
    });

    let onclick = Callback::from(move |_| {
        info!("Hello {}", msg_ctx.inner);
        // msg_ctx.inner;
        msg_ctx.dispatch(!msg_ctx.inner);
    });

    html! {
            <>
         <div class="switch">
           <input id="darkToggle" type="checkbox" name="darkToggle" class={classes!("switch")} onclick={onclick} />
             <label for="darkToggle">{"Dark Mode"}</label>
            </div>
            </>
    }
}
