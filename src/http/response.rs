use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    io::{Result as IOResult, Write},
};
#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub const fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }
    pub fn send(&self, stream: &mut dyn Write) -> IOResult<()> {
        let body = self.body.as_ref().map_or("", |body| body);
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
    pub const fn reason_phrase(&self) -> &str {
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
