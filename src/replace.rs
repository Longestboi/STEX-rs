use regex::Regex;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unknown template error")]
    DataError,
    #[error("Could not find token {0}")]
    TokenNotFound(String),
    #[error("Unknown token error")]
    TokenError,
    #[error("Unknown core error")]
    ReplacerError,
}

pub struct Replacer;

impl Replacer {
    pub fn replace_token_in_string(
        data: impl TryInto<String>,
        token: impl TryInto<String>,
        replacer: impl TryInto<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let sdata = match data.try_into() as Result<String, _> {
            Ok(data) => data,
            Err(_) => return Err(Box::new(Error::DataError)),
        };

        let stok = match token.try_into() as Result<String, _> {
            Ok(token) => token,
            Err(_) => return Err(Box::new(Error::TokenError)),
        };

        let srepl = match replacer.try_into() as Result<String, _> {
            Ok(replacement) => replacement,
            Err(_) => return Err(Box::new(Error::ReplacerError)),
        };

        let regex_pat = format!("(.*)//.*{}.*", stok);
        let re = Regex::new(&regex_pat)?;

        if let Some(testing) = re.captures(&sdata) {
            let before_tag = match testing.get(1) {
                Some(e) => e.as_str(),
                None => return Err(Box::new(Error::TokenError)),
            };

            let replacement_mod = srepl
                .split("\n")
                .map(|f| format!("{before_tag}{f}"))
                .collect::<Vec<String>>()
                .join("\n");

            // println!("{}", test.as_str().to_string());
            Ok(sdata.replace(&format!("{before_tag}// {stok}"), &replacement_mod))
        } else {
            return Err(Box::new(Error::TokenNotFound(stok)));
        }
    }
}
