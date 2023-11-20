use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    io::{Result as IOResult, Write},
    net::TcpStream,
};

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }
    pub fn send(&self, stream: &mut dyn Write) -> IOResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {} \r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}
impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            //_ => "Not implemented",
        }
    }
}
impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}
