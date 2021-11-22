use crate::{html::render_html_file, lang::Language};
use mongodb::Database;
use rocket::{futures::TryStreamExt, response::Redirect, State};
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Image {
    pub desc: String,
    pub url: String,
}

impl Image {
    pub fn new(desc: &str, url: &str) -> Self {
        Self {
            desc: desc.to_string(),
            url: url.to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Project {
    pub title: String,
    pub desc: String,
    pub images: Vec<Image>,
    pub languages: Vec<Language>,
}

impl Project {
    pub fn new(name: &str, desc: &str, images: Vec<Image>, languages: Vec<Language>) -> Self {
        Self {
            title: name.to_string(),
            desc: desc.to_string(),
            images,
            languages,
        }
    }
}

pub async fn init_database_project(database: &Database) -> Option<Vec<Project>> {
    let collections = database.collection::<Project>("Projects");
    let lists = collections.find(None, None).await.ok()?;

    let projects = lists.try_collect().await.ok()?;
    Some(projects)
}

#[get("/project/<name>")]
pub fn proj(projects: &State<Vec<Project>>, name: &str) -> Result<Template, Redirect> {
    // Todo: Render page 404 or a new page for project not found ?
    // check for the better options but actually just redirect to 404 page
    let res = projects.iter().rfind(|p| p.title == name);
    if res.is_none() {
        Err(Redirect::to("/404"))
    } else {
        let proj = res.unwrap().clone();
        Ok(render_html_file("proj", Some(proj)))
    }
}
