use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Image {
    pub data: String,
    pub alt: String,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            data: "/proxy-image.jpg".into(),
            alt: "No image for project".into(),
        }
    }
}
