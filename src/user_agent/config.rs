use super::{device::Device, Type};

pub static DEFAULT_BYTES: &'static [u8] = include_bytes!("./../../config/default-ua-config.json");

use std::collections::HashMap;

// User Agent configuration
pub type Config = HashMap<String, Device>;

pub trait DeviceGetter {
    fn get_device(&self, ua_type: &Type) -> Option<&Device>;
}

impl DeviceGetter for Config {
    fn get_device(&self, ua_type: &Type) -> Option<&Device> {
        self.get(&ua_type.to_string())
    }
}

pub trait Configurator {
    // init user agent config and recursively init all devices
    fn init(&mut self);
}

impl Configurator for Config {
    fn init(&mut self){
        for (_, device) in self {
            device.init();
        }
    }
}
