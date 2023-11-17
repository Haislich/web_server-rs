use crate::http::request;
use request::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;
const BUF_SIZE: usize = 1024;
pub struct Server {
    address: String,
}
impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }
    pub fn run(self) {
        println!("Listening on {}", self.address);
        let listener = TcpListener::bind(self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut tcp_stream, _socket_addr)) => {
                    println!("Connection accepted.{:?}", tcp_stream);
                    let mut buf = [0; BUF_SIZE];
                    match tcp_stream.read(&mut buf) {
                        Ok(n) => {
                            if n >= BUF_SIZE || n <= 0 {
                                println!("Invalid incoming message")
                            } else {
                                println!("Recieved : {}", String::from_utf8_lossy(&buf));
                                match Request::try_from(&buf[..]) {
                                    Ok(request) => {
                                        println!("ciao")
                                    }
                                    Err(e) => println!("Error while parsing the request : {}", e),
                                }
                            }
                        }
                        Err(e) => println!("Failed reading from socket: {e}"),
                    };
                }
                Err(e) => eprintln!("Failed to establish a connection: {}.", e),
            }
        }
    }
}
