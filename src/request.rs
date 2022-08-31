use crate::constants::{BUFFER_SIZE, DELETE, GET, HTTP, PATCH, POST};
use crate::route::{
    routes, 
    hosts,
    Route
};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub enum RequestMethod {
    GET,
    POST,
    PATCH,
    DELETE,
    INVALID,
}

pub struct Request {
    method: RequestMethod,
    route: Route,
    path: String,
    rel_path: String,
    headers: HashMap<String, String>
}

impl Request {
    pub fn new(request: &[u8; BUFFER_SIZE]) -> Request {
        let req = String::from_utf8(request.to_vec()).unwrap();

        let req: Vec<&str> = req.split("\r\n").collect();

        // Get header informations
        let parts = match req.get(0) {
            Some(v) => v.split(" ").collect(),
            None => vec![],
        };

        // GET, POST, PATCH or DELETE
        let method_string = match parts.get(0) {
            Some(v) => v,
            None => "",
        };

        // Get Header
        let mut headers = HashMap::new();

        for i in 1..req.len() {
            let key_value: Vec<&str> = req[i].split(":").collect();
            
            if key_value[0] == "" {
                break;
            }

            let key = match key_value.get(0) {
                Some(v) => String::from(*v),
                None => "".to_string()
            };

            let split_key = String::from(&key) + &":".to_string();

            let value: Vec<&str> = req[i].split(&split_key).collect();

            let mut value = value.join("");

            if value.starts_with(" ") {
                value.remove(0);
            }

            headers.insert(key, value);
        }

        let mut method = find_method_from_string(method_string.to_string());

        let (route, path, rel_path) = parse_route(parts.get(1), method, &headers);

        // Check if HTTP/1.1 is present
        let http_valid = match parts.get(2) {
            Some(value) => value.starts_with(HTTP),
            None => false,
        };

        if !http_valid {
            method = RequestMethod::INVALID
        }

        Request {
            method,
            route,
            path,
            rel_path,
            headers,
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

    pub fn rel_path(&self) -> String {
        self.rel_path.to_string()
    }
}

pub fn find_method_from_string(method_string: String) -> RequestMethod {
    if method_string.starts_with(GET) {
        RequestMethod::GET
    } else if method_string.starts_with(POST) {
        RequestMethod::POST
    } else if method_string.starts_with(PATCH) {
        RequestMethod::PATCH
    } else if method_string.starts_with(DELETE) {
        RequestMethod::DELETE
    } else {
        RequestMethod::INVALID
    }
}

pub fn parse_route(route_string: Option<&&str>, method: RequestMethod, headers: &HashMap<String, String>) -> (Route, String, String) {
    match route_string {
        Some(v) => {
            let value = String::from(*v);

            let keys: Vec<&str> = value.split("/").collect();

            let root = match keys.get(1) {
                Some(v) => format!("/{}", *v),
                None => String::from(""),
            };

            let rel_path = format!("/{}", keys[2..].join("/"));
            let mut path = String::new();

            let host = match headers.get("Host") {
                Some(v) => String::from(v),
                None => "".to_string()
            };

            let routes = hosts::get_routes_by_host(&host);

            match routes(&method).get(&root) {
                Some(r) => {
                    // Only set the path if there is a valid route
                    path = value;

                    (r.copy(), path, rel_path)
                }
                None => (routes::routes(&RequestMethod::GET).get("/404").unwrap().copy(), path, rel_path),
            }
        }
        None => (routes::routes(&RequestMethod::GET).get("/404").unwrap().copy(), "".to_string(), "".to_string()),
    }
}
