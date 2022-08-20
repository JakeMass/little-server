use crate::constants::{BUFFER_SIZE, GET, POST, PATCH, DELETE, HTTP};
use crate::route::{Route, routes::routes};

#[derive(Copy, Clone, Debug)]
pub enum RequestMethod {
    GET,
    POST,
    PATCH,
    DELETE,
    INVALID
}



pub struct Request {
    method: RequestMethod,
    route: Route
}

impl Request {
    pub fn new(request: &[u8; BUFFER_SIZE]) -> Request {
        let req = String::from_utf8(request.to_vec()).unwrap();

        let req: Vec<&str> = req.split("\r\n").collect();

        // Get header informations
        let parts = match req.get(0) {
            Some(v) => v.split(" ").collect(),
            None => vec![]
        };

        // GET, POST, PATCH or DELETE
        let method_string = match parts.get(0) {
            Some(v) => v,
            None => ""
        };

        // What route does the client want
        let route = match parts.get(1)  {
            Some(v) => match routes().get(*v) {
                Some(r) => r.copy(),
                None => routes().get("/404").unwrap().copy()
            },
            None => routes().get("/404").unwrap().copy()
        };

        // Check if HTTP/1.1 is present
        let http_valid = match parts.get(2) {
            Some(value) => value.starts_with(HTTP),
            None => false
        };

        let method = 
            if !http_valid {
                RequestMethod::INVALID
            } else if method_string.starts_with(GET) {
                RequestMethod::GET
            } else if method_string.starts_with(POST) {
                RequestMethod::POST
            } else if method_string.starts_with(PATCH) {
                RequestMethod::PATCH
            } else if method_string.starts_with(DELETE) {
                RequestMethod::DELETE
            } else {
                RequestMethod::INVALID
            };

        Request {
            method,
            route
        }
    }

    pub fn respond(&self) -> String {
        self.route.respond(&self)
    }
}