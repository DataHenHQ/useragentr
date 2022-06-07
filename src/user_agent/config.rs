use serde::{Deserialize};
use super::device::Device;

pub static DEFAULT_BYTES: &'static [u8] = include_bytes!("./../../config/default-ua-config.json");

// User agents configuration
#[derive(Deserialize, Debug)]
pub struct Config {
    pub desktop: Option<Device>,
    pub tablet: Option<Device>,
    pub mobile: Option<Device>,
}