use thiserror::Error;

// use super::Type;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Config file parsing error: `{0}`")]
    ParseError(String),

    #[error("Device not found for: `{0}`")]
    DeviceNotFound(String),

    #[error("there is no available OS")]
    NoAvailableOS,

    #[error("there is no available OS Variant")]
    NoAvailableOSVariant,

    #[error("there is no available Browser")]
    NoAvailableBrowser,

    #[error("there is no available Browser Variant")]
    NoAvailableBrowserVariant,

    #[error("at least one value is required for `{0}` data variable")]
    AtLeastOneDataValueRequired(String),

    #[error("`{0}` Browser variant: `{1}`")]
    BrowserVariant(String, Box<Self>),

    #[error("`{0}` OS variant: `{1}`")]
    OSVariant(String, Box<Self>),

    #[error("at least one signature is required")]
    AtLeastOneSignatureRequired,

    #[error("No ua_format found for `{0}` browser")]
    NoUAFormatForBrowser(String),
    
    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::ParseError(err.to_string())
    }
}
