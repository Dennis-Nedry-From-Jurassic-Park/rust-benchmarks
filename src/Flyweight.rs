use std::collections::HashMap;
use std::rc::Rc;

use dashmap::DashMap;
use chashmap::CHashMap;

#[derive(Default)]
pub struct FlyweightFactory {
    map: CHashMap<String, Rc<Flyweight>>
    // https://docs.rs/chashmap/2.2.2/chashmap/struct.CHashMap.html
    // https://docs.rs/dashmap/4.0.2/dashmap/struct.DashMap.html
    // https://crates.io/crates/sharded
}

impl FlyweightFactory {
    pub fn with_capacity(&mut self) {
        self.map = CHashMap::with_capacity(100_000_000_000);
    }
    pub fn ins(&mut self, key: &str) {
        self.map.insert(key.to_string(), Rc::new(Flyweight::new()));
    }

    pub fn get(&mut self, key: &str) -> Rc<Flyweight> {
        if !self.map.contains_key(key) {
            self.map.insert(key.to_string(), Rc::new(Flyweight::new()));
        }
        self.map.get(key).unwrap().clone()
    }
}

pub struct Flyweight {
    clipper_size: u32,
    gas_powered_clippers: bool,
    solar_powered_clippers: bool,
    color_to_dye_yak: String,
    clipper_color: String
}

impl Flyweight {
    pub fn new() -> Self {
        Self {
            clipper_size: 1u32,
            gas_powered_clippers: true,
            solar_powered_clippers: true,
            color_to_dye_yak: "fsafsa".to_string(),
            clipper_color: "fsafsa".to_string()
        }
    }
}

pub struct Viewer {
    uniq_key: Rc<Flyweight>
}

impl Viewer {
    pub fn new(uniq_key: Rc<Flyweight>) -> Self {
        Self {
            uniq_key
        }
    }
    pub fn display(&self) -> String {
        format!("clipper_color: {}", self.uniq_key.clipper_color)
    }
}