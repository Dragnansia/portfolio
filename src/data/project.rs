use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Project {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub images: Vec<String>,
}
