mod generate_fn;
use generate_fn::generate_fn;

fn main() {
    let hello: u64 = 42;
    println!("{hello}");
    
    generate_fn();
    println!("Hello, world!");
}