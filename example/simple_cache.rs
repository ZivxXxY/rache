extern crate rache;
use rache::Rache;

fn main() {
    let mut r: Rache<String, i32> = Rache::new().set_duration_time(2);
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
}
