use super::ParseError;
use std::str::FromStr;
#[derive(PartialEq, Debug)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}
impl FromStr for Method {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use super::Method::*;
        match s {
            "GET" => Ok(Get),
            "HEAD" => Ok(Head),
            "POST" => Ok(Post),
            "PUT" => Ok(Put),
            "DELETE" => Ok(Delete),
            "CONNECT" => Ok(Connect),
            "OPTIONS" => Ok(Options),
            "TRACE" => Ok(Trace),
            "PATCH" => Ok(Patch),
            _ => Err(ParseError::InvalidMethod),
        }
    }
}
