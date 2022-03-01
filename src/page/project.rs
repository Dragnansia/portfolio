use yew::prelude::*;

use crate::data::project::Project;

/// Display project on HTML
pub fn view(id: u64) -> Html {
    let project = Project {
        id,
        title: "lkjhdfgsdfhiusdfhgisdf".into(),
        description: "qmlodsimhglqodshgiqudfghiudhfgidbvjkhhdfiughiudfhgiudsfhishdfgiuhsdfighsdijfjfdhg hdfgiuhsdfiugh isudfhgiushd ihf guhs fi uh".into(),
        images: vec![],
    };

    html! {
        <div class="snap-container">
            <section id="project">
                <div>
                    <img src="" alt="Project Image" class="m-5 w-500 p-0" />
                    <p class="text-center font-size-16 m-0 mb-5">{ project.title.clone() }</p>
                    <div></div>
                    <div class="d-flex justify-content-center">
                        <a target="_blank" href="https://github.com/Dragnansia"><i style="color: #ddd" class="font-size-20 fa fa-github"></i></a>
                    </div>
                </div>
            </section>

            <section id="information">
                { project.description }
            </section>

            <section id="photos">

            </section>
        </div>
    }
}
