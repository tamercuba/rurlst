use crate::address::Address;
use crate::cli::run;
use clap::{Arg, ArgMatches, Command, value_parser};

mod address;
mod cli;

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
        .get_matches()
}

fn main() {
    let matches = get_arguments();
    let url = matches.get_one::<Address>("url").unwrap();

    let _response = run(url.clone());
}
