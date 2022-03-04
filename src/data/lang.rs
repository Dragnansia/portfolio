use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Language {
    French,
    English,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LData {
    /// The associated project id
    id: u64,

    /// All data for this project with this language
    data: HashMap<String, String>,

    /// Language specification
    lang: Language,
}

impl LData {
    /// Check if language is the correct one
    pub fn is_this_lang(&self, id: u64, lang: Language) -> bool {
        self.id == id && self.lang == lang
    }

    /// Return data from name
    pub fn data(&self, name: &str) -> String {
        self.data[name].clone()
    }
}

impl PartialEq for LData {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.lang == other.lang
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn data_creation() {
        let l1 = LData {
            id: 10,
            data: HashMap::from([("A".into(), "B".into())]),
            lang: Language::English,
        };

        assert_eq!(l1.data("A"), "B");
    }

    #[test]
    fn chek_is_valid() {
        let l1 = LData {
            id: 10,
            data: HashMap::from([("A".into(), "B".into())]),
            lang: Language::English,
        };

        assert!(l1.is_this_lang(10, Language::English));
    }

    #[test]
    fn compare() {
        let l1 = LData {
            id: 10,
            data: HashMap::from([("A".into(), "B".into())]),
            lang: Language::English,
        };

        let l2 = LData {
            id: 10,
            data: HashMap::new(),
            lang: Language::French,
        };

        assert_ne!(l1, l2);
    }

    #[test]
    fn deserialize() {
        let json = r#"
            {
                "id": 1000,
                "data": {
                    "A": "B"
                },
                "lang": "French"
            }
        "#;

        let ldata = serde_json::from_str::<LData>(json).unwrap();

        assert!(ldata.is_this_lang(1000, Language::French));
    }
}
