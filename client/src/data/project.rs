use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Project {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub images: Vec<String>,
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.title == other.title
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compare() {
        let p1 = Project {
            id: 1000,
            title: "P1".into(),
            ..Default::default()
        };

        let p2 = Project {
            id: 1001,
            title: "P2".into(),
            ..Default::default()
        };

        assert_ne!(p1, p2);
    }

    #[test]
    fn deserialize() {
        let json = r#"
            {
                "id": 1000,
                "title": "Json Project",
                "description": "",
                "images": []
            }
        "#;

        let project = serde_json::from_str::<Project>(json).unwrap();

        assert_eq!(project.id, 1000u64);
    }

    #[test]
    fn serialize() {
        let project = Project {
            id: 1000,
            title: "Project".into(),
            ..Default::default()
        };

        let serialize = serde_json::to_string(&project).unwrap();

        assert_eq!(
            serialize,
            r#"{"id":1000,"title":"Project","description":"","images":[]}"#
        );
    }
}
