use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Image {
    pub url: String,
    pub alt: String,
    pub base64: String,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            url: "/proxy-image.jpg".into(),
            alt: "No image for project".into(),
            base64: String::new(),
        }
    }
}
