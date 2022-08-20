use std::collections::HashMap;
use crate::route::{Route, RouteClb};
use crate::request::RequestMethod;
use crate::handlers::index;

pub fn routes() -> HashMap<String, Route> {
    HashMap::from([
        get("/", index::get),
        get("/404", index::not_found),
        get("/res", index::resources)
    ])
}

pub fn get(path: &str, clb: RouteClb) -> (String, Route) {
    (
        String::from(path),
        Route::new(RequestMethod::GET, path, clb)
    )
}
     