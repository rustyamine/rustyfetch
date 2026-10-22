use std::env;

fn main() {
    let user = env::var("USER").unwrap();
    println!("user: {}", user);
}
