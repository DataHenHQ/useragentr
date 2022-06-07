use serde::Deserialize;
use super::browser_variant::BrowserVariant;

// Browser's user agent configuration
#[derive(Deserialize, Debug)]
pub struct Browser {
    pub probability: f64,
    pub id: String,
    pub ua_format: String,
    pub variants: Vec<BrowserVariant>,

    #[serde(skip_serializing, skip_deserializing)]
    pub variant_probability_limit: f64, 
}
