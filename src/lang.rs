use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Language {
    pub name: String,
    pub data: HashMap<String, String>,
}

impl Language {
    pub fn new(name: &str, data: HashMap<String, String>) -> Self {
        Self {
            name: name.to_string(),
            data,
        }
    }
}
