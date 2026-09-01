use rust_ci_cd_example::{add, multiply, subtract};

fn main() {
    let a = 10;
    let b = 5;

    println!("Rust CI/CD Example");
    println!("{} + {} = {}", a, b, add(a, b));
    println!("{} - {} = {}", a, b, subtract(a, b));
    println!("{} * {} = {}", a, b, multiply(a, b));
}



//cargo fmt --check
//cargo fmt 
//cargo clippy --all-targets --all-features -- -D warnings

//git init 
//git status
//git add .
// git commit -m "Initial Rust project"  
//git remote add origin rust-ci-cd-example
//git branch -M main
//git remote set-url origin https://github.com/prashant-g/rust-ci-cd-example.git
//git remote -v
//git push -u origin main