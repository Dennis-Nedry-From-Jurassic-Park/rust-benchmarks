//#![cfg(feature = "alloc")]

extern crate criterion;
extern crate slab;
extern crate rand;

use criterion::{criterion_group, criterion_main, Criterion};

use smallvec::{SmallVec, smallvec};
use tinyvec::{array_vec, ArrayVec, TinyVec};
use tinyvec::tiny_vec;

use criterion::*;

fn smallvec() {
    let mut vec: Vec<SmallVec<[i32; 4]>> = Vec::new();
    //let mut sv: SmallVec<[i32; 4]> = smallvec![1, 2, 3, 4];

    for i in 1..=100_000_000_000u64 {
        vec.push(smallvec![1, 2, 3, 4]);
        //sv[0] += i;
    }
}
fn tinyvec() {
    let mut vec: Vec<TinyVec<[i32; 4]>> = Vec::new();

    for i in 1..=100_000_000_000u64 {
        vec.push(tiny_vec![1, 2, 3, 4]);
    }
}
fn arrayvec() {
    //let mut v = tiny_vec!([u64; 4] => 1, 2, 3, 4);
    let mut vec: Vec<ArrayVec<[i32; 4]>> = Vec::new();

    for i in 1..=100_000_000_000u64 {
        vec.push(array_vec![1, 2, 3, 4]);
    }
}

fn bench_fibs(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()

        .summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group(
        "benchmark ",
    );
    group.sampling_mode(SamplingMode::Auto);
    group.plot_config(plot_config);
    group.bench_function("A -> smallvec", move |b| {
        // This will avoid timing the create_data call.
        b.iter(||smallvec)
    });
    group.bench_function("B -> tinyvec", move |b| {
        b.iter(||tinyvec)
    });
    group.bench_function("C -> arrayvec", move |b| {
        b.iter(||arrayvec)
    });
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);