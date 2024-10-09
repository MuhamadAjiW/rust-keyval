use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct Store {
    map: Arc<Mutex<HashMap<String, String>>>,
}

impl Store {
    pub fn new() -> Self {
        return Store {
            map: Arc::new(Mutex::new(HashMap::new())),
        };
    }

    pub fn set(&mut self, key: &str, value: &str) {
        let mut map = self.map.lock().unwrap();
        map.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> String {
        let map = self.map.lock().unwrap();
        return map.get(key).cloned().unwrap_or("".to_string());
    }

    pub fn remove(&mut self, key: &str) -> String {
        let mut map = self.map.lock().unwrap();
        return map.remove(key).unwrap_or("".to_string());
    }
}
