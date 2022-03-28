use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Link {
    pub url: String,
    pub icone: LinkIcone,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LinkIcone {
    Image(String),
    FontAwesome(String, String),
}
