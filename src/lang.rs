use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Language {
    pub name: &'static str,
    pub data: HashMap<String, String>,
}

impl Language {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            data: HashMap::new(),
        }
    }

    pub fn insert_with_format(&mut self, k: &str, v: &str) -> &Self {
        self.data
            .insert(k.to_string(), format!("<lg id=\"{}\">{}</lg>", k, v));
        self
    }
}
