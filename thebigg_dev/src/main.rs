use log::info;
use yew::html::Scope;
use yew::prelude::*;
use yew_router::prelude::*;

// use log::info;
// use wasm_bindgen::JsValue;
use pages::home::Home;
use pages::page_not_found::PageNotFound;
mod pages;

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
}

pub struct Model {
    navbar_active: bool,
    dark_mode: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            navbar_active: false,
            dark_mode: false,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
            Msg::DarkMode => {
                info!("DarkMode*********");
                // self.update();
                // ctx.link().route();
                // ctx.link().send_message(Msg::DarkMode);
                self.dark_mode = !self.dark_mode;
                true
            }
            Msg::AddOne => true,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        /// Create a trait called DarkComponent.
        /// Make all of your components(including things like your footer tag) DarkComponents
        /// I think this might make things easier to manage...maybe.
        let dark_class = if !self.dark_mode { "dark-mode" } else { "" };

        info!("view:main component");

        html! {
        <div>
        <BrowserRouter>

        //To make life easy, always wrap these html chunks into functions. Code looks better like that too anyway.
        { self.view_nav(ctx.link()) }
                    <main>
                <Switch<Route> render={switch} />
        </main>
            <footer class="footer">
                <div class="content has-text-centered is-dark">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a href="https://bulma.io">
                    <img src="images/bulma.png" alt="Bulma" style="width:32px;height:32px;"/></a>
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

impl Model {
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
            <nav class={classes!("navbar", dark_mode)}  role="navigation" aria-label="main navigation">
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

            {self.view_toggle(link)}
                    </div>
                </div>
                </div>
            </nav>
            </div>


        }
    }

    fn view_toggle(&self, link: &Scope<Self>) -> Html {
        let Self {
            navbar_active,
            dark_mode,
        } = *self;

        // let active_class = if !navbar_active { "is-active" } else { "" };

        // let dark_toggle = |_| Msg::DarkMode;
        //"click" event does not seem to work input tag at the moment...
        html! {
            <>
         <div class="switch">
           <input id="switchExample" type="checkbox" name="switchExample" class={classes!("switch")} checked={dark_mode} onclick={link.callback(|_| Msg::DarkMode)}/>
             <label for="switchExample">{"Dark Mode"}</label>
            </div>
            </>
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

fn main() {
    println!("main*********");
    wasm_logger::init(wasm_logger::Config::default());
    info!("Hello {}", 101);

    // let object = JsValue::from("world");
    // info!("Hello {}", object.as_string().unwrap());
    yew::Renderer::<Model>::new().render();
}
