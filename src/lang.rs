use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Language {
    pub name: String,
    pub data: HashMap<String, String>,
}

impl Language {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            data: HashMap::new(),
        }
    }

    pub fn insert_with_format(&mut self, k: &str, v: &str) -> &Self {
        self.data
            .insert(k.to_string(), format!("<lg id=\"{}\">{}</lg>", k, v));
        self
    }
}
