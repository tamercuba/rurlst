use std::{
    error::Error,
    fmt,
    str::{self, FromStr},
};

#[derive(Clone, Debug)]
pub struct RequestHeader(pub String);

#[derive(Clone, Debug)]
pub enum RequestProtocol {
    Http1_1,
}

#[derive(Clone, Debug)]
pub enum RequestProtocolParsingError {
    InvalidProtocol(String),
}

impl Error for RequestProtocolParsingError {}

impl ToString for RequestProtocol {
    fn to_string(&self) -> String {
        match self {
            RequestProtocol::Http1_1 => "HTTP/1.1".to_string(),
        }
    }
}

impl FromStr for RequestProtocol {
    type Err = RequestProtocolParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.1" => Ok(RequestProtocol::Http1_1),
            _ => Err(RequestProtocolParsingError::InvalidProtocol(s.to_string())),
        }
    }
}

impl fmt::Display for RequestProtocolParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestProtocolParsingError::InvalidProtocol(protocol) => {
                write!(f, "Invalid request protocol: {}", protocol)
            }
        }
    }
}

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
