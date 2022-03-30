// TODO: not worked with bencher
// #![feature(allocator_api)]
// use bumpalo::{Bump, collections::Vec as Vector};
//
// extern crate criterion;
// extern crate slab;
// extern crate rand;
//
// use criterion::{criterion_group, criterion_main, Criterion};
// use slab::Slab;
// use nested::Nested;
//
// use sharded_slab::Slab as ShardSlab;
//
// fn nested_vec() {
//     let mut nested_vec = Nested::<String>::new();
//
//     for _ in 1..=100_000_000_000u64 {
//         nested_vec.push("hello");
//     }
// }
//
// fn bumpalo_vec() {
//     let mut bump = Bump::new();
//
//     let mut bumpalo_vec = Vector::new_in(&bump);
//
//     for i in 1..=100_000_000_000u64 {
//         bumpalo_vec.insert(i as usize, "hello");
//     }
// }
//
// fn shard_slab<'a>() -> ShardSlab<&'a str> {
//     let sl: ShardSlab<&str> = ShardSlab::new();
//
//     for _ in 1..=100_000_000_000i64 {
//         sl.insert("hello"); // &*rand_word
//     }
//     sl
// }
//
// fn slab<'a>() -> Slab<&'a str> {
//     let mut s: Slab<&str> = Slab::new();
//
//     for _ in 1..=100_000_000_000i64 {
//         s.insert("hello"); // &*rand_word
//     }
//     s
// }
// fn vec<'a>() -> Vec<&'a str>{
//     let mut v = Vec::new();
//
//     for _ in 1..=100_000_000_000i64 {
//         v.push("hello");
//     }
//     v
// }
//
// fn bench_fibs(c: &mut Criterion) {
//     let mut group = c.benchmark_group(
//         "benchmark structs",
//     );
//     group.bench_function("A -> slab", move |b| {
//         // This will avoid timing the create_data call.
//         b.iter(||slab)
//     });
//     group.bench_function("B -> vec", move |b| {
//         b.iter(||vec)
//     });
//     group.bench_function("B2 -> bumpalo_vec", move |b| {
//         b.iter(||bumpalo_vec)
//     });
//     group.bench_function("N -> nested_vec", move |b| {
//         b.iter(||nested_vec)
//     });
//     group.bench_function("C -> shard_slab", move |b| {
//         b.iter(||shard_slab)
//     });
//     // TODO Criterion.rs ERROR: At least one measurement of benchmark prime_number_generator-vars :
//     group.finish();
// }
//
// criterion_group!(benches, bench_fibs);
// criterion_main!(benches);