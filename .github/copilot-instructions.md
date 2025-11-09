# GitHub Copilot Instructions

## Project Context

This is a Rust learning project (`rust-xp_rs`) that implements classic algorithms:
- Factorial (iterative and recursive)
- Fibonacci (iterative and recursive)

## Commit Message Guidelines

Follow the Conventional Commits specification as detailed in `COMMIT_GUIDELINES.md`:

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
- `tests`: Test code
- `main`: Main entry point
- `ci`: CI configuration

### Examples
```
feat(fibonacci): add iterative implementation
fix(factorial): handle overflow for large inputs
test(fibonacci): add edge case tests
docs(readme): update usage examples
ci(gitlab): add test coverage stage
```

## Code Style

- Follow Rust standard formatting (`rustfmt`)
- Use meaningful variable and function names
- Add documentation comments (`///`) for public functions
- Include examples in doc comments
- Write tests for new functionality

## Testing

- Run `cargo test` before committing
- Include unit tests for new functions
- Add doc tests in function documentation
- Test edge cases (0, 1, negative numbers, large values)

## When Suggesting Commits

1. Keep commits atomic (one logical change per commit)
2. Use present tense, imperative mood ("add" not "added")
3. Keep subject line under 50 characters
4. Reference relevant files in commit body if helpful
5. Suggest appropriate type and scope based on changes
