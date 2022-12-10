use std::collections::HashMap;

pub struct RustStorage {
    dict: HashMap<String, String>,
}

impl RustStorage {
    pub fn store(&mut self, key: &str, val: &str) {
        self.dict.insert(key.to_string(), val.to_string());
    }

    pub fn load(&self, key: &str) -> *const String {
        match self.dict.get(key) {
            None => std::ptr::null(),
            Some(val) => val,
        }
    }
}

pub fn create_rust_storage() -> Box<RustStorage> {
    Box::new(RustStorage { dict: HashMap::new() })
}
