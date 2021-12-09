use crate::{html::render_html_file, lang::Language};
use mongodb::{bson::doc, Collection, Database};
use rocket::{
    futures::{TryFutureExt, TryStreamExt},
    response::Redirect,
    State,
};
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub fn new(title: String, desc: String, images: Vec<Image>, languages: Vec<Language>) -> Self {
        Self {
            title,
            desc,
            images,
            languages,
        }
    }

    pub fn language(&self, lang: &str) -> Option<&HashMap<String, String>> {
        let lg = self.languages.iter().find(|lg| lg.id == lang);
        match lg {
            Some(l) => Some(l.get_data()),
            None => None,
        }
    }
}

pub async fn find_project(collection: Collection<Project>, project_name: &str) -> Option<Project> {
    let res = collection
        .find(doc! {"title": project_name}, None)
        .await
        .ok()?;
    let collect: Vec<Project> = res.try_collect().await.ok()?;
    Some(collect.first().unwrap().clone())
}

pub async fn all_projects(collection: Collection<Project>) -> Option<Vec<Project>> {
    let res = collection.find(None, None).await.ok()?;
    let collect: Option<Vec<Project>> = res.try_collect().await.ok();
    collect
}

#[get("/project/<name>?<lang>")]
pub async fn proj(
    db: &State<Database>,
    name: &str,
    lang: Option<&str>,
) -> Result<Template, Redirect> {
    let proj = find_project(db.collection::<Project>("Projects"), name).await;

    if proj.is_none() {
        Err(Redirect::to("/404"))
    } else {
        let p = proj.unwrap();
        let l = p.language(lang.unwrap_or("fr"));

        Ok(render_html_file("proj", Some(p)))
    }
}
