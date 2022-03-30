extern crate criterion;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode};
use nanoid::nanoid;

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_id_generator_3");
    group.sample_size(10000);
    //group.sampling_mode(SamplingMode::Auto);
    group.bench_function(
        "nanoid()",
        |b| b.iter(|| {
            nanoid!();
        }),
    );
    group.bench_function(
        "nanoid(10)",
        |b| b.iter(|| {
            nanoid!(10);
        }),
    );
    group.bench_function(
        "nanoid(25)",
        |b| b.iter(|| {
            nanoid!(25);
        }),
    );
    group.bench_function(
        "nanoid(100)",
        |b| b.iter(|| {
            nanoid!(100);
        }),
    );
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);