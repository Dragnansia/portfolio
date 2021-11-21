use crate::lang::Language;
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
    pub name: String,
    pub desc: String,
    pub images: Vec<Image>,
    pub languages: Vec<Language>,
}

impl Project {
    pub fn new(name: &str, desc: &str, images: Vec<Image>, languages: Vec<Language>) -> Self {
        Self {
            name: name.to_string(),
            desc: desc.to_string(),
            images,
            languages,
        }
    }
}
