use rust_xp::{fibonacci_iterative, parse_single_arg};

fn main() {
    let n = parse_single_arg("6");
    let result = fibonacci_iterative(n);
    println!("{}", result);
}
