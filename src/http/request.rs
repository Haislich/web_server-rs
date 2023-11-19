use super::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::{self, Utf8Error};

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

#[derive(PartialEq, Debug)]
pub struct Request<'buf> {
    // This means that the request struct is generic over a lifetime
    method: Method,
    path: &'buf str, //String, -> Changin from String to &str assures that no useless heap allocation happen.
    query: Option<&'buf str>, //Option<&str>, -> Changin from String to &str assures that no useless heap allocation happen.
}
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    // The lifetime here assures us that we cannot deallocate the buffer before deallocating the request.
    type Error = ParseError;
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        println!("{method}");
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query = None;
        if let Some(i) = path.find('?') {
            query = Some(&path[i + 1..]);
            path = &path[..i];
        };

        Ok(Self {
            method,
            path,
            query,
        })
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use ParseError::*;
        match self {
            InvalidRequest => write!(f, "Invalid Request"),
            InvalidEncoding => write!(f, "Invalid Encoding"),
            InvalidProtocol => write!(f, "Invalid Protocol"),
            InvalidMethod => write!(f, "Invalid Method"),
        }
    }
}
impl Error for ParseError {}
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
