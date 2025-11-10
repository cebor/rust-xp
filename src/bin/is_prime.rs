use rust_xp::{is_prime, parse_single_arg};

fn main() {
    let n = parse_single_arg("17");
    let result = is_prime(n);
    println!("{}", result);
}
