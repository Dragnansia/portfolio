use crate::component::circle_anim::CircleAnim;
use portfolio::project::Project;
use reqwasm::http::Request;
use yew::prelude::*;

#[function_component(Home)]
pub fn view() -> Html {
    let projects: UseStateHandle<Vec<Project>> = use_state(|| vec![]);

    {
        let projects = projects.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetch_projects: Vec<Project> = Request::get("/projects")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    projects.set(fetch_projects);
                });

                || ()
            },
            (),
        );
    }

    html! {
        <div class="snap-container">
            <section id="home">
                <CircleAnim />

                { info() }
            </section>

            <section id="information">
                { description() }
            </section>

            <section id="projects">
                <ProjectsList projects={ (*projects).clone() } />
            </section>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ProjectsListProps {
    projects: Vec<Project>,
}

#[function_component(ProjectsList)]
fn projects(ProjectsListProps { projects }: &ProjectsListProps) -> Html {
    projects.iter().map(|project| {
            let url = format!("/project/{}", project.id);
            let title = project.name.clone();
            let image = match project.images.get(0) {
                Some(s) => s.clone(),
                None => Default::default()
            };

            html! {
                <div class="d-flex flex-wrap justify-content-center" data-toggle="tooltip" data-title={ title } style="overflow: unset;">
                    <a href={ url } class="m-5 w-200 p-0">
                        <img class="img-fluid rounded" alt="" src={ image.url.clone() } />
                    </a>
                </div>
            }
        }).collect()
}

fn info() -> Html {
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

fn description() -> Html {
    html! {
        { "This is my full description for my portfolio" }
    }
}
