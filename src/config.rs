use serde_derive::Deserialize;

use std::fs::File;
use std::io::prelude::*;

use crate::features::keyboard_click::KeyboardClickConfig;
use crate::features::scroll::ScrollConfig;

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub scroll: Option<ScrollConfig>,
    pub keyboard_click: Option<KeyboardClickConfig>,
}

macro_rules! generate_loader {
    ($struct:tt) => {
        impl $struct {
            pub fn load() -> $struct {
                let filename = stringify!($struct).to_string() + ".toml";
                let filename = &filename;

                let mut file = File::open(filename)
                    .unwrap_or_else(|_| panic!("File {} doesn't exist", filename));
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .unwrap_or_else(|_| panic!("{} is not UTF-8 formatted", filename));
                toml::from_str(&contents)
                    .unwrap_or_else(|_| panic!("{} is not a valid configuration file", filename))
            }
        }
    };
}

generate_loader!(Config);
