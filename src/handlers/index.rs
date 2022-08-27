use std::collections::HashMap;

use super::response::{json, resource, view, Response};
use crate::{
    constants::{NF_404, OK_204},
    request::{Request, RequestMethod},
    response::view_with_code,
    route::{routes, Route},
};

pub fn routes(method: &RequestMethod) -> HashMap<String, Route> {
    match method {
        RequestMethod::GET => HashMap::from([routes::get("/test", test)]),
        _ => HashMap::from([]),
    }
}

pub fn prefix(request: &Request) -> Response {
    let rel_path = request.rel_path();
    let path: Vec<&str> = rel_path.split("/").collect();

    let key = match path.get(1) {
        Some(v) => format!("/{}", *v),
        None => String::from(""),
    };

    match routes(request.method()).get(&key) {
        Some(r) => r.clb()(request),
        None => not_found(request),
    }
}

fn test(request: &Request) -> Response {
    view("test.html")
}

pub fn get(request: &Request) -> Response {
    //

    view("hello.html")
}

pub fn post(request: &Request) -> Response {
    json("{ \"test\": \"Das ist ein Test\" }")
}

pub fn not_found(request: &Request) -> Response {
    view_with_code("404.html", NF_404)
}

pub fn resources(request: &Request) -> Response {
    resource(&request.rel_path())
}
