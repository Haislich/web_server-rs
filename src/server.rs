use crate::http::request::{self, ParseError};
use request::Request;
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;
const BUF_SIZE: usize = 1024;
pub struct Server {
    address: String,
}
fn consume(buf: [u8; 1024]) {}
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
                    let request: Result<Request, ParseError> = match tcp_stream.read(&mut buf) {
                        Ok(n) => {
                            if n >= BUF_SIZE || n == 0 {
                                Err(ParseError::InvalidRequest)
                            } else {
                                Request::try_from(&buf[..])
                            }
                        }
                        Err(e) => Err(ParseError::InvalidRequest),
                    };
                    //buf = [0; BUF_SIZE];
                    println!("{:?}", request);
                }
                Err(e) => eprintln!("Failed to establish a connection: {}.", e),
            }
        }
    }
}
