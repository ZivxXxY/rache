use std::collections::HashMap;
use std::hash::Hash;

pub trait Cache<K, V> {
    fn set(&mut self, k: K, v: V);
    fn get(&mut self, k: &K) -> Option<&V>;

    // TODO:
    // fn start_updater(&mut self, dutation: i32);
    // fn set_with_expiry(&mut self, k: K, v: V, expr: i32);
    // fn set_with_override
}

pub struct Rache<K, V> {
    data: HashMap<K, V>,
}

impl<K: Hash + Eq, V> Rache<K, V> {
    pub fn new() -> Rache<K, V> {
        Rache {
            data: HashMap::new(),
        }
    }
}

impl<K: Hash + Eq, V> Rache<K, V> {
    pub fn set(&mut self, k: K, v: V) {
        self.data.insert(k, v);
    }

    pub fn get(&mut self, k: &K) -> Option<&V> {
        match self.data.get(k) {
            Some(v) => Some(v),
            None => None,
        }
    }
}
