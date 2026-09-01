use rust_ci_cd_example::{add, multiply, subtract};

fn main() {
    let a = 10;
    let b = 5;

    println!("Rust CI/CD Example");
    println!("{} + {} = {}", a, b, add(a, b));
    println!("{} - {} = {}", a, b, subtract(a, b));
    println!("{} * {} = {}", a, b, multiply(a, b));
}
