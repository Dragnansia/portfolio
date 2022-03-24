use portfolio::project::Project;
use yew::prelude::*;

/// Display project on HTML
pub fn view(project: Option<Project>) -> Html {
    html! {
        <div class="modal modal-full ie-scroll-fix" id="modal" tabindex="-1" role="dialog">
            <div class="modal-dialog" role="document">
                <div class="modal-content h-full">
                    <a href="#" class="close" role="button" aria-label="Close">
                        <span aria-hidden="true"></span>
                    </a>

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
                            <a target="_blank" href="https://github.com/Dragnansia"><i style="color: #ddd" class="font-size-20 fa fa-github"></i></a>
                        </div>
                    </div>
                </section>

                <section id="information">
                    { project.description.clone() }
                </section>

                <section id="photos">

                </section>
            </>
        }
    } else {
        html! {
            <div class="lds-dual-ring" />
        }
    }
}
