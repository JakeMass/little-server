pub mod routes;
pub mod hosts;

use crate::request::{Request, RequestMethod};
use crate::response::Response;

pub type RouteClb = fn(&Request) -> Response;

pub struct Route {
    path: String,
    method: RequestMethod,
    clb: RouteClb,
}

impl Route {
    pub fn new(method: RequestMethod, path: &str, clb: RouteClb) -> Route {
        Route {
            path: String::from(path),
            method,
            clb,
        }
    }

    pub fn copy(&self) -> Route {
        Route {
            path: self.path.to_string(),
            method: self.method,
            clb: self.clb,
        }
    }

    pub fn respond(&self, request: &Request) -> String {
        let response = (self.clb)(request);

        response.to_string()
    }

    pub fn path(&self) -> String {
        self.path.to_string()
    }

    pub fn clb(&self) -> RouteClb {
        self.clb
    }
}
