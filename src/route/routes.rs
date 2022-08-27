use std::collections::HashMap;
use crate::route::{Route, RouteClb};
use crate::request::{RequestMethod, Request};
use crate::handlers::index;

pub fn routes(method: &RequestMethod) -> HashMap<String, Route> {
    match method {
        RequestMethod::GET => HashMap::from([
            get("/", index::get),
            get("/test", index::prefix),
            get("/404", index::not_found),
            get("/res", index::resources)
        ]),
        RequestMethod::POST => HashMap::from([
            post("/", index::post)
        ]),
        _ => HashMap::from([])
    }
}

pub fn get(path: &str, clb: RouteClb) -> (String, Route) {
    (
        String::from(path),
        Route::new(RequestMethod::GET, path, clb)
    )
}

pub fn get_prefix(path: &str, clb: RouteClb) -> (String, Route) {
    (
        String::from(path),
        Route::new(RequestMethod::GET, path, clb)
    )
}

pub fn post(path: &str, clb: RouteClb) -> (String, Route) {
    (
        String::from(path),
        Route::new(RequestMethod::POST, path, clb)
    )
}
     