use std::collections::HashMap;

pub struct AccessCache {
    cache: HashMap<String, bool>,  // Cache of user access to resources
}

impl AccessCache {
    pub fn new() -> Self {
        AccessCache {
            cache: HashMap::new(),
        }
    }

    pub fn check_cache(&self, key: &str) -> Option<&bool> {
        self.cache.get(key)
    }

    pub fn add_to_cache(&mut self, key: &str, access_granted: bool) {
        self.cache.insert(key.to_string(), access_granted);
    }
}
