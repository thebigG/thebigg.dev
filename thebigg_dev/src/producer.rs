use log::info;
use yew::html::Scope;
use yew::prelude::*;
use yew::props;

use crate::msg_ctx::MessageContext;

#[derive(Properties, PartialEq, Default, Debug, Clone)]
pub struct Props {
    #[prop_or(true)]
    pub dark_mode: bool,
}

impl Props {
    pub fn new() -> Self {
        Self { dark_mode: true }
    }
}

pub enum Msg {
    ToggleNavbar,
    DarkMode,
    Noop,
}

/// Main is a producer of messages. This pattern is extremely useful for things like a global
/// state(dark mode). https://github.com/thebigG/thebigg.dev/issues/1
pub struct Producer {
    dark_mode: bool,
}

impl Component for Producer {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        // let onclick = Callback::from(move |_| msg_ctx.dispatch("Message Received.".to_string()));

        // ctx.link()
        //     let (message, _) = ctx
        // .link()
        // .context(ctx.link().context(|_|{MessageContext::from()}))
        // .expect("No Message Context Provided");
        Self { dark_mode: false }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => true,
            Msg::DarkMode => {
                info!("DarkMode*********");
                // self.update();
                // ctx.link().route();
                // ctx.link().send_message(Msg::DarkMode);
                self.dark_mode = !self.dark_mode;
                true
            }
            Msg::Noop => true,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        /// Create a trait called DarkComponent.
        /// Make all of your components(including things like your footer tag) DarkComponents
        /// I think this might make things easier to manage...maybe.
        let dark_class = if !self.dark_mode { "dark-mode" } else { "" };

        let (msg_ctx, msg_ctx_handle) = ctx
            .link()
            .context(Callback::from(move |ctx: MessageContext| {
                info!("Hello callback {}", "callback ");
                // ctx.dispatch(!ctx.inner)
                // ctx.
            }))
            .unwrap();

        // let (msg_ctx, handle) = ctx.link().context(yew::Callback({})).unwrap();

        let onclick_callback = Callback::from(move |_| {
            info!("Hello start {}", msg_ctx.inner);
            if msg_ctx.inner {
                // msg_ctx.inner  = false;
                info!("Hello false {}", msg_ctx.inner);
            } else {
                info!("Hello true {}", msg_ctx.inner);
                // msg_ctx.inner  = true;
            }
            info!("Hello end {}", msg_ctx.inner);
            // msg_ctx.inner;
            msg_ctx.dispatch(!msg_ctx.inner);
            // |_| Msg::DarkMode
        });
        info!("view:main component");
        html! {
            <>
         <div class="switch">
           <input id="switchExample" type="checkbox" name="switchExample" class={classes!("switch")} checked={self.dark_mode} onclick={onclick_callback }/>
             <label for="switchExample">{"Dark Mode"}</label>
            </div>
            </>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }
}

impl Producer {}

// #[function_component]
// pub fn Producer() -> Html {
//     let msg_ctx = use_context::<MessageContext>().unwrap();
//
//
//     let dark_mode  = true;
//
//     let onclick_callback = Callback::from(move |_| {
//         info!("Hello {}", msg_ctx.inner);
//         msg_ctx.dispatch(!msg_ctx.inner)
//     });
//
//         html! {
//             <>
//          <div class="switch">
//            <input id="switchExample" type="checkbox" name="switchExample" class={classes!("switch")} checked={!dark_mode} onclick={onclick_callback }/>
//              <label for="switchExample">{"Dark Mode"}</label>
//             </div>
//             </>
//         }
// }
