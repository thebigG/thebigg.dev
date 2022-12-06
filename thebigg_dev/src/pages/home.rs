use log::info;
use std::thread::Builder;

use yew::prelude::*;

use crate::Props;
use yew::html::Buildable;

pub enum Msg {
    AddOne,
}

// impl Builder for Home{

// }

pub struct Home {
    value: i64,
}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                info!("Hello from Home update  {}", 101);
                // the value has changed so we need to
                // re-render for it to appear on the page
                // Figure a way to print in yew: https://yew.rs/docs/more/debugging
                // println!("{}", _ctx.props().active);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        info!("Hello from Home view{}", 101);
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
               <h1>
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
