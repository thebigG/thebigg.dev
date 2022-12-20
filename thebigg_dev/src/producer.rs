use log::info;
use yew::prelude::*;

use super::msg_ctx::MessageContext;

#[function_component]
pub fn Producer() -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();

    let onclick = Callback::from(move |_| {
        info!("Hello {}", msg_ctx.inner);
        // msg_ctx.inner;
        msg_ctx.dispatch(!msg_ctx.inner)
    });

    html! {
        <button {onclick}>
            {"PRESS ME"}
        </button>
    }
}
