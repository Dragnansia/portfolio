use crate::arguments::array_to_hashmap;
use crate::{
    html::render_html_file,
    page::project::{all_projects, Project},
};
use mongodb::Database;
use rocket::State;
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Home<T> {
    txt: T,
    projects: Vec<Project>,
}

#[get("/")]
pub async fn home(db: &State<Database>) -> Template {
    let projects = all_projects(db.collection::<Project>("Projects"))
        .await
        .unwrap_or_default();

    render_html_file(
        "home",
        Some(Home {
            txt: array_to_hashmap(&[("title", "Home")]),
            projects,
        }),
    )
}
