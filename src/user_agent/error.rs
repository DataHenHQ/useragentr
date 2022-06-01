use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Config file parsing error: `{0}`")]
    ParseError(String),
    
    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::ParseError(err.to_string())
    }
}