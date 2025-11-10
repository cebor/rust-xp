use rust_xp::{fibonacci_recursive, parse_single_arg};

fn main() {
    let n = parse_single_arg("6");
    let result = fibonacci_recursive(n);
    println!("{}", result);
}
