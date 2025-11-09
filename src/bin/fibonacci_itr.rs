use std::env;
use std::process;

/// Calculates the nth Fibonacci number using iteration.
///
/// This implementation is more efficient than the recursive version,
/// with O(n) time complexity and O(1) space complexity.
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
/// assert_eq!(fibonacci_iterative(0), 0);
/// assert_eq!(fibonacci_iterative(1), 1);
/// assert_eq!(fibonacci_iterative(6), 8);
/// assert_eq!(fibonacci_iterative(10), 55);
/// ```
fn fibonacci_iterative(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
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

    let result = fibonacci_iterative(n);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_base_cases() {
        assert_eq!(fibonacci_iterative(0), 0);
        assert_eq!(fibonacci_iterative(1), 1);
    }

    #[test]
    fn test_fibonacci_small_values() {
        assert_eq!(fibonacci_iterative(2), 1);
        assert_eq!(fibonacci_iterative(3), 2);
        assert_eq!(fibonacci_iterative(4), 3);
        assert_eq!(fibonacci_iterative(5), 5);
        assert_eq!(fibonacci_iterative(6), 8);
    }

    #[test]
    fn test_fibonacci_larger_values() {
        assert_eq!(fibonacci_iterative(10), 55);
        assert_eq!(fibonacci_iterative(15), 610);
        assert_eq!(fibonacci_iterative(20), 6765);
        assert_eq!(fibonacci_iterative(30), 832040);
    }

    #[test]
    fn test_fibonacci_very_large_values() {
        // The iterative version can handle much larger values efficiently
        assert_eq!(fibonacci_iterative(50), 12586269025);
    }
}
