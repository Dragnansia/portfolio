use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Link {
    pub url: String,
    pub icon: LinkIcon,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LinkIcon {
    Image(String),
    FontAwesome(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize() {
        let link = Link {
            icon: LinkIcon::Image("link image".into()),
            url: "".into(),
        };

        let serialize = serde_json::to_string(&link).unwrap();

        assert_eq!(serialize, r#"{"url":"","icon":{"Image":"link image"}}"#);
    }
}
