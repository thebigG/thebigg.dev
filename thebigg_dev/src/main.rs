use pages::home;
use pages::page_not_found::PageNotFound;
use yew::html::Scope;
use yew::prelude::*;
use yew::Component;
use yew_router::prelude::*;

mod pages;

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

pub enum Msg {
    ToggleNavbar,
}

pub struct Model {
    navbar_active: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
        <BrowserRouter>
        { self.view_nav(ctx.link()) }

            <main>
                <Switch<Route> render={Switch::render(switch)} />
            </main>
            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a href="https://bulma.io">
                    <img src="images/bulma.png" alt="Bulma" style="width:32px;height:32px;"/></a>
                </div>
            </footer>

        </BrowserRouter>
        }
    }
}

impl Model {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
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
                </div>
            </nav>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Home => {
            html! { <home::Home/> }
        }

        Route::NotFound => {
            html! {  <PageNotFound/>}
        }

        Route::Post { id } => {
            html! { <h1>{format!("Under consutruction...:{id}")}</h1>}
        }

        Route::Posts => {
            html! { <h1>{"Under consutruction..."}</h1>}
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
