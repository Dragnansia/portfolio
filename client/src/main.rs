mod bindings;
mod component;
mod page;

use component::navbar::NavBar;
use page::home::Home;
use wasm_logger::Config;
use yew::prelude::*;

struct Client;
impl Component for Client {
    type Message = ();
    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &yew::Context<Self>) -> yew::Html {
        html! {
            <>
                <div class="page-wrapper with-navbar">
                    <NavBar />
                    <Home />
                </div>
            </>
        }
    }
}

fn main() {
    wasm_logger::init(Config::default());
    yew::start_app::<Client>();
}
