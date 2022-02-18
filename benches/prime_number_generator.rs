extern crate criterion;
extern crate primal;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use primal::*;

fn is_prime(n: &u128) -> bool {
    if *n == 0 || *n == 1 {
        return false;
    }
    if *n == 2 {
        return true;
    }
    if *n % 2 == 0 {
        return false;
    }
    !(3..=(n/2)).filter(|x| x % 2 != 0).any(|x| n % x == 0)
}
fn is_prime_d(n: &u128) -> bool {
    // if *n == 0 || *n == 1 {
    //     return false;
    // }
    if *n == 2 {
        return true;
    }
    if *n % 2 == 0 || *n == 0 || *n == 1 {
        return false;
    }
    !(3..=(n/2)).filter(|x| x % 2 != 0).any(|x| n % x == 0)
}
// pub fn nth(n: u128) -> u128 {
//     (0..).filter(|x| is_prime(x))
//         .take(n as usize + 1)
//         .last().unwrap()
// }

fn prime_number_generator_a(max: u128) -> Vec<u128> {
    let mut primes = vec![2];
    for n in 3..max {
        if primes.iter().all(|p| n % p != 0) {
            primes.push(n);
        }
    }
    primes
}

fn prime_number_generator_b(n: u128) -> Vec<u128> {
    let mut primes = vec![true; (n + 1) as usize];
    let mut p = 2;
    while p * p <= n {
        if primes[p as usize] {
            let mut index = p * p;
            while index <= n {
                primes[index as usize] = false;
                index += p;
            }
        }

        p += 1;
    }
    let mut result = vec![];
    for (i, p) in primes.iter().enumerate() {
        if *p {
            result.push(i as u128);
        }
    }
    result.remove(0);
    result.remove(0);
    result
}

fn prime_number_generator_d(max: u128) -> Vec<u128> {
    (2..max).filter(|x| is_prime(x))
        .collect::<Vec<_>>()
}
fn prime_number_generator_d2(max: u128) -> Vec<u128> {
    (2..max).filter(|x| is_prime_d(x))
        .collect::<Vec<_>>()
}

fn prime_number_generator_c(max: u64) -> Vec<u64> {
    let mut p = 0;
    let mut v: Vec<u64> = vec![];

    for k in 1..max {
        //p = k * (1 << (27 + 1)) + 1;
        p = k * (1 << 28) + 1;
        if primal::is_prime(p) {
            v.push(p)
        }
    }
    v
}

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group(
        "prime_number_generator-vars:",
    );
    for i in [10000u128].iter() {
        group.bench_with_input(
            BenchmarkId::new("A prime_number_generator_a", i),
            i,
            |b, i| b.iter(|| prime_number_generator_a(*i)),
        );
        group.bench_with_input(
            BenchmarkId::new("B prime_number_generator_b", i),
            i,
            |b, i| b.iter(|| prime_number_generator_b(*i)),
        );
        group.bench_with_input(
            BenchmarkId::new("C prime_number_generator_c", i),
            i,
            |b, i| b.iter(|| prime_number_generator_c(*i as u64)),
        );
        group.bench_with_input(
            BenchmarkId::new("D prime_number_generator_d", i),
            i,
            |b, i| b.iter(|| prime_number_generator_d(*i)),
        );
        group.bench_with_input(
            BenchmarkId::new("D2 prime_number_generator_d2", i),
            i,
            |b, i| b.iter(|| prime_number_generator_d2(*i)),
        );
    }
    group.finish();
}
// https://users.rust-lang.org/t/help-fastest-prime-sieve-in-rust/31049/11
// https://docs.rs/primal/0.3.0/primal/
// https://ru.wikipedia.org/wiki/%D0%A1%D0%B5%D0%BA%D1%83%D0%BD%D0%B4%D0%B0
criterion_group!(benches, bench_fibs);
criterion_main!(benches);