use crate::address::Address;
use std::net::TcpStream;

pub fn run(addr: Address) -> Result<(), ()> {
    println!("[DEBUG] Informed address: {addr:?}");
    let tcp_socket = TcpStream::connect(&addr);

    match tcp_socket {
        Ok(stream) => {
            println!("Connected! {}", stream.peer_addr().unwrap())
        }
        Err(e) => {
            println!("Error {}", e)
        }
    }

    return Ok(());
}
