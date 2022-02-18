extern crate criterion;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn fibonacci_slow(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci_slow(n - 1) + fibonacci_slow(n - 2),
    }
}

fn fibonacci_fast(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");
    for i in [40u64].iter() {
        group.bench_with_input(BenchmarkId::new("Recursive", i), i, |b, i| {
            b.iter(|| fibonacci_slow(*i))
        });
        group.bench_with_input(BenchmarkId::new("Iterative", i), i, |b, i| {
            b.iter(|| fibonacci_fast(*i))
        });
    }
    group.finish();
}
// https://ru.wikipedia.org/wiki/%D0%A1%D0%B5%D0%BA%D1%83%D0%BD%D0%B4%D0%B0
criterion_group!(benches, bench_fibs);
criterion_main!(benches);
