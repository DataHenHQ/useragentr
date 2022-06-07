use std::collections::HashMap;
use serde::{Deserialize};
use super::{os::OS, browser::Browser};

// Device's user agent configuration
#[derive(Deserialize, Debug)]
pub struct Device {
    pub id: String,
    pub oses: Vec<OS>,
    pub browsers: HashMap<String, Browser>,

    #[serde(skip_serializing, skip_deserializing)]
    pub os_probability_limit: f64,
}
