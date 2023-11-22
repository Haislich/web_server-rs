use super::ParseError;
use std::str::FromStr;
#[derive(PartialEq, Eq, Debug)]
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
        //super::Method::{Connect, Delete, Get, Head, Options, Patch, Post, Put, Trace};
        match s {
            "GET" => Ok(Self::Get),
            "HEAD" => Ok(Self::Head),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            "CONNECT" => Ok(Self::Connect),
            "OPTIONS" => Ok(Self::Options),
            "TRACE" => Ok(Self::Trace),
            "PATCH" => Ok(Self::Patch),
            _ => Err(ParseError::Method),
        }
    }
}
