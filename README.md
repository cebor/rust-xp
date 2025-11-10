# rust-xp

A Rust learning project implementing classic algorithms including factorial and Fibonacci calculations.

## Features

- **Factorial**: Both iterative and recursive implementations
- **Fibonacci**: Both iterative and recursive implementations
- **Prime Numbers**: Check if a number is prime

## Installation

```bash
git clone <repository-url>
cd rust-xp
cargo build --release
```

## Usage

### Main CLI (Recommended)

The unified command-line interface provides access to all algorithms:

```bash
# Factorial (iterative by default)
cargo run -- fac 5
./target/release/rust-xp fac 5

# Factorial (recursive)
cargo run -- fac --rec 5
./target/release/rust-xp fac --rec 5

# Fibonacci (iterative by default)
cargo run -- fib 7
./target/release/rust-xp fib 7

# Fibonacci (recursive)
cargo run -- fib --rec 10
./target/release/rust-xp fib --rec 10
```

### Individual Binaries

Individual binary executables are also available:

```bash
# Factorial
cargo run --bin factorial_itr 5
cargo run --bin factorial_rec 5

# Fibonacci
cargo run --bin fibonacci_itr 7
cargo run --bin fibonacci_rec 10

# Prime check
cargo run --bin is_prime 17
```

## Project Structure

```
rust-xp/
├── src/
│   ├── lib.rs          # Core library with all implementations
│   ├── main.rs         # Main CLI interface
│   └── bin/            # Individual binary executables
│       ├── factorial_itr.rs
│       ├── factorial_rec.rs
│       ├── fibonacci_itr.rs
│       ├── fibonacci_rec.rs
│       └── is_prime.rs
├── Cargo.toml
├── README.md
├── CONTRIBUTING.md
└── LICENSE
```

## Development

### Running Tests

```bash
# Run all tests (includes unit tests and doc tests)
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test --lib -- tests
```

### Code Coverage

```bash
# Generate coverage report
cargo llvm-cov

# Generate HTML coverage report
cargo llvm-cov --html
```

### Benchmarking

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench factorial

# Run benchmarks and save baseline
cargo bench -- --save-baseline my-baseline

# Compare against baseline
cargo bench -- --baseline my-baseline
```

The benchmarks compare:
- Iterative vs recursive implementations
- Performance across different input sizes
- Prime checking performance

Results are saved in `target/criterion/` with detailed HTML reports.

See [BENCHMARKING.md](BENCHMARKING.md) for detailed benchmarking guide.

### Documentation

```bash
# Build and open documentation
cargo doc --open
```

## Algorithms

### Factorial

Calculates n! = n × (n-1) × (n-2) × ... × 2 × 1

- **Iterative**: Uses a loop, O(n) time and O(1) space
- **Recursive**: Uses recursion, O(n) time and O(n) space

### Fibonacci

Calculates the nth Fibonacci number where F(n) = F(n-1) + F(n-2)

- **Iterative**: Uses a loop, O(n) time and O(1) space
- **Recursive**: Uses recursion, O(2^n) time and O(n) space

### Prime Check

Determines if a number is prime using trial division up to √n.

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to this project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Learning Goals

This project is designed to:
- Practice Rust syntax and idioms
- Understand iterative vs recursive approaches
- Learn Rust's module system and project structure
- Explore testing in Rust (unit tests and doc tests)
- Work with Rust's error handling
- Implement a CLI application

## Roadmap

- [ ] Add more algorithms (sorting, searching)
- [ ] Implement memoization for recursive Fibonacci
- [x] Add benchmarking
- [ ] Improve error handling and input validation
- [ ] Add more comprehensive tests
