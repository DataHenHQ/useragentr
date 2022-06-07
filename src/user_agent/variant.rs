use std::collections::HashMap;

use crate::utils;

use super::error::Error;

pub trait DataAdder {
    // add_data_vars adds the data variables to a map
    fn add_data_vars(&self, prefix: &str, data: &mut HashMap<String, String>) -> Result<(), Error>;
}

// Variant represents a variant
pub type Variant = HashMap<String, Vec<String>>;

// add_data_vars adds the data variables to a map
impl DataAdder for Variant {
    fn add_data_vars(&self, prefix: &str, data: &mut HashMap<String, String>) -> Result<(), Error> {
        for (key, vals) in self.iter() {
            let real_key = format!("{}:{}", prefix, key);

            let v = match utils::random_vector_element(&vals) {
                Some(v) => v,
                None => return Err(Error::AtLeastOneDataValueRequired(key.to_string())),
            };

            data.insert(real_key, v.to_string());
        }

        Ok(())
    }
}
