use serde::{Deserialize};

pub static DEFAULT_BYTES: &'static [u8] = include_bytes!("./../../config/default-ua-config.json");

#[derive(Deserialize, Debug)]
pub struct Config {
    pub desktop: serde_json::Value,
    pub tablet: serde_json::Value,
    pub mobile: serde_json::Value,
}