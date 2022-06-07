use serde::Deserialize;
use super::variant::Variant;

// BrowserVariant represents an user agent configuration for a specific browser variant
#[derive(Deserialize, Debug)]
pub struct BrowserVariant {
    pub probability: f64,
    pub id: String,
    pub data: Variant,
}
