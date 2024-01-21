use std::{path::Path, error::Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReplacerTokenError {
    #[error("Data string could not be ")]
    DataError,
    #[error("Could not find fixed value, unlikely to be a GBA game")]
    TokenError,
    #[error("Unknown Error")]
    ReplacerError,
}

pub struct Replacer;

impl Replacer {
    pub fn replace_token_in_file(file_path: impl AsRef<Path>, token: impl TryInto<String>, replacer: impl TryInto<String>)  -> Result<String, Box<dyn Error>> {
        let file_string = std::fs::read_to_string(file_path)?;

        Self::replace_token_in_string(file_string, token, replacer)
    }

    pub fn replace_token_in_string(data: impl TryInto<String>, token: impl TryInto<String>, replacer: impl TryInto<String>) -> Result<String, Box<dyn Error>> {

        let sdata = match data.try_into() as Result<String, _> {
            Ok(data) => data,
            Err(_) => return Err(Box::new(ReplacerTokenError::DataError)),
        };

        let stok = match token.try_into() as Result<String, _> {
            Ok(token) => token,
            Err(_) => return Err(Box::new(ReplacerTokenError::TokenError)),
        };

        let srepl = match replacer.try_into() as Result<String, _> {
            Ok(replacement) => replacement,
            Err(_) => return Err(Box::new(ReplacerTokenError::ReplacerError)),
        };

        Ok(sdata.replace(&format!("// {}", &stok), &srepl))
    } 
}