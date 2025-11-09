use rust_xp_rs::{factorial_iterative, factorial_recursive, fibonacci_iterative, fibonacci_recursive};
use std::env;
use std::process;

fn print_usage(program: &str) {
    eprintln!("Usage:");
    eprintln!("  {} fac [--rec] <n>    Calculate factorial", program);
    eprintln!("  {} fib [--rec] <n>    Calculate fibonacci", program);
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --rec                 Use recursive implementation (default: iterative)");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {} fac 5              Calculate 5! iteratively", program);
    eprintln!("  {} fac --rec 5        Calculate 5! recursively", program);
    eprintln!("  {} fib 10             Calculate fib(10) iteratively", program);
    eprintln!("  {} fib --rec 10       Calculate fib(10) recursively", program);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    if args.len() < 3 {
        print_usage(program);
        process::exit(1);
    }

    let command = &args[1];
    let mut use_recursive = false;
    let mut number_arg_index = 2;

    // Check if --rec flag is present
    if args.len() >= 3 && args[2] == "--rec" {
        use_recursive = true;
        number_arg_index = 3;
    }

    // Validate we have the number argument
    if args.len() <= number_arg_index {
        eprintln!("Error: Missing number argument");
        eprintln!();
        print_usage(program);
        process::exit(1);
    }

    // Parse the number
    let n: u64 = match args[number_arg_index].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: '{}' is not a valid number", args[number_arg_index]);
            process::exit(1);
        }
    };

    // Execute the appropriate function
    let result = match command.as_str() {
        "fac" => {
            if use_recursive {
                factorial_recursive(n)
            } else {
                factorial_iterative(n)
            }
        }
        "fib" => {
            if use_recursive {
                fibonacci_recursive(n)
            } else {
                fibonacci_iterative(n)
            }
        }
        _ => {
            eprintln!("Error: Unknown command '{}'", command);
            eprintln!();
            print_usage(program);
            process::exit(1);
        }
    };

    println!("{}", result);
}
