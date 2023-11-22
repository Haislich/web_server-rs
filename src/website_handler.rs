use crate::{
    http::{Method, Request, Response, StatusCode},
    server::Handler,
};
use std::env;
use std::fs::{canonicalize, read_to_string};
pub struct WebsiteHandler {
    public_path: String,
}
impl WebsiteHandler {
    pub const fn new(public_path: String) -> Self {
        Self { public_path }
    }
    fn read_file(&self, file_name: &str) -> Option<String> {
        let path = if env::consts::OS == "windows" {
            format!("{}\\{}", self.public_path, file_name)
        } else {
            format!("{}/{}", self.public_path, file_name)
        };
        match (canonicalize(path), canonicalize(&self.public_path)) {
            (Ok(path), Ok(public_path)) => {
                if path.starts_with(public_path) {
                    read_to_string(path).ok()
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::Get => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, Some("Hello".to_string())),
                file_name => Response::new(StatusCode::Ok, self.read_file(file_name)),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
