extern crate rache;

use rache::Rache;

fn main() {
    let r: Rache<String, i32> = Rache::new();
    r.set("hoge".to_string(), 10);
    println!("example");
}
