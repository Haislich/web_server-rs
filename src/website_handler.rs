use crate::{
    http::{Method, Request, Response, StatusCode},
    server::Handler,
};
use std::fs::{canonicalize, read_to_string};

pub struct WebsiteHandler {
    public_path: String,
}
impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
    fn read_file(&self, file_name: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_name);
        match canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    read_to_string(path).ok()
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}
impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        // Response::new(StatusCode::Ok, Some("<h1>TEST</h1>".to_string()))
        match request.method() {
            Method::Get => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, Some("Hello".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
