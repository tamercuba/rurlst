use std::net::TcpStream;

use clap::{Arg, ArgMatches, Command};

pub fn get_arguments() -> ArgMatches {
    Command::new("rURLst - A Blazingly slow and unoptimized cURL implemented in rust")
        .about("It helps you to make HTTP requests")
        .version("0.0.1")
        .author("Tâmer Pinheiro Cuba")
        .get_matches()
}

pub fn run(addr: String) -> Result<(), ()> {
    println!("[ADDRS]: {}", addr);
    let tcp_socket = TcpStream::connect(addr);

    match tcp_socket {
        Ok(stream) => {
            println!("Connected! {}", stream.local_addr().unwrap())
        }
        Err(e) => {
            println!("Error {}", e)
        }
    }

    return Ok(());
}
