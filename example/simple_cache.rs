extern crate rache;
use rache::Rache;
use std::time::Duration;
use std::thread;

fn main() {
    println!("==================> Rache");

    let mut r: Rache<String, i32> = Rache::new();
    let first = String::from("first");
    r.set(first, 10);

    let first_key = String::from("first");
    match r.get(&first_key) {
        Some(v) => println!("value: {}", v),
        None => println!("not found"),
    }

    let second = "second".to_string();
    r.set(second, 20);

    let second_key = "second".to_string();
    let value = r.get(&second_key).unwrap();
    println!("second value : {}", value);


    println!("==================> Rache with expiration");

    let mut r: Rache<String, i32> = Rache::new()
        .start_expiration_checker(Duration::from_secs(1));
    r.set("first_with_expiration".to_string(), 10);
    let first_key = String::from("first_with_expiration");
    match r.get(&first_key) {
        Some(v) => println!("value: {}", v),
        None => println!("not found"),
    }

    thread::sleep(Duration::from_secs(10));
}
