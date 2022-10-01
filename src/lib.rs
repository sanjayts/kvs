use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore {
    kv_store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set(&mut self, key: String, value: String) {
        // let _ = self.kv_store.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        // self.kv_store.get(&key).map(|v| v.to_owned())
        todo!()
    }

    pub fn remove(&mut self, key: String) -> Option<String> {
        // self.kv_store.remove(&key)
        todo!()
    }
}
