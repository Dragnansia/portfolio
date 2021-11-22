use crate::lang::Language;
use mongodb::Database;
use rocket::futures::TryStreamExt;
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

// TODO: get project and languages and need to remove typing
pub async fn init_database_project(database: &Database) -> Option<Vec<Project>> {
    let collections = database.collection::<Project>("Projects");
    let lists = collections.find(None, None).await.ok()?;

    let projects = lists.try_collect().await.ok()?;
    Some(projects)
}
