# GitHub Copilot Instructions

## Project Context

This is a Rust learning project (`rust-xp_rs`) that implements classic algorithms:
- Factorial (iterative and recursive)
- Fibonacci (iterative and recursive)

### Project Structure

- `src/lib.rs`: Core library with all math functions and helper utilities
  - `factorial_iterative()`: Iterative factorial implementation
  - `factorial_recursive()`: Recursive factorial implementation
  - `fibonacci_iterative()`: Iterative Fibonacci implementation
  - `fibonacci_recursive()`: Recursive Fibonacci implementation
  - `parse_single_arg()`: CLI argument parser helper
- `src/main.rs`: Main CLI interface (default binary: `rust-xp_rs`)
  - Unified command-line interface for all algorithms
  - Supports: `fac [--rec] <n>` and `fib [--rec] <n>`
- `src/bin/`: Individual binary executables (legacy, kept for compatibility)
  - `factorial_itr.rs`, `factorial_rec.rs`
  - `fibonacci_itr.rs`, `fibonacci_rec.rs`

### Usage

```bash
# Main CLI (recommended)
./rust-xp fac 5              # Factorial iterative (default)
./rust-xp fac --rec 5        # Factorial recursive
./rust-xp fib 7              # Fibonacci iterative (default)
./rust-xp fib --rec 10       # Fibonacci recursive

# Individual binaries
cargo run --bin factorial_itr 5
cargo run --bin fibonacci_rec 10
```

## Commit Message Guidelines

Follow the Conventional Commits specification:

### Format
```
<type>(<scope>): <subject>
```

### Common Types
- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation changes
- `test`: Test additions/modifications
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `ci`: CI/CD changes
- `build`: Build system changes

### Common Scopes
- `factorial`: Factorial implementations
- `fibonacci`: Fibonacci implementations
- `lib`: Library functions in src/lib.rs
- `cli`: Main CLI interface in src/main.rs
- `bin`: Individual binary files
- `tests`: Test code
- `ci`: CI configuration

### Examples
```
feat(fibonacci): add iterative implementation
fix(factorial): handle overflow for large inputs
refactor(lib): move math functions to library
feat(cli): add unified command-line interface
test(fibonacci): add edge case tests
docs(readme): update usage examples
ci(gitlab): add test coverage stage
```

## Code Style

- Follow Rust standard formatting (`rustfmt`)
- Use meaningful variable and function names
- Add documentation comments (`///`) for public functions
- Include examples in doc comments (they run as doc tests)
- Write tests for new functionality
- Keep binary files minimal - put logic in the library
- Use the library functions from `rust_xp_rs` crate in binaries

## Testing

- Run `cargo test` before committing (includes unit tests and doc tests)
- All tests are in `src/lib.rs` for centralized maintenance
- Include unit tests for new functions
- Add doc tests in function documentation
- Test edge cases (0, 1, large values)
- Use `cargo llvm-cov` for code coverage analysis

## When Suggesting Commits

1. Keep commits atomic (one logical change per commit)
2. Use present tense, imperative mood ("add" not "added")
3. Keep subject line under 50 characters
4. Reference relevant files in commit body if helpful
5. Suggest appropriate type and scope based on changes
