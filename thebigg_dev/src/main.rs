use yew::html::Scope;
use yew::prelude::*;
use yew_router::prelude::*;

use pages::home;
use pages::page_not_found::PageNotFound;
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
        /// Create a trait called DarkComponent.
        /// Make all of your components(including things like your footer tag) DarkComponents
        /// I think this might make things easier to manage...maybe.
        let dark_class = if !true { "dark-mode" } else { "" };
        html! {
        <div class="">
        <BrowserRouter>
        { self.view_nav(ctx.link()) }

            <main>
                <Switch<Route> render={switch} />
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
        </div>
        }
    }
}

impl Model {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <div>
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
            <div class="switch">
              <input id="switchExample" type="checkbox" name="switchExample" class={classes!("switch")} checked=false/>
                <label for="switchExample">{"Dark Mode"}</label>
            </div>
            </div>


        }
    }
}

fn switch(routes: Route) -> Html {
    match routes.clone() {
        Route::Home => {
            html! { <home::Home/> }
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
    yew::Renderer::<Model>::new().render();
}
