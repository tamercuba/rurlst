use std::{
    error::Error,
    fmt,
    str::{self, FromStr},
};

#[derive(Clone, Debug)]
pub struct RequestHeader(pub String);

impl RequestHeader {
    pub fn new(s: String) -> Self {
        RequestHeader(s)
    }
}

#[derive(Debug, Clone)]
pub enum RequestHeaderParsingError {
    Todo,
}

impl Error for RequestHeaderParsingError {}

impl fmt::Display for RequestHeaderParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequestHeaderParsingError::Todo => {
                write!(f, "Request header parsing is not implemented yet")
            }
        }
    }
}

impl FromStr for RequestHeader {
    type Err = RequestHeaderParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = match String::from_str(s) {
            Ok(v) => v,
            Err(_) => return Err(RequestHeaderParsingError::Todo),
        };

        return Ok(RequestHeader::new(data));
    }
}
