use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rust_xp::{factorial_iterative, factorial_recursive, fibonacci_iterative, fibonacci_recursive, is_prime};

fn bench_factorial_iterative(c: &mut Criterion) {
    let mut group = c.benchmark_group("factorial_iterative");
    
    for n in [5, 10, 15, 20].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            b.iter(|| factorial_iterative(black_box(n)));
        });
    }
    
    group.finish();
}

fn bench_factorial_recursive(c: &mut Criterion) {
    let mut group = c.benchmark_group("factorial_recursive");
    
    for n in [5, 10, 15, 20].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            b.iter(|| factorial_recursive(black_box(n)));
        });
    }
    
    group.finish();
}

fn bench_factorial_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("factorial_comparison");
    let n = 15;
    
    group.bench_function("iterative", |b| {
        b.iter(|| factorial_iterative(black_box(n)));
    });
    
    group.bench_function("recursive", |b| {
        b.iter(|| factorial_recursive(black_box(n)));
    });
    
    group.finish();
}

fn bench_fibonacci_iterative(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci_iterative");
    
    for n in [10, 20, 30, 40].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            b.iter(|| fibonacci_iterative(black_box(n)));
        });
    }
    
    group.finish();
}

fn bench_fibonacci_recursive(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci_recursive");
    
    // Using smaller values for recursive due to exponential complexity
    for n in [10, 15, 20, 25].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(n), n, |b, &n| {
            b.iter(|| fibonacci_recursive(black_box(n)));
        });
    }
    
    group.finish();
}

fn bench_fibonacci_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci_comparison");
    let n = 20;
    
    group.bench_function("iterative", |b| {
        b.iter(|| fibonacci_iterative(black_box(n)));
    });
    
    group.bench_function("recursive", |b| {
        b.iter(|| fibonacci_recursive(black_box(n)));
    });
    
    group.finish();
}

fn bench_is_prime(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_prime");
    
    // Test with various types of numbers
    let test_cases = vec![
        ("small_prime", 17),
        ("medium_prime", 997),
        ("large_prime", 104729),
        ("small_composite", 100),
        ("medium_composite", 10000),
    ];
    
    for (name, n) in test_cases {
        group.bench_with_input(BenchmarkId::new("check", name), &n, |b, &n| {
            b.iter(|| is_prime(black_box(n)));
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_factorial_iterative,
    bench_factorial_recursive,
    bench_factorial_comparison,
    bench_fibonacci_iterative,
    bench_fibonacci_recursive,
    bench_fibonacci_comparison,
    bench_is_prime
);

criterion_main!(benches);
