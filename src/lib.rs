

use std::collections::HashMap;

///We use hasmap to make a kv store
pub struct KvStore{
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore { map: HashMap::new() }
    }
    pub fn set(&mut self,par1: String,par2: String) {
        self.map.insert(par1, par2);
    }
    pub fn get(&self,par1: String) -> Option<String> {
        self.map.get(&par1).cloned()
    }
    pub fn remove(&mut self,par1: String) {
        self.map.remove(&par1);
    }
}




