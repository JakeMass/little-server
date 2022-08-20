use std::fs;

use crate::constants::{
    OK_200,
    NF_404
};

pub struct Response {
    headers: String,
    contents: String
}

impl Response {
    pub fn new(headers: String, contents: String) -> Response {
        Response { headers, contents }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}\r\n{}",
            self.headers,
            self.contents
        )
    }
}

pub fn view(path: &str) -> Response {
    match fs::read_to_string(path) {
        Ok(v) => {
            let headers = format!(
                "{}Content-Length: {}\r\n",
                OK_200,
                v.len()
            );

            Response::new(headers, v)
        },
        Err(e) => {
            let error = e.to_string();

            let headers = format!(
                "{}Content-Length: {}\r\n",
                NF_404,
                error.len()
            );

            Response::new(headers, error)
        }
    }
}

pub fn view_with_code(path: &str, code: &str) -> Response {
    match fs::read_to_string(path) {
        Ok(v) => {
            let headers = format!(
                "{}Content-Length: {}\r\n",
                code,
                v.len()
            );

            Response::new(headers, v)
        },
        Err(e) => {
            let error = e.to_string();

            let headers = format!(
                "{}Content-Length: {}\r\n",
                NF_404,
                error.len()
            );

            Response::new(headers, error)
        }
    }
}