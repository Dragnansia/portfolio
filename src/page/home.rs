use crate::{component::circle_anim::CircleAnim, data::project::Project};
use yew::prelude::*;

pub struct Home {
    projects: Vec<Project>,
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            projects: vec![Project {
                id: 7868937628793,
                title: "qsdfqsdfqsdf".into(),
                description: "qsdfqsdfqsdf".into(),
                images: vec![],
            }],
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div class="snap-container">
                <section id="home">
                    <CircleAnim />

                    { self.info() }
                </section>

                <section id="info">
                    { self.description() }
                </section>

                <section id="projects">
                    { self.projects() }
                </section>
            </div>
        }
    }
}

impl Home {
    fn projects(&self) -> Vec<Html> {
        self.projects.iter().map(|project| {
            let url = format!("/project/{}", project.id);
            let title = project.title.clone();
            let image = match project.images.get(0) {
                Some(s) => s.clone(),
                None => String::new()
            };

            html! {
                <div class="d-flex flex-wrap justify-content-center">
                    <a href={ url } data-toggle="tooltip" data-title={ title } class="m-5 w-200 p-0">
                        <img src={ image.clone() } class="img-fluid rounded" alt="" />
                    </a>
                </div>
            }
        }).collect()
    }

    fn info(&self) -> Html {
        html! {
            <div>
                <p class="text-center font-size-24 mb-5">{ "Romuald Aucouturier" }</p>
                <div></div>
                <p class="text-center font-size-16 m-0 mb-5"> { "Software, video game Dev" } </p>
                <div class="d-flex justify-content-center">
                <a href="https://github.com/Dragnansia"><i style="color: #ddd;" class="font-size-20 fa fa-github-alt"></i></a>
                <a href="mailto:romuald.auc.pro@protonmail.com" class="ml-10"><i style="color: #ddd;" class="font-size-20 fa fa-inbox"></i></a>
                </div>
            </div>
        }
    }

    fn description(&self) -> Html {
        html! {
            { "This is my full description for my portfolio" }
        }
    }
}
