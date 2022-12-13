use std::collections::HashMap;
use crate::ffi::{LoadFuture, StoreFuture, seastar_sleep_1s};

pub struct RustStorage {
    dict: HashMap<String, String>,
}

impl RustStorage {
    pub fn store(&mut self, key: &str, val: &str) -> StoreFuture {
        StoreFuture::infallible(async {
            seastar_sleep_1s().await;
            self.dict.insert(key.to_string(), val.to_string());
        })
    }

    pub fn load(&self, key: &str) -> LoadFuture {
        LoadFuture::fallible(async {
            seastar_sleep_1s().await;
            match self.dict.get(key) {
                None => Err(CxxAsyncException::new(format!("entry for {key} not found").to_owned().into_boxed_str())),
                Some(val) => Ok(val.clone()),
            }
        })
    }
}

pub fn create_rust_storage() -> Box<RustStorage> {
    Box::new(RustStorage { dict: HashMap::new() })
}
