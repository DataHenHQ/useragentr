use crate::utils;

use super::super::user_agent::error::Error;
use super::browser_variant::BrowserVariant;
use super::Probability;
use serde::Deserialize;

// Browser's user agent configuration
#[derive(Deserialize, Debug, Clone)]
pub struct Browser {
    pub probability: f64,
    pub id: String,
    pub ua_format: String,
    pub variants: Vec<BrowserVariant>,

    #[serde(skip_serializing, skip_deserializing)]
    pub variant_probability_limit: f64,
}

impl Browser {

    // initializes the browser
    pub fn init(&mut self){
        self.variant_probability_limit = utils::calculate_probability_limit(self.variants.iter().collect());
    }

    // get a random browser variant
    pub fn random_variant(&self) -> Result<&BrowserVariant, Error> {
        match utils::random_weighted(
            self.variants.iter().collect(),
            self.variant_probability_limit.clone(),
        ) {
            Some(v) => Ok(v),
            None => Err(Error::NoAvailableBrowserVariant),
        }
    }
}

impl Probability for Browser {
    fn get_probability(&self) -> f64 {
        self.probability
    }
}
