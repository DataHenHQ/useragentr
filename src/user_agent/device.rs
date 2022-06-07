use super::{browser::Browser, os::OS};
use crate::{user_agent::error::Error, utils};
use serde::Deserialize;
use std::collections::HashMap;

// Device's user agent configuration
#[derive(Deserialize, Debug)]
pub struct Device {
    pub id: String,
    pub oses: Vec<OS>,
    pub browsers: HashMap<String, Browser>,

    #[serde(skip_serializing, skip_deserializing)]
    pub os_probability_limit: f64,
}

impl Device {

    // oses_init recusively inits all oses
    fn oses_init(&mut self) {
        self.os_probability_limit = utils::calculate_probability_limit(self.oses.iter().collect());
        for os in &mut self.oses {
            os.init();
        }
    }

    // browsers_init recusively inits all browsers
    fn browsers_init(&mut self)  {
        for ( _, browser) in &mut self.browsers {
            browser.init();
        }
    }

    // init the device
    pub fn init(&mut self) {
        self.oses_init();
        self.browsers_init();
    }

    // gets a random os
    fn random_os(&self) -> Result<&OS, Error> {
        match utils::random_weighted(
            self.oses.iter().collect(),
            self.os_probability_limit.clone(),
        ) {
            Some(v) => Ok(v),
            None => Err(Error::NoAvailableOS),
        }
    }

    // gets a random browser
    fn random_browser(&self, browser_ids: &Vec<String>) -> Result<&Browser, Error> {
        if browser_ids.len() < 1 {
            return Err(Error::NoAvailableBrowser);
        };

        // find browsers by id
        let mut browsers: Vec<&Browser> = vec![];
        let mut limit: f64 = 0_f64;

        for browser_id in browser_ids {
            let browser = &self.browsers[browser_id];
            browsers.insert(0, browser);
            limit += browser.probability;
        }

        // gets random browser by it's match probability
        match utils::random_weighted(browsers, limit) {
            Some(v) => Ok(v),
            None => return Err(Error::NoAvailableBrowser),
        }
    }

    // builds a random user agent string
    pub fn build_user_agent(&self) -> Result<String, Error> {
        // Set seed of randomness on thread local
        utils::reseed_randomness();

        // get os variant
        let os = self.random_os()?;
        let os_variant = os.random_variant()?;

        // get browser variant
        let browser = self.random_browser(&os_variant.browser_ids)?;
        let browser_variant = browser.random_variant()?;

        // add data vars
        let mut data: HashMap<String, String> = HashMap::new();
        os_variant.add_vars(&mut data)?;
        browser_variant.add_vars(&mut data)?;

        // build user agent
        if browser.ua_format == "" {
            return Err(Error::NoUAFormatForBrowser(browser.id.clone()));
        }

        Ok(utils::apply_named_format(&browser.ua_format, &data))
    }
}
