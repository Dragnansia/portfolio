mod bindings;
mod component;
mod data;
mod page;

use component::navbar::NavBar;
use page::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("project/:id")]
    Project { id: u64 },

    #[at("/404")]
    NotFound,
}

struct App;
impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        html! {
            <BrowserRouter>
                <div class="page-wrapper">

                    <Switch<Route> render={ Switch::render(switch) } />
                </div>
            </BrowserRouter>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Project { id } => todo!(),
        Route::NotFound => todo!(),
    }
}

fn main() {
    yew::start_app::<App>();
}
