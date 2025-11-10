use rust_xp::{factorial_recursive, parse_single_arg};

fn main() {
    let n = parse_single_arg("5");
    let result = factorial_recursive(n);
    println!("{}", result);
}
