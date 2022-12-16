use crate::utils;
use serde::Deserialize;

use super::super::user_agent::error::Error;
use super::os_variant::OSVariant;
use super::Probability;

// Operating system's user agent configuration
#[derive(Deserialize, Debug, Clone)]
pub struct OS {
    pub id: String,
    pub probability: f64,
    pub variants: Vec<OSVariant>,

    #[serde(skip_serializing, skip_deserializing)]
    pub variant_probability_limit: f64,
}

impl OS {

    // init the os
    pub fn init(&mut self){
        self.variant_probability_limit = utils::calculate_probability_limit(self.variants.iter().collect());
    }

    // gets a random os variant
    pub fn random_variant(&self) -> Result<&OSVariant, Error> {
        match utils::random_weighted(
            self.variants.iter().collect(),
            self.variant_probability_limit.clone(),
        ) {
            Some(v) => Ok(v),
            None => Err(Error::NoAvailableOSVariant),
        }
    }
}

impl Probability for OS {
    fn get_probability(&self) -> f64 {
        self.probability
    }
}
