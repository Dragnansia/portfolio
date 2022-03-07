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
                <div class="navbar-content" />

                <div class="navbar-brand">
                    <ul class="navbar-nav hidden-sm-and-down">
                        <li class="nav-item">
                            <but onclick={ home } class="nav-link">{ "Accueil" }</but>
                        </li>
                        <li class="nav-item">
                            <but onclick={ information } class="nav-link">{ "À Propos de moi" }</but>
                        </li>
                        <li class="nav-item">
                            <but onclick={ projects } class="nav-link">{ "Projets" }</but>
                        </li>
                    </ul>
                </div>

                <div class="navbar-content" />
            </nav>
        }
    }
}
