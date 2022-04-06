use crate::{image::Image, link::Link};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Project {
    #[serde(rename = "_id")]
    pub id: ObjectId,
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
            id: ObjectId::new(),
            name: "P1".into(),
            ..Default::default()
        };

        let p2 = Project {
            id: ObjectId::new(),
            name: "P2".into(),
            ..Default::default()
        };

        assert_ne!(p1, p2);
    }

    #[test]
    fn deserialize() {
        let json = r#"
            {
                "_id": ObjectId("624d94cc39e3ee54af9351e8"),
                "name": "Json Project",
                "description": "",
                "images": [],
                "links": []
            }
        "#;

        let project = serde_json::from_str::<Project>(json).unwrap();

        assert_eq!(project.name, "Json Project".to_string());
    }

    #[test]
    fn serialize() {
        let project = Project {
            id: ObjectId::new(),
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
