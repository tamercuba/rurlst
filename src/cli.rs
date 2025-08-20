use clap::error::Result;

use crate::{address::Address, headers::RequestHeader, request_protocol::RequestProtocol};
use std::{fmt, net::TcpStream};

#[derive(Debug)]
pub struct Request<'a> {
    addr: &'a Address,
    headers: &'a RequestHeader,
    method: String,
    protocol: RequestProtocol,
}

impl<'a> Request<'a> {
    pub fn new(
        addr: &'a Address,
        headers: &'a RequestHeader,
        method: Option<String>,
        protocol: RequestProtocol,
    ) -> Self {
        let method = method.unwrap_or_else(|| "GET".to_string());
        Request {
            addr,
            headers,
            method,
            protocol,
        }
    }
}

impl<'a> fmt::Display for Request<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Running request...")?;
        write!(f, "{}", self.addr)?;
        writeln!(f, "Headers: {:?}", self.headers)?;
        writeln!(f, "Method: {:?}", self.method)?;
        writeln!(f, "Protocol: {}", self.protocol)?;
        Ok(())
    }
}

pub fn run(r: Request) -> Result<(), ()> {
    println!("{r}");

    let tcp_socket = TcpStream::connect(r.addr);

    match tcp_socket {
        Ok(stream) => {
            println!("Connected! {}", stream.peer_addr().unwrap())
        }
        Err(e) => {
            println!("Error {e}")
        }
    }

    Ok(())
}
