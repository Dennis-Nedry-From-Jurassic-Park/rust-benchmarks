extern crate criterion;
extern crate crossbeam_utils;
extern crate contrie;
extern crate easy_parallel;
extern crate chashmap;

use easy_parallel::Parallel;

use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../src/YakShaverBuilder.rs"]
mod yshb;
use yshb::{YakShaver, YakShaverBuilder};
#[path = "../src/YakShaver.rs"]
mod ysh;
//use ysh::{YakShaverInit,YakShaver as YakShaver2 };

//use YakShaverDeriveBuilder;
use crate::ysh::YakShaverInit;

use derive_builder::Builder;
#[derive(Builder, Debug)]
struct YakShaverDerive {
    clipper_size: u32,
    gas_powered_clippers: bool,
    solar_powered_clippers: bool,
    color_to_dye_yak: String,
    clipper_color: String
}

use typed_builder::TypedBuilder;
#[derive(TypedBuilder)]
struct YakShaverDeriveTyped {
    clipper_size: u32,
    gas_powered_clippers: bool,
    solar_powered_clippers: bool,
    color_to_dye_yak: String,
    clipper_color: String
}
// TODO: https://github.com/raiden-rs/safe-builder

#[path = "../src/Flyweight.rs"]
mod flyweight;
use flyweight::FlyweightFactory;
use crossbeam_utils::thread;
use contrie::ConMap;
use chashmap::CHashMap;

fn yak_shaver_builder() {
    let mut vec = vec![];
    for _ in 0..100_000_000_000u64 {
        let yak_shaver = yshb::YakShaverBuilder::new()
            .clipper_size(4)
            .color_to_dye_yak(String::from("hot pink"))
            .clipper_color(String::from("red"))
            .build();
        vec.push(yak_shaver);
    }
}
fn init_state() {
    let mut vec = vec![];
    for _ in 0..100_000_000_000u64 {
        let yak_shaver = YakShaverInit {
            clipper_size: 4,
            color_to_dye_yak: String::from("hot pink"),
            clipper_color: String::from("red"),
            ..Default::default()
        }.init();
        vec.push(yak_shaver);
    }
}

fn derive_builder() {
    let mut vec = vec![];
    for _ in 0..100_000_000_000u64 {
        let ch = YakShaverDeriveBuilder::default()
            .clipper_size(1u32)
            .gas_powered_clippers(true)
            .solar_powered_clippers(true)
            .color_to_dye_yak("fsafsa".to_string())
            .clipper_color("fsafsa".to_string())
            .build();
        vec.push(ch);
    }
}
fn typed_builder() {
    let mut vec = vec![];
    for _ in 0..100_000_000_000u64 {
        let tb = YakShaverDeriveTyped::builder()
            .clipper_size(1u32)
            .gas_powered_clippers(true)
            .solar_powered_clippers(true)
            .color_to_dye_yak("fsafsa".to_string())
            .clipper_color("fsafsa".to_string());
        vec.push(tb);
    }
}

fn easy_parallel_contrie_map() {
    let map = CHashMap::new();

    Parallel::new()
        .each(0..100_000_000_000u64, |i|
            map.insert(i.to_string(), YakShaverInit {
                clipper_size: 4,
                color_to_dye_yak: String::from("hot pink"),
                clipper_color: String::from("red"),
                ..Default::default()
            }.init())
        )
        .run();
}

fn contrie_map() {
    let map = ConMap::new();

    thread::scope(|scope| {
        for i in 0..100_000_000_000u64 {
            let yak_shaver = YakShaverInit {
                clipper_size: 4,
                color_to_dye_yak: String::from("hot pink"),
                clipper_color: String::from("red"),
                ..Default::default()
            }.init();
            map.insert(i.to_string(), yak_shaver);
        }
    }).unwrap();
}

fn flyweight_factory() {
    let mut factory = FlyweightFactory::default();
    factory.with_capacity();

    for i in 0..100_000_000_000u64 {
        factory.ins(&*i.to_string());
    }
}

// https://github.com/colin-kiegel/rust-derive-builder
fn bench_fibs(c: &mut Criterion) {
    for i in 0..5 {
        let mut group = c.benchmark_group(
            "flyweight (easy_parallel_c) n-".to_owned() + &i.to_string(),
        );
        group.bench_function("A -> init_state", move |b| {
            b.iter(||init_state)
        });
        group.bench_function("B -> yak_shaver_builder", move |b| {
            b.iter(||yak_shaver_builder)
        });
        group.bench_function("TB -> typed_builder", move |b| {
            b.iter(||typed_builder)
        });
        group.bench_function("C -> flyweight_factory", move |b| {
            b.iter(||flyweight_factory)
        });
        group.bench_function("D -> derive_builder", move |b| {
            b.iter(||derive_builder)
        });
        group.bench_function("CM -> contrie_map", move |b| {
            b.iter(||contrie_map)
        });
        group.bench_function("EP -> easy_parallel_contrie_map", move |b| {
            b.iter(||easy_parallel_contrie_map)
        });
        group.finish();
    }
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);