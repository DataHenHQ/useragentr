use super::{variant::{DataAdder, Variant}, Probability};
use crate::{user_agent::error::Error, utils};
use serde::Deserialize;
use std::collections::HashMap;

// OSVariant represents an user agent configuration for a specific operating system variant
#[derive(Deserialize, Debug)]
pub struct OSVariant {
    pub id: String,
    pub probability: f64,
    pub signatures: Vec<String>,
    pub browser_ids: Vec<String>,
    pub data: Variant,
}

impl OSVariant {
    // add_vars adds the os data variables to a map
    pub fn add_vars(&self, data: &mut HashMap<String, String>) -> Result<(), Error> {
        self.data
            .add_data_vars("os", data)
            .map_err(|e| Error::OSVariant(self.id.clone(), Box::new(e)))?;

        let signature = self.random_signature()?;

        data.insert("os:signature".to_string(), signature.to_string());

        Ok(())
    }

    // random_signature gets a random signature
    pub fn random_signature(&self) -> Result<&String, Error> {
        match utils::random_vector_element(&self.signatures) {
            Some(v) => Ok(v),
            None => Err(Error::AtLeastOneSignatureRequired),
        }
    }
}

impl Probability for OSVariant {
    fn get_probability(&self) -> f64 {
        self.probability
    }
}

