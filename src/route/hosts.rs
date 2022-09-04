use crate::request::RequestMethod;
use crate::route::routes;
use crate::route::Route;
use std::collections::HashMap;

pub fn get_routes_by_host(host: &String) -> fn(&RequestMethod) -> HashMap<String, Route> {
    match host_routes().get(host) {
        Some(v) => *v,
        None => dummy_routes,
    }
}

fn host_routes() -> HashMap<String, fn(&RequestMethod) -> HashMap<String, Route>> {
    HashMap::from([host("test.ls:7878", routes::routes)])
}

fn host(
    name: &str,
    routes: fn(&RequestMethod) -> HashMap<String, Route>,
) -> (String, fn(&RequestMethod) -> HashMap<String, Route>) {
    (String::from(name), routes)
}

fn dummy_routes(_method: &RequestMethod) -> HashMap<String, Route> {
    HashMap::new()
}
