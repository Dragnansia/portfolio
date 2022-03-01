//! All error page
use yew::prelude::*;

pub fn err_404() -> Html {
    html! {
        <div class="page-wrapper">
            <div class="snap-container">
                <section id="project">
                    <div>
                        <img src="/img/404.png" alt="404 Error" class="m-5 w-500 p-0" />
                        <div></div>
                        <div class="d-flex justify-content-center">
                            { "Page not found" }
                        </div>
                    </div>
                </section>
            </div>
        </div>
    }
}
