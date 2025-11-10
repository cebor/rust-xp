# Contributing to rust-xp

Thank you for your interest in contributing to rust-xp! This document provides guidelines and instructions for contributing to this project.

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone <your-fork-url>`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Test your changes: `cargo test`
6. Commit your changes (see commit guidelines below)
7. Push to your fork: `git push origin feature/your-feature-name`
8. Open a Pull Request

## Development Setup

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Building the Project

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_factorial_iterative
```

### Code Coverage

```bash
# Install llvm-cov if not already installed
cargo install cargo-llvm-cov

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

# View HTML reports
open target/criterion/report/index.html
```

When adding new features, consider adding benchmarks to measure performance, especially when:
- Implementing multiple approaches to the same problem
- Optimizing existing code
- Comparing iterative vs recursive solutions

## Commit Message Guidelines

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification.

### Format

```
<type>(<scope>): <subject>

[optional body]

[optional footer]
```

### Types

- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation changes
- `test`: Test additions/modifications
- `refactor`: Code refactoring (no functional changes)
- `perf`: Performance improvements
- `ci`: CI/CD changes
- `build`: Build system changes
- `style`: Code style changes (formatting, etc.)

### Common Scopes

- `factorial`: Factorial implementations
- `fibonacci`: Fibonacci implementations
- `prime`: Prime number functionality
- `lib`: Library functions in `src/lib.rs`
- `cli`: Main CLI interface in `src/main.rs`
- `bin`: Individual binary files
- `tests`: Test code
- `bench`: Benchmarking code
- `ci`: CI configuration
- `docs`: Documentation

### Examples

```
feat(fibonacci): add iterative implementation
fix(factorial): handle overflow for large inputs
refactor(lib): move math functions to library
feat(cli): add unified command-line interface
test(fibonacci): add edge case tests
docs(readme): update usage examples
perf(fibonacci): optimize iterative calculation
feat(bench): add criterion benchmarks for all algorithms
```

### Best Practices

1. **Keep commits atomic**: One logical change per commit
2. **Use present tense, imperative mood**: "add" not "added" or "adds"
3. **Keep subject line under 50 characters**
4. **Capitalize the subject line**
5. **Don't end subject line with a period**
6. **Wrap body at 72 characters** (if adding a body)
7. **Use the body to explain what and why, not how**

## Code Style

### Rust Style Guidelines

- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- Use `rustfmt` for formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`

### Specific Conventions

1. **Function naming**: Use descriptive names with underscores (`factorial_iterative`)
2. **Documentation**: Add doc comments (`///`) for all public functions
3. **Examples**: Include examples in doc comments (they run as doc tests!)
4. **Error handling**: Use `Result` and `?` operator where appropriate
5. **Keep binaries minimal**: Put logic in the library (`src/lib.rs`)
6. **Import organization**: Group imports (std, external crates, local)

### Documentation Comments

```rust
/// Calculates the factorial of a number iteratively.
///
/// # Arguments
///
/// * `n` - The number to calculate the factorial of
///
/// # Returns
///
/// The factorial of `n` as a `u128`
///
/// # Examples
///
/// ```
/// use rust_xp::factorial_iterative;
/// assert_eq!(factorial_iterative(5), 120);
/// ```
pub fn factorial_iterative(n: u32) -> u128 {
    // implementation
}
```

## Project Structure

### File Organization

- `src/lib.rs`: Core library with all math functions and utilities
- `src/main.rs`: Main CLI interface (default binary)
- `src/bin/`: Individual binary executables (legacy, kept for compatibility)

### Adding New Features

1. **Add the implementation to `src/lib.rs`**
   - Write the function with documentation
   - Include doc test examples
   - Add unit tests in the `#[cfg(test)]` module

2. **Add CLI support to `src/main.rs`** (if applicable)
   - Add command handling
   - Update help text

3. **Optional: Create a standalone binary in `src/bin/`**
   - Keep it minimal
   - Import and use library functions

4. **Update documentation**
   - Update `README.md` with usage examples
   - Update this file if contributing guidelines change

## Testing Guidelines

### Test Organization

All tests are centralized in `src/lib.rs` for easier maintenance.

### Test Categories

1. **Unit tests**: Test individual functions
2. **Doc tests**: Test code examples in documentation
3. **Integration tests**: Test CLI behavior (if needed)

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_zero() {
        assert_eq!(factorial_iterative(0), 1);
    }

    #[test]
    fn test_factorial_positive() {
        assert_eq!(factorial_iterative(5), 120);
    }
}
```

### Test Coverage

- Test edge cases (0, 1, maximum values)
- Test typical cases
- Test error conditions
- Aim for high code coverage (use `cargo llvm-cov`)

## Pull Request Process

1. **Ensure all tests pass**: `cargo test`
2. **Format your code**: `cargo fmt`
3. **Run clippy**: `cargo clippy`
4. **Update documentation** if needed
5. **Write a clear PR description**:
   - What changes were made
   - Why the changes were necessary
   - Any breaking changes
   - Related issues (if any)
6. **Wait for review**
7. **Address feedback** if requested

## Questions or Issues?

- Open an issue for bugs or feature requests
- Use discussions for questions
- Check existing issues before creating new ones

## Learning Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/)

Thank you for contributing! 🦀
