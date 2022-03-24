use portfolio::project::{Image, Project};
use yew::prelude::*;

/// Display project on HTML
pub fn view(project: Option<Project>) -> Html {
    html! {
        <div class="modal modal-full ie-scroll-fix" id="modal" tabindex="-1" role="dialog">
            <div class="modal-dialog" role="document">
                <div class="modal-content h-full">
                    <button class="close" data-dismiss="modal" type="button" aria-label="Close" />

                    <div class="container">
                        <div class="row">
                            <div class="col-md-8 offset-md-2">
                                <ProjectView project={ project } />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ProjectViewProps {
    project: Option<Project>,
}

#[function_component(ProjectView)]
pub fn project(ProjectViewProps { project }: &ProjectViewProps) -> Html {
    if let Some(project) = project {
        html! {
            <>
                <section id="project">
                    <div>
                        <img src="" alt="Project Image" class="m-5 w-500 p-0" />
                        <p class="text-center font-size-16 m-0 mb-5">{ project.name.clone() }</p>
                        <div></div>
                        <div class="d-flex justify-content-center">
                            <a href="https://github.com/Dragnansia"><i style="color: #ddd" class="font-size-20 fa fa-github"></i></a>
                        </div>
                    </div>
                </section>

                <section id="information">
                    { project.description.clone() }
                </section>

                <section id="photos">
                    { images(project.images.clone()) }
                </section>
            </>
        }
    } else {
        html! {
            <div class="lds-dual-ring" />
        }
    }
}

fn images(images: Vec<Image>) -> Html {
    images
        .iter()
        .map(|i| {
            html! {
                <img src={ i.url.clone() } alt={ i.alt.clone() }/>
            }
        })
        .collect()
}
