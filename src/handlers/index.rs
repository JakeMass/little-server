use crate::{request::Request, response::view_with_code, constants::NF_404};
use super::response::{Response, view, resource};

pub fn get(request: &Request) -> Response {

    //

    view("hello.html")
}

pub fn not_found(request: &Request) -> Response {
    view_with_code("404.html", NF_404)
}

pub fn resources(request: &Request) -> Response {
    resource(&request.rel_path())
}