use std::collections::HashMap;
use std::hash::Hash;

pub trait Cache<K, V> {
    fn set(&mut self, k: K, v: V);
    fn get(&mut self, k: &K) -> Option<&V>;
}

struct Entry<V> {
    value: V,
}

impl<V> Entry<V> {
    fn new(v: V) -> Self {
        Entry {
            value: v,
        }
    }
}

pub struct Rache<K, V> {
    data: HashMap<K, Entry<V>>,
}

impl<K: Hash + Eq, V> Rache<K, V> {
    pub fn new() -> Rache<K, V> {
        Rache {
            data: HashMap::new(),
        }
    }
}

impl <K: Hash + Eq, V> Rache<K, V> {
    pub fn set(&mut self, k: K, v: V) {
        let entry = Entry::new(v);
        self.data.insert(k, entry);
    }

    pub fn get(&mut self, k: &K) -> Option<&V> {
        match self.data.get(k) {
            Some(v) => Some(&v.value),
            None => None,
        }
    }
}
