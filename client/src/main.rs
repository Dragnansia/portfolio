mod bindings;
mod component;
mod page;

use component::navbar::NavBar;
use page::{error, home::Home, project};
use wasm_logger::Config;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/project/:id")]
    Project { id: u64 },

    #[at("/404")]
    NotFound,
}

struct Client;
impl Component for Client {
    type Message = ();
    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        html! {
            <BrowserRouter>
                <div class="page-wrapper with-navbar">
                    <Switch<Route> render={ Switch::render(switch) } />
                </div>
            </BrowserRouter>
        }
    }
}

fn switch(routes: &Route) -> Html {
    println!("fb gjsqgbsdfg");
    match routes {
        Route::Home => html! {
            <>
                <NavBar />
                <Home />
            </>
        },
        Route::Project { id } => project::view(*id),
        Route::NotFound => error::err_404(),
    }
}

fn main() {
    wasm_logger::init(Config::default());
    yew::start_app::<Client>();
}
