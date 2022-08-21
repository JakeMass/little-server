use crate::{request::Request, response::view_with_code, constants::{NF_404, OK_204}};
use super::response::{Response, view, resource, json};

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