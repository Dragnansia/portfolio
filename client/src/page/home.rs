use crate::{bindings, component::circle_anim::CircleAnim, page::project};
use bson::oid::ObjectId;
use portfolio::project::Project;
use reqwasm::http::Request;
use yew::prelude::*;

pub enum Message {
    ViewProject(ObjectId),
    ProjectList(Vec<Project>),
}

pub struct Home {
    projects: Vec<Project>,
    project_focus: Option<Project>,
}

impl Component for Home {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            let fetch_projects: Vec<Project> = Request::get("/api/projects")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            Message::ProjectList(fetch_projects)
        });

        Self {
            projects: vec![],
            project_focus: None,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ViewProject(id) => {
                self.project_focus = self.projects.iter().find(|p| p.id == id).cloned();
            }
            Message::ProjectList(list) => {
                self.projects = list;
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let lists: Html = self
            .projects
            .iter()
            .map(|project| {
                let title = project.name.clone();
                let id = project.id;
                let onclick = ctx.link().callback(move |_| {
                    bindings::toggle_modal("modal");
                    Message::ViewProject(id)
                });

                html! {
                    <div class="d-flex flex-wrap justify-content-center" style="overflow: unset;">
                        <div class="card" style="max-width: 40rem;">
                            <h2 class="card-title">{ title }</h2>
                            <p class="text-muted">{ project.description.clone() }</p>
                            <div class="text-right">
                                <button onclick={ onclick } class="btn" type="button">{ "Lire plus" }</button>
                            </div>
                        </div>
                    </div>
                }
            })
            .collect();

        html! {
            <>
                <div class="snap-container">
                    <section id="home">
                        <CircleAnim />

                        { info() }
                    </section>

                    <section id="information">
                        { description() }
                    </section>

                    <section id="projects">
                        { lists }
                    </section>
                </div>

                { project::view(self.project_focus.clone()) }
            </>
        }
    }
}

fn info() -> Html {
    html! {
        <div>
            <p class="text-center font-size-24 mb-5"> { "Romuald Aucouturier" } </p>
            <div />
            <p class="text-center font-size-16 m-0 mb-5"> { "Software, video game Dev" } </p>
            <div class="d-flex justify-content-center">
                <a href="https://github.com/Dragnansia">
                    <i style="color: #ddd;" class="font-size-20 fa fa-github-alt" />
                </a>
                <a href="mailto:romuald.auc.pro@protonmail.com" class="ml-10">
                    <i style="color: #ddd;" class="font-size-20 fa fa-inbox" />
                </a>
            </div>
        </div>
    }
}

fn description() -> Html {
    html! {
        { "This is my full description for my portfolio" }
    }
}
