extern crate slab;
extern crate rand;

use smallvec::{SmallVec, smallvec};
use tinyvec::{array_vec, ArrayVec, TinyVec};
use tinyvec::tiny_vec;

use {
    glassbench::*,
};

fn smallvec() {
    let mut vec: Vec<SmallVec<[u64; 4]>> = Vec::new();

    for i in 1..=100_000_000u64 {
        vec.push(smallvec![i, i, i, i]);
    }
}
fn tinyvec() {
    let mut vec: Vec<TinyVec<[u64; 4]>> = Vec::new();

    for i in 1..=100_000_000u64 {
        vec.push(tiny_vec![i, i, i, i]);
    }
}
fn arrayvec() {
    let mut vec: Vec<ArrayVec<[u64; 4]>> = Vec::new();

    for i in 1..=100_000_000u64 {
        vec.push(array_vec![i, i, i, i]);
    }
}

fn bench_flat_structs(bench: &mut Bench) {
    bench.task("smallvec", |task| {
        task.iter(|| {
            smallvec();
        });
    });
    bench.task("tinyvec", |task| {
        task.iter(|| {
            tinyvec();
        });
    });
    bench.task("arrayvec", |task| {
        task.iter(|| {
            arrayvec();
        });
    });
}

glassbench!(
    "benchmark flat structs",
    bench_flat_structs,
);