use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Instant, Duration};
use std::thread;
use std::sync::{Arc, RwLock};


#[derive(Debug)]
struct Entry<V> {
    value: V,
    timestamp: Instant,
    expiration: Duration,
    has_expiration: bool
}

impl<V> Entry<V> {
    fn new(v: V, timestamp: Instant, expiration: Duration, has_expiration: bool) -> Self {
        Entry {
            value: v,
            timestamp,
            expiration,
            has_expiration
        }
    }
}

pub struct Rache<K, V> {
    data: Arc<RwLock<HashMap<K, Entry<V>>>>
}

impl<K: Hash + Eq + Send + Sync, V: Send + Sync> Rache<K, V> {
    pub fn new() -> Rache<K, V> {
        Rache {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn start_expiration_checker(self, dur: Duration) -> Rache<K, V> {
        thread::spawn(move || {
            loop {
                thread::sleep(dur);
            }
        });
        self
    }

    pub fn set(&mut self, k: K, v: V) {
        let entry = Entry::new(v, Instant::now(), Duration::new(0, 0), false);

        if let Ok(mut d) = self.data.write() {
            d.insert(k, entry);
        }
    }

    pub fn get(&mut self, k: &K) -> Option<&V> {
        // TODO: check whether the has_expiration is false
        if let Ok(d) = self.data.read() {
            match d.get(k) {
                Some(v) => return Some(&v.value),
                None => return None
            }
        }

        None
    }

    pub fn set_with_expiration(&mut self, k: K, v: V, dur: Duration) {
        let entry = Entry::new(v, Instant::now(), dur, true);

        if let Ok(mut d) = self.data.write() {
            d.insert(k, entry);
        }
    }

    pub fn check_expiration(self) {
        let data = self.data.read().unwrap();

        for (_, v) in data.iter() {
            if v.has_expiration {
                println!("PL");
            }
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
