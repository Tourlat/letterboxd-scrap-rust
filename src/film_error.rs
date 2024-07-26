use reqwest;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum FilmError {
    ParseError,
    SelectorError,
    AttributeError,
    ReqwestError(reqwest::Error),
}

impl fmt::Display for FilmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FilmError::ReqwestError(ref err) => write!(f, "Request error: {}", err),
            FilmError::ParseError => write!(f, "Parsing error"),
            FilmError::SelectorError => write!(f, "Selector error"),
            FilmError::AttributeError => write!(f, "Attribute error"),
        }
    }
}

impl Error for FilmError {}

impl From<reqwest::Error> for FilmError {
    fn from(err: reqwest::Error) -> FilmError {
        FilmError::ReqwestError(err)
    }
}
