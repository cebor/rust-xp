use std::env;
use std::process;

/// Calculates the nth Fibonacci number using recursion.
///
/// # Arguments
///
/// * `n` - The position in the Fibonacci sequence (0-indexed)
///
/// # Returns
///
/// The nth Fibonacci number
///
/// # Examples
///
/// ```
/// assert_eq!(fibonacci_recursive(0), 0);
/// assert_eq!(fibonacci_recursive(1), 1);
/// assert_eq!(fibonacci_recursive(6), 8);
/// assert_eq!(fibonacci_recursive(10), 55);
/// ```
fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <n>", args[0]);
        eprintln!("Example: {} 6", args[0]);
        process::exit(1);
    }

    let n: u64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: '{}' is not a valid number", args[1]);
            process::exit(1);
        }
    };

    let result = fibonacci_recursive(n);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_base_cases() {
        assert_eq!(fibonacci_recursive(0), 0);
        assert_eq!(fibonacci_recursive(1), 1);
    }

    #[test]
    fn test_fibonacci_small_values() {
        assert_eq!(fibonacci_recursive(2), 1);
        assert_eq!(fibonacci_recursive(3), 2);
        assert_eq!(fibonacci_recursive(4), 3);
        assert_eq!(fibonacci_recursive(5), 5);
        assert_eq!(fibonacci_recursive(6), 8);
    }

    #[test]
    fn test_fibonacci_larger_values() {
        assert_eq!(fibonacci_recursive(10), 55);
        assert_eq!(fibonacci_recursive(15), 610);
        assert_eq!(fibonacci_recursive(20), 6765);
    }
}
