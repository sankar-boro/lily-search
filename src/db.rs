use std::collections::HashMap;

pub struct DB {
    inner: HashMap<String, Vec<String>>
}

impl DB {
    pub fn new() -> Self {
        DB {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, val: Vec<String>) {
        self.inner.insert(key, val);
    }

    pub fn push(&mut self, key: &str, val: String) {
        let mut data: &mut Vec<String> = self.inner.get_mut(key).unwrap();
        data.push(val);
    }
}