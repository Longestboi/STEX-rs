use serde::de;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Errors {
    Message(String),
}

impl Display for Errors {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Errors::Message(msg) => formatter.write_str(msg),
        }
    }
}

impl serde::ser::StdError for Errors {}

impl de::Error for Errors {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Errors::Message(msg.to_string())
    }
}