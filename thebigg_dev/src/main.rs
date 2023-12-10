use compile_time_run::{run_command, run_command_str};
use gloo::utils::document;
use js_sys::Array;
use log::info;
use std::process::Command;
use wasm_bindgen::JsValue;
use web_sys::{Element, Node};
use yew::html::Scope;
use yew::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod msg_ctx;
// use log::info;
// use wasm_bindgen::JsValue;
use msg_ctx::MessageContext;
use msg_ctx::MessageProvider;
use pages::home::Home;
use pages::page_not_found::PageNotFound;
mod pages;
mod producer;
use producer::Producer;

#[derive(Properties, PartialEq, Default, Debug, Clone)]
pub struct Props {
    #[prop_or(true)]
    pub dark_mode: bool,
}

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/posts/:id")]
    Post { id: u64 },
    #[at("/posts")]
    Posts,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}
impl Props {
    pub fn new() -> Self {
        Self { dark_mode: true }
    }
}
pub enum Msg {
    ToggleNavbar,
    DarkMode,
    AddOne,
    MessageContextUpdated(MessageContext),
}

/// Main is a producer of messages. This pattern is extremely useful for things like a global
/// state(dark mode). https://github.com/thebigG/thebigg.dev/issues/1
pub struct Main {
    navbar_active: bool,
    dark_mode: bool,

    message: MessageContext,
    _context_listener: ContextHandle<MessageContext>,
}

impl Component for Main {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let (message, _context_listener) = ctx
            .link()
            .context(ctx.link().callback(Msg::MessageContextUpdated))
            .expect("No Message Context Provided");
        Self {
            navbar_active: false,
            dark_mode: false,
            message,
            _context_listener,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
            Msg::DarkMode => {
                self.dark_mode = !self.dark_mode;
                true
            }
            Msg::AddOne => true,
            Msg::MessageContextUpdated(message) => {
                self.message = message;
                self.dark_mode = self.message.inner;
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let dark_mode = if self.dark_mode {
            "has-background-dark"
        } else {
            "is-primary"
        };

        let dark_mode_text = if self.dark_mode { "has-text-light" } else { "" };

        let dark_mode_link_hover_text = if self.dark_mode { "dark-mode-link" } else { "" };

        self.toggle_dark_mode_for_root_node();

        html! {
        <div class={classes!(dark_mode, "is-fullheight")}>
        <BrowserRouter>

        //To make life easy, always wrap these html chunks into functions. Code looks better like that too anyway.
        { self.view_nav(ctx.link()) }
                    <main>
                <Switch<Route> render={switch} />
        </main>
            <footer class={classes!("footer", dark_mode, dark_mode_text, "is-primary")}>
                <div class="has-text-centered">
                    <a href="https://github.com/thebigG">
                    <img src="images/icons8-github.svg" alt="github" style="width:32px;height:32px;"/></a>
                    <a href="https://www.linkedin.com/in/gomezlorenzo/">
                    <img src="images/icons8-linkedin-circled.svg" alt="linkedin" style="width:32px;height:32px;"/></a>
                </div>
                <div class={classes!("content", "has-text-centered", dark_mode, dark_mode_text)}>
                    { "Powered by " }
                    <a class={classes!(dark_mode_link_hover_text)}  href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a href="https://bulma.io">
                    <img src="images/bulma.png" alt="Bulma" style="width:32px;height:32px;"/></a>
                </div>
                    <div class={classes!("has-text-centered", dark_mode, dark_mode_text, dark_mode_link_hover_text)}>
                <a class={classes!(dark_mode_link_hover_text)}  href={format!("https://github.com/thebigG/thebigg.dev/commit/{}",run_command_str!("git", "rev-parse", "--short", "HEAD") ) }>
                    { run_command_str!("git", "rev-parse", "--short", "HEAD") }</a>
                </div>
            </footer>

        </BrowserRouter>
        </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }
}

impl Main {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let dark_mode = if self.dark_mode {
            "is-dark"
        } else {
            "is-primary"
        };

        let active_class = if !navbar_active { "is-active" } else { "" };
        info!("view_nav:main component");

        ///For some reason "is-dark" class is not working even though it is in Bulma...

        html! {

            <div class={classes!(dark_mode)}>
            <nav class={classes!("navbar", "mb-3", dark_mode)}  role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "thebigg.dev" }</h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Posts}>
                            { "Posts" }
                        </Link<Route>>

                        <div class="navbar-item has-dropdown is-hoverable">
                            <div class="navbar-link">
                                { "More" }
                            </div>

                        </div>

                    </div>
                <div class="navbar-end">
                    <div class="navbar-item">

             {Self::get_producer()}
                    </div>
                </div>
                </div>
            </nav>
            </div>


        }
    }

    /// In this case, the Producer is simply the toggle that triggers dark mode.
    fn get_producer() -> Html {
        html! {
            <Producer/>
        }
    }

    fn toggle_dark_mode_for_root_node(&self) {
        let dark_class_background = "has-background-dark";
        let root: Element = document().document_element().unwrap();
        let classes: Array = Array::new();

        let dark_class_js: JsValue = JsValue::from_str(dark_class_background);

        if self.dark_mode {
            classes.push(&dark_class_js);
            root.class_list().add(&classes);
        } else {
            root.class_list().remove_1(&dark_class_background);
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes.clone() {
        Route::Home {} => {
            html! { <Home dark_mode=true/> }
        }

        Route::NotFound => {
            html! {  <PageNotFound/>}
        }

        Route::Post { id } => {
            html! { <h1>{format!("Under construction...:{id}")}</h1>}
        }

        Route::Posts => {
            html! { <h1>{"Under construction..."}</h1>}
        }
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <MessageProvider>
            <Main/>
        </MessageProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    // let object = JsValue::from("world");
    // info!("Hello {}", object.as_string().unwrap());
    yew::Renderer::<App>::new().render();
}
