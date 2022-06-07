use serde::Deserialize;
use super::os_variant::OSVariant;

// Operating system's user agent configuration
#[derive(Deserialize, Debug)]
pub struct OS {
    pub id: String,
    pub probability: f64,
    pub variants: Vec<OSVariant>,

    #[serde(skip_serializing, skip_deserializing)]
    pub variant_probability_limit: f64,
}


