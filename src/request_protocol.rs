use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum RequestProtocol {
    Http1_1,
}

impl RequestProtocol {
    pub fn new(protocol: &str) -> Result<Self, RequestProtocolParsingError> {
        match protocol {
            "HTTP/1.1" => Ok(RequestProtocol::Http1_1),
            _ => Err(RequestProtocolParsingError::InvalidProtocol(
                protocol.to_string(),
            )),
        }
    }
}

impl fmt::Display for RequestProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let protocol_str = match self {
            RequestProtocol::Http1_1 => "HTTP/1.1",
        };
        write!(f, "{}", protocol_str)
    }
}

#[derive(Clone, Debug)]
pub enum RequestProtocolParsingError {
    InvalidProtocol(String),
}

impl Error for RequestProtocolParsingError {}

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
