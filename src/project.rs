use crate::lang::Language;

#[derive(Clone, Debug)]
pub struct Image {
    pub desc: &'static str,
    pub url: &'static str,
}

impl Image {
    pub fn new(desc: &'static str, url: &'static str) -> Self {
        Self { desc, url }
    }
}

#[derive(Clone, Debug)]
pub struct Project {
    pub name: &'static str,
    pub desc: &'static str,
    pub images: Vec<Image>,
    pub languages: Vec<Language>,
}

impl Project {
    pub fn new(
        name: &'static str,
        desc: &'static str,
        images: Vec<Image>,
        languages: Vec<Language>,
    ) -> Self {
        Self {
            name,
            desc,
            images,
            languages,
        }
    }
}
