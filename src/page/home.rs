use crate::{html::render_html_file, page::project::Project};
use rocket::State;
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Home<'a> {
    title: &'a str,
    projects: Vec<Project>,
}

#[get("/")]
pub fn home(projects: &State<Vec<Project>>) -> Template {
    render_html_file(
        "home",
        Some(Home {
            title: "Home",
            projects: projects.to_vec(),
        }),
    )
}
