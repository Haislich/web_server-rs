use super::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::Path;
use std::str::{self, Utf8Error};
// GET /software/htp/cics/index.html HTTP/1.1
#[derive(PartialEq, Debug)]
pub struct Request {
    method: Method,
    path: String,
    query: Option<String>,
}
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(value)?;
        todo!()
    }
}
impl TryFrom<&str> for Request {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let tmp = value.split_whitespace().collect::<Vec<&str>>();
        match tmp.len() {
            3 => {
                return Ok(Request {
                    method: Method::Get,
                    path: tmp[1].to_string(),
                    query: Some(tmp[2].to_string()),
                })
            }
            2 => {
                return Ok(Request {
                    method: Method::Get,
                    path: tmp[1].to_string(),
                    query: None,
                })
            }
            _ => return Err(ParseError::InvalidRequest),
        }
        //todo!()
        //for element in value.split_whitespace() {}
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_request_from_str() {
        let req1 = Request {
            method: Method::Get,
            path: String::from("Ciao"),
            query: Some(String::from("Query")),
        };
        let req2 = Request {
            method: Method::Get,
            path: String::from("Ciao"),
            query: None,
        };
        assert_eq!(req1, Request::try_from("Get Ciao Query").unwrap());
        assert_eq!(req2, Request::try_from("ciao Ciao").unwrap());
        // assert_eq!(ParseError::InvalidRequest,from)
    }
}
