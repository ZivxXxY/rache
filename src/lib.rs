use std::collections::HashMap;
use std::hash::Hash;

trait Cache<K, V> {
    fn set(&mut self, k: K, v: V);
    fn get(&mut self, k: &K) -> Option<&V>;
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_variables() {
        let mut int_rache: Rache<u32, u32> = Rache::new();
        let zero = 0;
        let ten = 1;
        int_rache.set(zero, 0);
        int_rache.set(ten, 10);
        match int_rache.get(&zero) {
            Some(v) => assert_eq!(0, *v, "v must be 0 but v is {}", v),
            None => panic!("it must get variables"),
        }
        match int_rache.get(&ten) {
            Some(v) => assert_eq!(10, *v, "v must be 10 but v is {}", v),
            None => panic!("it must get variables"),
        }

        let mut mix_rache: Rache<String, u32> = Rache::new();
        let apple = String::from("apple");
        let lemon = String::from("lemon");
        mix_rache.set(apple, 10);
        mix_rache.set(lemon, 200);
        match mix_rache.get(&"apple".to_string()) {
            Some(v) => assert_eq!(10, *v, "v must be 10 but v is {}", v),
            None => panic!("it must get variables"),
        }
        match mix_rache.get(&"lemon".to_string()) {
            Some(v) => assert_eq!(200, *v, "v must be 200 but v is {}", v),
            None => panic!("it must get variables"),
        }
    }
}
