use crate::{image::Image, link::Link};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Project {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub images: Vec<Image>,
    pub links: Vec<Link>,
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compare() {
        let p1 = Project {
            id: 1000,
            name: "P1".into(),
            ..Default::default()
        };

        let p2 = Project {
            id: 1001,
            name: "P2".into(),
            ..Default::default()
        };

        assert_ne!(p1, p2);
    }

    #[test]
    fn deserialize() {
        let json = r#"
            {
                "id": 1000,
                "name": "Json Project",
                "description": "",
                "images": [],
                "links": []
            }
        "#;

        let project = serde_json::from_str::<Project>(json).unwrap();

        assert_eq!(project.id, 1000u64);
    }

    #[test]
    fn serialize() {
        let project = Project {
            id: 1000,
            name: "Project".into(),
            ..Default::default()
        };

        let serialize = serde_json::to_string(&project).unwrap();

        assert_eq!(
            serialize,
            r#"{"id":1000,"name":"Project","description":"","images":[],"links":[]}"#
        );
    }
}
