use std::env;
use std::process;

/// Parses a single u64 argument from command line arguments.
///
/// This helper function handles argument validation and parsing,
/// providing consistent error messages across binaries.
///
/// # Arguments
///
/// * `example_value` - An example value to show in the usage message
///
/// # Returns
///
/// The parsed u64 value
///
/// # Panics
///
/// Exits the process with exit code 1 if:
/// - Wrong number of arguments provided
/// - Argument cannot be parsed as u64
///
/// # Examples
///
/// ```no_run
/// use rust_xp_rs::parse_single_arg;
/// let n = parse_single_arg("5");
/// ```
pub fn parse_single_arg(example_value: &str) -> u64 {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <n>", args[0]);
        eprintln!("Example: {} {}", args[0], example_value);
        process::exit(1);
    }

    match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: '{}' is not a valid number", args[1]);
            process::exit(1);
        }
    }
}

/// Calculates the factorial of n using iteration.
///
/// This implementation uses iteration for better performance,
/// with O(n) time complexity and O(1) space complexity.
///
/// # Arguments
///
/// * `n` - The number to calculate the factorial of
///
/// # Returns
///
/// The factorial of n (n!)
///
/// # Examples
///
/// ```
/// use rust_xp_rs::factorial_iterative;
/// assert_eq!(factorial_iterative(0), 1);
/// assert_eq!(factorial_iterative(1), 1);
/// assert_eq!(factorial_iterative(5), 120);
/// assert_eq!(factorial_iterative(10), 3628800);
/// ```
pub fn factorial_iterative(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 1;
    }

    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }

    result
}

/// Calculates the factorial of n using recursion.
///
/// This implementation uses recursion for educational purposes.
/// Note: The iterative version is more efficient for larger values
/// as it avoids the overhead of recursive function calls.
///
/// # Arguments
///
/// * `n` - The number to calculate the factorial of
///
/// # Returns
///
/// The factorial of n (n!)
///
/// # Examples
///
/// ```
/// use rust_xp_rs::factorial_recursive;
/// assert_eq!(factorial_recursive(0), 1);
/// assert_eq!(factorial_recursive(1), 1);
/// assert_eq!(factorial_recursive(5), 120);
/// assert_eq!(factorial_recursive(10), 3628800);
/// ```
pub fn factorial_recursive(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial_recursive(n - 1)
    }
}

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
/// use rust_xp_rs::fibonacci_iterative;
/// assert_eq!(fibonacci_iterative(0), 0);
/// assert_eq!(fibonacci_iterative(1), 1);
/// assert_eq!(fibonacci_iterative(6), 8);
/// assert_eq!(fibonacci_iterative(10), 55);
/// ```
pub fn fibonacci_iterative(n: u64) -> u64 {
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
/// use rust_xp_rs::fibonacci_recursive;
/// assert_eq!(fibonacci_recursive(0), 0);
/// assert_eq!(fibonacci_recursive(1), 1);
/// assert_eq!(fibonacci_recursive(6), 8);
/// assert_eq!(fibonacci_recursive(10), 55);
/// ```
pub fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_iterative_base_cases() {
        assert_eq!(factorial_iterative(0), 1);
        assert_eq!(factorial_iterative(1), 1);
    }

    #[test]
    fn test_factorial_iterative_small_values() {
        assert_eq!(factorial_iterative(2), 2);
        assert_eq!(factorial_iterative(3), 6);
        assert_eq!(factorial_iterative(4), 24);
        assert_eq!(factorial_iterative(5), 120);
        assert_eq!(factorial_iterative(6), 720);
    }

    #[test]
    fn test_factorial_iterative_larger_values() {
        assert_eq!(factorial_iterative(10), 3628800);
        assert_eq!(factorial_iterative(12), 479001600);
        assert_eq!(factorial_iterative(15), 1307674368000);
    }

    #[test]
    fn test_factorial_iterative_edge_values() {
        assert_eq!(factorial_iterative(20), 2432902008176640000);
    }

    #[test]
    fn test_factorial_recursive_base_cases() {
        assert_eq!(factorial_recursive(0), 1);
        assert_eq!(factorial_recursive(1), 1);
    }

    #[test]
    fn test_factorial_recursive_small_values() {
        assert_eq!(factorial_recursive(2), 2);
        assert_eq!(factorial_recursive(3), 6);
        assert_eq!(factorial_recursive(4), 24);
        assert_eq!(factorial_recursive(5), 120);
        assert_eq!(factorial_recursive(6), 720);
    }

    #[test]
    fn test_factorial_recursive_larger_values() {
        assert_eq!(factorial_recursive(10), 3628800);
        assert_eq!(factorial_recursive(12), 479001600);
        assert_eq!(factorial_recursive(15), 1307674368000);
    }

    #[test]
    fn test_factorial_recursive_edge_values() {
        assert_eq!(factorial_recursive(20), 2432902008176640000);
    }

    #[test]
    fn test_fibonacci_iterative_base_cases() {
        assert_eq!(fibonacci_iterative(0), 0);
        assert_eq!(fibonacci_iterative(1), 1);
    }

    #[test]
    fn test_fibonacci_iterative_small_values() {
        assert_eq!(fibonacci_iterative(2), 1);
        assert_eq!(fibonacci_iterative(3), 2);
        assert_eq!(fibonacci_iterative(4), 3);
        assert_eq!(fibonacci_iterative(5), 5);
        assert_eq!(fibonacci_iterative(6), 8);
    }

    #[test]
    fn test_fibonacci_iterative_larger_values() {
        assert_eq!(fibonacci_iterative(10), 55);
        assert_eq!(fibonacci_iterative(15), 610);
        assert_eq!(fibonacci_iterative(20), 6765);
        assert_eq!(fibonacci_iterative(30), 832040);
    }

    #[test]
    fn test_fibonacci_iterative_very_large_values() {
        assert_eq!(fibonacci_iterative(50), 12586269025);
    }

    #[test]
    fn test_fibonacci_recursive_base_cases() {
        assert_eq!(fibonacci_recursive(0), 0);
        assert_eq!(fibonacci_recursive(1), 1);
    }

    #[test]
    fn test_fibonacci_recursive_small_values() {
        assert_eq!(fibonacci_recursive(2), 1);
        assert_eq!(fibonacci_recursive(3), 2);
        assert_eq!(fibonacci_recursive(4), 3);
        assert_eq!(fibonacci_recursive(5), 5);
        assert_eq!(fibonacci_recursive(6), 8);
    }

    #[test]
    fn test_fibonacci_recursive_larger_values() {
        assert_eq!(fibonacci_recursive(10), 55);
        assert_eq!(fibonacci_recursive(15), 610);
        assert_eq!(fibonacci_recursive(20), 6765);
    }
}
