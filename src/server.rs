use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;
const BUF_SIZE: usize = 1024;
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_error(&mut self, error: &ParseError) -> Response {
        println!("Failed to parse request: {}", error);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    address: String,
}
impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }
    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.address);
        let listener = TcpListener::bind(self.address).unwrap();

        loop {
            let (mut tcp_stream, _socket_addr) = listener.accept().unwrap();
            println!("Connection accepted.{:?}", tcp_stream);
            let mut buf = [0; BUF_SIZE];
            tcp_stream.read(&mut buf).unwrap();
            let response = match Request::try_from(&buf[..]) {
                Ok(request) => handler.handle_request(&request),
                Err(e) => {
                    eprintln!("Found error {:?}", e);
                    handler.handle_error(&e)
                }
            };
            if let Err(e) = response.send(&mut tcp_stream) {
                println!("Failed to send response {}", e);
            }
        }
    }
}
