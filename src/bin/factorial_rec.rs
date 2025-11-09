use std::env;
use std::process;

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
/// assert_eq!(factorial_recursive(0), 1);
/// assert_eq!(factorial_recursive(1), 1);
/// assert_eq!(factorial_recursive(5), 120);
/// assert_eq!(factorial_recursive(10), 3628800);
/// ```
fn factorial_recursive(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial_recursive(n - 1)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <n>", args[0]);
        eprintln!("Example: {} 5", args[0]);
        process::exit(1);
    }

    let n: u64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: '{}' is not a valid number", args[1]);
            process::exit(1);
        }
    };

    let result = factorial_recursive(n);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_base_cases() {
        assert_eq!(factorial_recursive(0), 1);
        assert_eq!(factorial_recursive(1), 1);
    }

    #[test]
    fn test_factorial_small_values() {
        assert_eq!(factorial_recursive(2), 2);
        assert_eq!(factorial_recursive(3), 6);
        assert_eq!(factorial_recursive(4), 24);
        assert_eq!(factorial_recursive(5), 120);
        assert_eq!(factorial_recursive(6), 720);
    }

    #[test]
    fn test_factorial_larger_values() {
        assert_eq!(factorial_recursive(10), 3628800);
        assert_eq!(factorial_recursive(12), 479001600);
        assert_eq!(factorial_recursive(15), 1307674368000);
    }

    #[test]
    fn test_factorial_edge_values() {
        // u64::MAX is 18446744073709551615
        // 20! = 2432902008176640000, which fits in u64
        assert_eq!(factorial_recursive(20), 2432902008176640000);
    }
}
