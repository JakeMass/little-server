use std::{fmt::format, fs};

use crate::constants::{NF_404, OK_200, OK_204, RES_FOLDER};

pub struct Response {
    headers: String,
    contents: String,
}

impl Response {
    pub fn new(headers: String, contents: String) -> Response {
        Response { headers, contents }
    }

    pub fn to_string(&self) -> String {
        format!("{}\r\n{}", self.headers, self.contents)
    }
}

pub fn json(contents: &str) -> Response {
    let contents = String::from(contents);

    let headers = format!(
        "{}Content-Length: {}\r\n{}",
        OK_200,
        contents.len(),
        "Content-Type: application/json"
    );

    Response { headers, contents }
}

pub fn view(path: &str) -> Response {
    match fs::read_to_string(path) {
        Ok(v) => {
            let headers = format!("{}Content-Length: {}\r\n", OK_200, v.len());

            Response::new(headers, v)
        }
        Err(e) => {
            let error = e.to_string();

            let headers = format!("{}Content-Length: {}\r\n", NF_404, error.len());

            Response::new(headers, error)
        }
    }
}

pub fn view_with_code(path: &str, code: &str) -> Response {
    match fs::read_to_string(path) {
        Ok(v) => {
            let headers = format!("{}Content-Length: {}\r\n", code, v.len());

            Response::new(headers, v)
        }
        Err(e) => {
            let error = e.to_string();

            let headers = format!("{}Content-Length: {}\r\n", NF_404, error.len());

            Response::new(headers, error)
        }
    }
}

pub fn resource(path: &str) -> Response {
    let path = format!("{}{}", RES_FOLDER, path);

    let parts: Vec<&str> = path.split(".").collect();

    let extension = match parts.get(parts.len() - 1) {
        Some(v) => *v,
        None => "",
    };

    let mime = guessMimeByExtension(extension);

    match fs::read_to_string(path) {
        Ok(v) => {
            let headers = format!(
                "{}Content-Length: {}\r\n{}\r\n",
                OK_200,
                v.len(),
                format!("Content-Type: {}", mime)
            );

            println!("{v}");

            Response::new(headers, v)
        }
        Err(e) => {
            let error = e.to_string();

            let headers = format!("{}Content-Length: {}\r\n", NF_404, error.len());

            Response::new(headers, error)
        }
    }
}

fn guessMimeByExtension(extension: &str) -> String {
    match extension {
        "js" => String::from("text/javascript"),
        _ => String::from(""),
    }
}
