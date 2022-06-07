use serde::{Deserialize};
use super::variant::Variant;

// OSVariant represents an user agent configuration for a specific operating system variant
#[derive(Deserialize, Debug)]
pub struct OSVariant {
    pub id: String,
    pub probability: f64,
    pub signatures: Vec<String>,
    pub browser_ids: Vec<String>,
    pub data: Variant,
}