use crate::bindings;
use yew::prelude::*;

pub struct NavBar;
impl Component for NavBar {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let home = ctx.link().callback(|_| bindings::move_to("home"));
        let information = ctx.link().callback(|_| bindings::move_to("information"));
        let projects = ctx.link().callback(|_| bindings::move_to("projects"));

        html! {
            <nav class="navbar justify-content-between">
                <div class="navbar-content">
                    <ul class="navbar-nav hidden-sm-and-down">
                    <li class="nav-item">
                        <p class="font-size-16">{ "Romuald Aucouturier" }</p>
                    </li>
                    </ul>
                </div>

                <div class="navbar-brand"></div>

                <div class="navbar-content">
                    <ul class="navbar-nav hidden-sm-and-down">
                        <li class="nav-item">
                            <a href="#" onclick={ home } class="nav-link">{ "Home" }</a>
                        </li>
                        <li class="nav-item">
                            <a href="#" onclick={ information } class="nav-link">{ "Infos" }</a>
                        </li>
                        <li class="nav-item">
                            <a href="#" onclick={ projects } class="nav-link">{ "Projects" }</a>
                        </li>
                    </ul>
                </div>
            </nav>
        }
    }
}
