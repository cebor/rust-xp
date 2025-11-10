# Benchmarking Guide

This project uses [Criterion.rs](https://github.com/bheisler/criterion.rs) for benchmarking, which provides statistical analysis and detailed HTML reports.

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific algorithm benchmarks
cargo bench factorial
cargo bench fibonacci
cargo bench is_prime

# Save baseline for comparison
cargo bench -- --save-baseline my-baseline

# Compare against saved baseline
cargo bench -- --baseline my-baseline
```

## Understanding Results

Benchmark results show:
- **time**: Average execution time with confidence intervals
- **outliers**: Measurements that deviate significantly from the norm
- **R² goodness of fit**: How well the measurements fit the model (closer to 1.0 is better)

### Reading Output

```
factorial_iterative/10  time:   [4.1558 ns 4.1732 ns 4.1923 ns]
                              └─ min ─┘ └─ mean ┘ └─ max ──┘
```

### HTML Reports

Detailed HTML reports are generated in `target/criterion/`:
- `target/criterion/report/index.html` - Main overview
- Individual benchmark directories contain detailed plots and analysis

View the HTML report:
```bash
open target/criterion/report/index.html  # macOS
xdg-open target/criterion/report/index.html  # Linux
start target/criterion/report/index.html  # Windows
```

## Benchmark Coverage

### Factorial Benchmarks

1. **factorial_iterative**: Tests iterative implementation with inputs 5, 10, 15, 20
2. **factorial_recursive**: Tests recursive implementation with inputs 5, 10, 15, 20
3. **factorial_comparison**: Compares both approaches with input 15

**Expected Results**: Iterative should be slightly faster for larger inputs due to no function call overhead.

### Fibonacci Benchmarks

1. **fibonacci_iterative**: Tests iterative implementation with inputs 10, 20, 30, 40
2. **fibonacci_recursive**: Tests recursive implementation with inputs 10, 15, 20, 25
   - Note: Uses smaller values due to O(2^n) complexity
3. **fibonacci_comparison**: Compares both approaches with input 20

**Expected Results**: Iterative should be dramatically faster, especially for larger inputs. The recursive version has exponential time complexity.

### Prime Number Benchmarks

1. **is_prime**: Tests with various input types:
   - Small prime (17)
   - Medium prime (997)
   - Large prime (104,729)
   - Small composite (100)
   - Medium composite (10,000)

**Expected Results**: Performance scales with the square root of the input due to the trial division algorithm.

## Adding New Benchmarks

When adding new algorithms, create benchmarks in `benches/algorithms.rs`:

```rust
fn bench_my_algorithm(c: &mut Criterion) {
    let mut group = c.benchmark_group("my_algorithm");
    
    for n in [10, 20, 30].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            b.iter(|| my_algorithm(black_box(n)));
        });
    }
    
    group.finish();
}

// Add to criterion_group! macro
criterion_group!(
    benches,
    // ... existing benchmarks ...
    bench_my_algorithm
);
```

## Performance Tips

1. **Use `black_box()`**: Prevents compiler optimizations that might skew results
2. **Warm-up iterations**: Criterion automatically does warm-up runs
3. **Sample size**: Default is 100 samples, adjustable if needed
4. **Measurement time**: Default is 5 seconds per benchmark
5. **Outliers**: Normal to have some outliers; Criterion handles them statistically

## Baseline Comparisons

Save baselines before making optimizations:

```bash
# Before optimization
cargo bench -- --save-baseline before

# Make your changes
# ...

# After optimization
cargo bench -- --baseline before
```

Criterion will show percentage improvements/regressions.

## CI Integration

To run benchmarks in CI without generating full reports:

```bash
cargo bench --no-fail-fast -- --quick
```

For smoke testing (faster):
```bash
cargo bench --no-fail-fast -- --test
```

## Troubleshooting

### "Gnuplot not found"
This is normal. Criterion falls back to plotters backend for generating graphs. To use gnuplot:
```bash
brew install gnuplot  # macOS
apt-get install gnuplot  # Ubuntu/Debian
```

### Inconsistent Results
- Close other applications to reduce system noise
- Run multiple times and compare results
- Check CPU governor settings (Linux)
- Disable CPU frequency scaling if possible

### Very Long Benchmark Times
- Reduce sample size: `cargo bench -- --sample-size 10`
- Use quick mode: `cargo bench -- --quick`
- For recursive Fibonacci, use smaller input values

## Resources

- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [Criterion.rs User Guide](https://bheisler.github.io/criterion.rs/book/user_guide/user_guide.html)
- [Statistical Analysis](https://bheisler.github.io/criterion.rs/book/analysis.html)
