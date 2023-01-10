use log::info;
use std::borrow::BorrowMut;
use web_sys::window;
use yew::prelude::*;
use yew::props;
use yew_hooks::prelude::*;

use crate::msg_ctx::MessageContext;

#[function_component]
pub fn Producer() -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();
    let msg_ctx_clone = msg_ctx.clone();
    use_effect_once(|| {
        info!("This code only runs once after page loads");
        // let result_w =  window().unwrap().match_media("(prefers-color-scheme: dark)").ok().unwrap().unwrap();
        // msg_ctx_clone.dispatch(result_w.matches());

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
           <input id="switchExample" type="checkbox" name="switchExample" class={classes!("switch")} onclick={onclick }/>
             <label for="switchExample">{"Dark Mode"}</label>
            </div>
            </>
    }
}

// /// `use_effect_once` demo
// #[function_component(MyComponent)]
// fn my_component() -> Html {
//     use_effect_once(|| {
//         info!("Running effect once on mount");
//
//         || alert("Running clean-up of effect on unmount")
//     });
//
//     html! {
//         <>{ "My Component" }</>
//     }
// }

#[function_component(UseEffectOnce)]
pub fn effect_once() -> Html {
    let toggle = use_toggle("Mount", "Unmount");
    let toggle = toggle.clone();
    toggle.toggle();
    // let onclick = {
    //     let toggle = toggle.clone();
    //     Callback::from(move |_: _| toggle.toggle())
    // };

    html! {
        <div class="app">
            // <header class="app-header">
            //     <div>
            //         <button {onclick}>{ *toggle }</button>
            //         <p>
            //             {
            //                 if *toggle == "Unmount" {
            //                     html! { <MyComponent /> }
            //                 } else {
            //                     html! {}
            //                 }
            //             }
            //         </p>
            //     </div>
            // </header>
        </div>
    }
}

//Possible implementation using structure components. Unfortunately I don't think it's worth it going all the way to Component structs since there is
//a whole bunch Hook yew boilerplate code one has to write every time to iterate through states...
// use crate::msg_ctx::MessageContext;
//
//
// pub enum Msg {
//     ToggleNavbar,
//     DarkMode,
//     Noop,
// }
//
// /// Main is a producer of messages. This pattern is extremely useful for things like a global
// /// state(dark mode). https://github.com/thebigG/thebigg.dev/issues/1
// pub struct Producer {
//     dark_mode: bool,
//     msg_ctx: MessageContext
// }
//
// impl Component for Producer {
//     type Message = ();
//     type Properties = ();
//
//     fn create(ctx: &yew::Context<Self>) -> Self {
//         // let onclick = Callback::from(move |_| msg_ctx.dispatch("Message Received.".to_string()));
//
//         // ctx.link()
//         //     let (message, _) = ctx
//         // .link()
//         // .context(ctx.link().context(|_|{MessageContext::from()}))
//         // .expect("No Message Context Provided");
//         // let (msg_ctx, mut msg_ctx_handle) = ctx
//         //     .link()
//         //     .context(Callback::from(move |ctx: MessageContext| {
//         //         info!("Hello callback {}", "callback ");
//         //         // ctx.inner = true;
//         //         // let mut msg_ctx = ctx.clone();
//         //
//         //         // ctx.reduce().inner = true;
//         //
//         //     }))
//         //     .unwrap();
//
//             let (msg_ctx, mut msg_ctx_handle) = ctx
//             .link()
//             .context(Callback::from(move |ctx: MessageContext| {
//                 info!("Hello callback {}", "callback ");
//                 // ctx.inner = true;
//                 // let mut msg_ctx = ctx.clone();
//
//                 // ctx.reduce().inner = true;
//                 // ctx.dispatch(ctx.inner);
//
//             }))
//             .unwrap();
//         Self { dark_mode: false, msg_ctx}
//
//
//     }
//
//     fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             // Msg::ToggleNavbar => {
//             //     info!("ToggleNavbar*********");
//             //     true
//             // },
//             // Msg::DarkMode => {
//             //     info!("DarkMode*********");
//             //     // self.update();
//             //     // ctx.link().route();
//             //     // ctx.link().send_message(Msg::DarkMode);
//             //     self.dark_mode = !self.dark_mode;
//             //     true
//             // }
//             // Msg::Noop =>
//             //     {
//             //         info!(" Msg::Noop*********");
//             //         true
//             //     },
//              _ => {
//                  info!(" Msg::Noop*********");
//                  true
//              }
//         }
//     }
//
//     fn view(&self, ctx: &yew::Context<Self>) -> Html {
//         /// Create a trait called DarkComponent.
//         /// Make all of your components(including things like your footer tag) DarkComponents
//         /// I think this might make things easier to manage...maybe.
//         let dark_class = if !self.dark_mode { "dark-mode" } else { "" };
//
//
//
//         // let (msg_ctx, mut msg_ctx_handle) = ctx
//         //     .link()
//         //     .context(Callback::from(move |ctx: MessageContext| {
//         //         info!("Hello callback {}", "callback ");
//         //         // ctx.inner = true;
//         //         // let mut msg_ctx = ctx.clone();
//         //
//         //         // ctx.reduce().inner = true;
//         //         // ctx.dispatch(ctx.inner);
//         //
//         //     }))
//         //     .unwrap();
//
//
//         let (msg_ctx, msg_ctx_handle) = ctx
//             .link().clone()
//             .context(Callback::from(move |ctx: MessageContext| {
//                 info!("Hello callback {}", "callback ");
//             }))
//             .unwrap();
//
//         // let (msg_ctx, handle) = ctx.link().context(yew::Callback({})).unwrap();
//
//         // let msg_ctx = ::yew::functional::Hook::run(
//         //     use_context::<MessageContext>(),
//         //     _ctx,
//         // )
//         // .unwrap();
//
//
//         let onclick = Callback::from(move |_| {
//             // info!("Hello start {}", msg_ctx.inner.with_subscriber());
//             // if msg_ctx.inner {
//             //      // let owned_msg_ctx = msg_ctx.clone() ;
//             //     info!("Hello false {}", msg_ctx.inner);
//             // } else {
//             //     info!("Hello true {}", msg_ctx.inner);
//             // }
//             info!("Hello end {}", msg_ctx.inner);
//
//             // msg_ctx.
//             // msg_ctx_handle.
//             // Callback::from(|_|msg_ctx.dispatch(!msg_ctx.inner))
//
//             // msg_ctx.dispatch(!msg_ctx.inner)
//
//             *msg_ctx_handle.clone().dispatch(!msg_ctx.inner);
//             // Msg::Noop
//             // ctx.link().send_message(Msg::Noop)
//             // ctx.link().callback(|_| Msg::Noop)
//
//         });
//         info!("view:main component");
//         html! {
//          //    <>
//          // <div class="switch">
//          //   <input id="switchExample" type="checkbox" name="switchExample" class={classes!("switch")} checked={self.dark_mode} onclick={onclick_callback }/>
//          //     <label for="switchExample">{"Dark Mode"}</label>
//          //    </div>
//          //    </>
//             <button onclick={onclick}>
//             {"PRESS ME"}
//         </button>
//         }
//     }
// }
