use rust_xp_rs::{factorial_iterative, parse_single_arg};

fn main() {
    let n = parse_single_arg("5");
    let result = factorial_iterative(n);
    println!("{}", result);
}
