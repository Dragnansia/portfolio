use portfolio::{image::Image, link::LinkIcon, project::Project};
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
        let links: Html = project
            .links
            .iter()
            .map(|link| match &link.icon {
                LinkIcon::Image(l) => {
                    html! {
                        <a href={ link.url.clone() } style="text-decoration: none;">
                            <img src={ l.clone() } alt="Image link" />
                        </a>
                    }
                }
                LinkIcon::FontAwesome(class) => {
                    let class = format!("font-size-20 {}", class);
                    html! {
                        <a href={ link.url.clone() } style="text-decoration: none;">
                            <icon style="color: #ddd" class={ class } />
                        </a>
                    }
                }
            })
            .collect();

        html! {
            <div class="text-center">
                <div class="mb-20">
                    <p class="text-center font-size-24 m-0 mb-5">{ project.name.clone() }</p>
                    <div>
                        { images(project.images.clone()) }
                    </div>
                    <div class="d-flex justify-content-center">
                        { links }
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
                <img src={ i.data.clone() } alt={ i.alt.clone() }/>
            }
        })
        .collect()
}
