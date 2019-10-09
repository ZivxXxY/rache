use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};

const DEFAULT_DURATION_NANOSECONDS: i32 = 5;
// const DEFAULT_SYNC_TIME_DURATION: i32 = 100;

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

    update_dur: i32,
    sender: Option<SyncSender<HashMap<K, V>>>,
    receiver: Option<Receiver<HashMap<K, V>>>
}

impl<K: Hash + Eq, V> Rache<K, V> {
    pub fn new() -> Rache<K, V> {
        Rache {
            data: HashMap::new(),
            update_dur: DEFAULT_DURATION_NANOSECONDS,
            sender: None,
            receiver: None
        }
    }

    pub fn set_duration_time<'a>(&'a mut self, dur: i32) -> &'a mut Rache<K, V> {
        self.update_dur = dur;
        start_tick(dur);

        let (s, r) = sync_channel::<HashMap<K, V>>(0);
        self.sender = Some(s);
        self.receiver = Some(r);
        self
    }

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

fn start_tick(update_dur: i32) {
    thread::spawn(move || {
        loop {
            println!("loop!!!!");
            println!("{}  ", update_dur);
        }
    });
}
