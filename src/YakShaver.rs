pub struct YakShaver {
    clipper_size: u32,
    gas_powered_clippers: bool,
    solar_powered_clippers: bool,
    color_to_dye_yak: String,
    clipper_color: String,
}

pub struct YakShaverInit {
    pub clipper_size: u32,
    pub gas_powered_clippers: bool,
    pub solar_powered_clippers: bool,
    pub color_to_dye_yak: String,
    pub clipper_color: String,
    #[doc(hidden)]
    pub __non_exhaustive: () // This is a hack, we might be able to stop using it in the future.
}

impl Default for YakShaverInit {
    fn default() -> Self {
        Self {
            clipper_size: 3,
            gas_powered_clippers: false,
            solar_powered_clippers: true,
            color_to_dye_yak: String::from("brown"),
            clipper_color: String::from("black"),
            __non_exhaustive: (),
        }
    }
}

impl YakShaverInit {
    pub fn init(self) -> YakShaver {
        YakShaver {
            clipper_size: self.clipper_size,
            gas_powered_clippers: self.gas_powered_clippers,
            solar_powered_clippers: self.solar_powered_clippers,
            color_to_dye_yak: self.color_to_dye_yak,
            clipper_color: self.clipper_color,
        }
    }
}