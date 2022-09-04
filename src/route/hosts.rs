use crate::request::RequestMethod;
use crate::route::routes;
use crate::route::Route;
use std::collections::HashMap;

pub fn get_routes_by_host(host: &String) -> fn(&RequestMethod) -> HashMap<String, Route> {
    if host == "test.ls:7878" {
        routes::routes
    } else {
        dummy_routes
    }
}

fn dummy_routes(_method: &RequestMethod) -> HashMap<String, Route> {
    HashMap::new()
}
