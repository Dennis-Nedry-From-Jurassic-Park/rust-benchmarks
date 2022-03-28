#[macro_use]
extern crate derive_builder;

mod YakShaver;
mod YakShaverBuilder;

extern crate num_primes;
extern crate tinyvec;
extern crate dashmap;
extern crate chashmap;

use num_primes::Generator;
use tinyvec::TinyVec;
use tinyvec::tiny_vec;

#[path = "./YakShaverBuilder.rs"]
mod yshb2;
mod flyweight;

use yshb2::{YakShaver as y2, YakShaverBuilder as yb2};

use derive_builder::Builder;
#[derive(Builder, Debug)]
pub struct YakShaverDerive {
    clipper_size: u32,
    gas_powered_clippers: bool,
    solar_powered_clippers: bool,
    color_to_dye_yak: String,
    clipper_color: String
}

fn main() {

}