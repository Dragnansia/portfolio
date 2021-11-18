use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Language {
    data: HashMap<&'static str, Vec<&'static str>>,
}

impl Language {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add_lg(&mut self, lg: &'static str, data: Vec<&'static str>) -> &Self {
        self.data.insert(lg, data);
        self
    }

    pub fn add_many_lg(&mut self, lgs: &[(&'static str, Vec<&'static str>)]) -> &Self {
        self.data.extend(lgs.iter().map(|lg| lg.clone()));
        self
    }
}
