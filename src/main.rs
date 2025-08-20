use std::str::FromStr;

use crate::cli::{Request, run};
use crate::request_protocol::RequestProtocol;
use crate::{address::Address, headers::RequestHeader};
use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};

mod address;
mod cli;
mod headers;
mod request_protocol;

pub fn get_arguments() -> ArgMatches {
    Command::new("rURLst - A Blazingly slow and unoptimized cURL implemented in rust")
        .about("It helps you to make HTTP requests")
        .version("0.0.1")
        .author("Tâmer Pinheiro Cuba")
        .arg(
            Arg::new("url")
                .index(1)
                .required(true)
                .value_parser(value_parser!(Address)),
        )
        .arg(
            Arg::new("headers")
                .short('H')
                .long("header")
                .help("Custom headers to include in the request")
                .action(ArgAction::Append)
                .value_parser(value_parser!(RequestHeader)),
        )
        .get_matches()
}

fn main() {
    let matches = get_arguments();
    let url = matches.get_one::<Address>("url").unwrap();

    // Concatenate headers into a RequestHeader
    let headers = matches
        .get_many::<RequestHeader>("headers")
        .unwrap_or_default()
        .fold(RequestHeader::new(String::new()), |mut acc, header| {
            acc.0.push_str(&header.0);
            acc.0.push('\r');
            acc.0.push('\n');
            acc
        });
    let http_protocol = RequestProtocol::new("HTTP/1.1").expect("Failed to parse HTTP protocol");
    let request = Request::new(url, &headers, None, http_protocol);

    let _response = run(request);
}
