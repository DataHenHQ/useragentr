pub mod config;
pub mod error;
pub mod device;
pub mod os;
pub mod os_variant;
pub mod variant;
pub mod browser;
pub mod browser_variant;

use std::{path::PathBuf, fs::File, io::BufReader, marker::PhantomData, fmt};

use config::Config;

use crate::user_agent::config::DeviceGetter;

use self::config::Configurator;


#[derive(Clone, Debug)]
pub enum Type {
    Desktop,
    Mobile,
    Tablet,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Type::Desktop => write!(f, "desktop"),
           Type::Mobile => write!(f, "mobile"),   
           Type::Tablet => write!(f, "tablet"),   
       }
    }
}

pub trait Probability {
    // get probabilty field
    fn get_probability(&self) -> f64;
}

pub struct UserAgent {
    dummy: PhantomData<bool>,
    config: Config,
}



impl UserAgent {
    /// create a new UserAgent
    /// 
    /// # Arguments:
    /// 
    /// path: specify the path to the user agent json file. If [`None`] is specified, then the default one will be used.
    /// 
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let ua = UserAgent::new(None).unwrap();
    /// let result = ua.generate_ua(&Type::Desktop).unwrap();
    /// let re = Regex::new(r"Mozilla/5.0").unwrap();  
    /// assert_eq!(re.is_match(&result), true);
    /// ```
    /// 
    /// To use a custom user agent config json file:
    /// ``` 
    /// let ua = UserAgent::new(Some(PathBuf::from("/path/to/your/ua-config.json"))).unwrap();
    /// let result = ua.generate_ua(&Type::Desktop).unwrap();
    /// let re = Regex::new(r"Mozilla/5.0").unwrap();  
    /// assert_eq!(re.is_match(&result), true);
    /// ```
    pub fn new(path: Option<PathBuf>) -> Result<Self, error::Error> {
        Ok(Self{
            dummy: PhantomData{},
            config: Self::build_config(path)?
        })
    }

    pub fn build_config(path: Option<PathBuf>) -> Result<Config, error::Error> {
        // load the config file from path, or from default
        let mut config: Config = match path {
            Some(file_path) => {
                let file = File::open(file_path).unwrap();
                let reader = BufReader::new(file);
                serde_json::from_reader(reader)?
            },
            None => serde_json::from_slice(config::DEFAULT_BYTES)?
        };

        // init config and recursively all children of it
        config.init();

        Ok(config)
    }

    pub fn generate_ua(&self, ua_type: &Type) -> Result<String, error::Error> {
        let config = self.config();
        let device = match config.get_device(ua_type){
            Some(v) => v,
            None => return Err(error::Error::DeviceNotFound(ua_type.to_string())),
        };
        device.build_user_agent()
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::user_agent::{UserAgent, Type};
    use regex::Regex;

    #[test]
    fn it_generates_desktop_ua() {
        let ua = UserAgent::new(None).unwrap();
        let result = ua.generate_ua(&Type::Desktop).unwrap();
        let re = Regex::new(r"Mozilla/5.0").unwrap();
        
        assert_eq!(re.is_match(&result), true);
    }

}
