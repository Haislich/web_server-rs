use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;
const BUF_SIZE: usize = 1024;
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_error(&mut self, error: &ParseError) -> Response {
        eprintln!("Failed to parse request: {error}");
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    address: String,
}
impl Server {
    pub const fn new(address: String) -> Self {
        Self { address }
    }
    pub fn run(self, mut handler: impl Handler) -> ! {
        println!("Listening on {}", self.address);
        let listener = TcpListener::bind(self.address).unwrap();
        loop {
            let (mut tcp_stream, _socket_addr) = listener.accept().unwrap();
            eprintln!("Connection accepted.{tcp_stream:?}");
            let mut buf = [0; BUF_SIZE];
            if let Err(e) = tcp_stream.read(&mut buf) {
                eprintln!("Error in read {e}");
                continue;
            }
            println!("{}", String::from_utf8_lossy(&buf[..]));
            let response = match Request::try_from(&buf[..]) {
                Ok(request) => handler.handle_request(&request),
                Err(e) => {
                    eprintln!("Found error {e:?}");
                    handler.handle_error(&e)
                }
            };
            if let Err(e) = response.send(&mut tcp_stream) {
                eprintln!("Failed to send response {e}");
            }
        }
    }
}
