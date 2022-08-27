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
    route: Route,
    path: String,
    rel_path: String
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

        let mut method = if method_string.starts_with(GET) {
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

        let mut path = String::from("");
        let mut rel_path = String::from("");

        // What route does the client want
        let route = match parts.get(1)  {
            Some(v) => {
                let value = String::from(*v);

                let keys: Vec<&str> = value.split("/").collect();

                let root = match keys.get(1) {
                    Some(v) =>format!("/{}", *v),
                    None => String::from("")
                };

                rel_path = format!("/{}", keys[2..].join("/"));

                match routes(&method).get(&root) {
                    Some(r) => {
                        // Only set the path if there is a valid route
                        path = value;
                        
                        r.copy()
                    },
                    None => routes(&RequestMethod::GET).get("/404").unwrap().copy()                                                
                }
            },
            None => routes(&RequestMethod::GET).get("/404").unwrap().copy()
        };

        // Check if HTTP/1.1 is present
        let http_valid = match parts.get(2) {
            Some(value) => value.starts_with(HTTP),
            None => false
        };

        if !http_valid {
            method = RequestMethod::INVALID
        } 

        Request {
            method,
            route,
            path,
            rel_path
        }
    }

    pub fn respond(&self) -> String {
        self.route.respond(&self)
    }

    pub fn method(&self) -> &RequestMethod {
        &self.method
    }

    pub fn path(&self) -> String {
        self.path.to_string()
    }

    pub fn rel_path(&self) -> String{
        self.rel_path.to_string()
    }
}