use portfolio::project::{Image, Project};
use yew::prelude::*;

/// Display project on HTML
pub fn view(project: Option<Project>) -> Html {
    html! {
        <div class="modal modal-full ie-scroll-fix" id="modal" tabindex="-1" role="dialog">
            <div class="modal-dialog" role="document">
                <div class="modal-content h-full">
                    <button class="close bg-danger" data-dismiss="modal" type="button" aria-label="Close" />

                    <div class="container h-full">
                        <div class="row align-middle">
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
            <div class="text-center">
                <div class="mb-20">
                    <p class="text-center font-size-24 m-0 mb-5">{ project.name.clone() }</p>
                    <div>
                        { images(project.images.clone()) }
                    </div>
                    <div class="d-flex justify-content-center">
                        <a href="https://github.com/Dragnansia"><i style="color: #ddd" class="font-size-20 fa fa-github"></i></a>
                    </div>
                </div>

                { project.description.clone() }
            </div>
        }
    } else {
        html! {}
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
