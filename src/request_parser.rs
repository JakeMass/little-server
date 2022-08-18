use crate::constants::{BUFFER_SIZE, GET, POST, PATCH, DELETE, HTTP};

#[derive(Debug)]
pub struct Request {
    method: RequestMethod,
    path: String
}

impl Request {
    pub fn new(request: &[u8; BUFFER_SIZE]) -> Request {
        let req = String::from_utf8(request.to_vec()).unwrap();

        let req: Vec<&str> = req.split("\r\n").collect();

        let mut method_string: &str = "";
        let mut path_string: &str = "";
        let mut http_valid: bool = false;

        match req.get(0) {
            Some(v) => {
                let parts: Vec<&str> = v.split(" ").collect();

                method_string = match parts.get(0) {
                    Some(value) => value,
                    None => ""
                };

                path_string = match parts.get(1) {
                    Some(value) => value,
                    None => ""
                };

                http_valid = match parts.get(2) {
                    Some(value) => value.starts_with(HTTP),
                    None => false
                };
            },
            None => ()
        };

        let method: Option<RequestMethod> = 
            if method_string.starts_with(GET) {
                Some(RequestMethod::GET)
            } else if method_string.starts_with(POST) {
                Some(RequestMethod::POST)
            } else if method_string.starts_with(PATCH) {
                Some(RequestMethod::PATCH)
            } else if method_string.starts_with(DELETE) {
                Some(RequestMethod::DELETE)
            } else {
                None
            };

        Request {
            method: match method {
                Some(m) => match http_valid {
                    true => m,
                    false => RequestMethod::INVALID
                },
                None => RequestMethod::INVALID
            },
            path: path_string.to_string()
        }
    }
}

#[derive(Debug)]
enum RequestMethod {
    GET,
    POST,
    PATCH,
    DELETE,
    INVALID
}