pub mod config;
pub mod error;
pub mod device;
pub mod os;
pub mod os_variant;
pub mod variant;
pub mod browser;
pub mod browser_variant;

use std::{path::PathBuf, fs::File, io::BufReader, marker::PhantomData};

use config::Config;


#[derive(Clone, Debug)]
pub enum Type {
    Desktop,
    Mobile,
    Googlebot2,
}

pub struct UserAgent {
    dummy: PhantomData<bool>,
    config: Config,
}

impl UserAgent {
    pub fn new(path: Option<PathBuf>) -> Result<Self, error::Error> {
        Ok(Self{
            dummy: PhantomData{},
            config: Self::build_config(path)?
        })
    }

    pub fn build_config(path: Option<PathBuf>) -> Result<Config, error::Error> {
        match path {
            Some(file_path) => {
                let file = File::open(file_path).unwrap();
                let reader = BufReader::new(file);
                Ok(serde_json::from_reader(reader)?)
            },
            None => Ok(serde_json::from_slice(config::DEFAULT_BYTES)?)
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}