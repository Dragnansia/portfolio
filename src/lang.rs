use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Language {
    pub id: String,
    pub data: HashMap<String, String>,
}

impl Language {
    pub fn new(id: String, data: HashMap<String, String>) -> Self {
        Self { id, data }
    }

    pub fn get_data(&self) -> &HashMap<String, String> {
        &self.data
    }
}
