use super::{Method, Query};
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

#[derive(Debug)]
#[allow(dead_code)]
pub struct Request<'buf> {
    // This means that the request struct is generic over a lifetime
    method: Method,
    path: &'buf str, //String, -> Changin from String to &str assures that no useless heap allocation happen.
    query: Option<Query<'buf>>, //Option<&str>, -> Changin from String to &str assures that no useless heap allocation happen.
}
impl<'buf> Request<'buf> {
    pub const fn path(&self) -> &str {
        self.path
    }
    pub const fn method(&self) -> &Method {
        &self.method
    }
    // pub const fn query(&self) -> Option<&Query> {
    //     self.query.as_ref()
    // }
}
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    // The lifetime here assures us that we cannot deallocate the buffer before deallocating the request.
    type Error = ParseError;
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::Request)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::Request)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::Request)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::Protocol);
        }

        let method: Method = method.parse()?;

        let mut query = None;
        if let Some(i) = path.find('?') {
            query = Some(Query::from(&path[i + 1..]));
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
    Request,
    Encoding,
    Protocol,
    Method,
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Request => write!(f, "Invalid Request"),
            Self::Encoding => write!(f, "Invalid Encoding"),
            Self::Protocol => write!(f, "Invalid Protocol"),
            Self::Method => write!(f, "Invalid Method"),
        }
    }
}
impl Error for ParseError {}
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::Encoding
    }
}
