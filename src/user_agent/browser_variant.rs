use std::collections::HashMap;

use super::super::user_agent::error::Error;
use super::Probability;
use super::variant::{DataAdder, Variant};
use serde::Deserialize;

// BrowserVariant represents an user agent configuration for a specific browser variant
#[derive(Deserialize, Debug, Clone)]
pub struct BrowserVariant {
    pub probability: f64,
    pub id: String,
    pub data: Variant,
}

impl BrowserVariant {
    // add_vars adds the browser data variables to a hashmap
    pub fn add_vars(&self, data: &mut HashMap<String, String>) -> Result<(), Error> {
        match self.data.add_data_vars("browser", data) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::BrowserVariant(self.id.clone(), Box::new(e))),
        }
    }
}

impl Probability for BrowserVariant {
    fn get_probability(&self) -> f64 {
        self.probability
    }
}

