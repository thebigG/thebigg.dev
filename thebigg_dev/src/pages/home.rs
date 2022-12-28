use log::info;
use std::thread::Builder;

use yew::prelude::*;

use crate::msg_ctx::MessageContext;
use yew::html::Buildable;

pub enum Msg {
    AddOne,
    DarkMode,
    MessageContextUpdated(MessageContext),
}

#[derive(Properties, PartialEq, Default, Debug, Clone)]
pub struct Props {
    #[prop_or(true)]
    pub dark_mode: bool,
}

pub struct Home {
    value: i64,
    dark_mode: bool,
    message: MessageContext,
    _context_listener: ContextHandle<MessageContext>,
}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let (message, _context_listener) = ctx
            .link()
            .context(ctx.link().callback(Msg::MessageContextUpdated))
            .expect("No Message Context Provided");
        Self {
            value: 0,
            dark_mode: false,
            message,
            _context_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::DarkMode => true,
            Msg::MessageContextUpdated(message) => {
                self.dark_mode = message.inner;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        info!("Hello from Home view{}", 101);
        let dark_mode = if self.dark_mode {
            "is-dark"
        } else {
            "is-primary"
        };
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
               <h1 >
                {"Welcome To My Site"}
               </h1>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        info!("Hello from Home changed  {}", 101);
        println!("Hello from changed ");
        true
    }
}
