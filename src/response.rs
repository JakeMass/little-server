use std::fs::File;
use std::io::{self, Read, Write};
use std::{fs, net::TcpStream};

use crate::constants::{NF_404, OK_200, RES_FOLDER};

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

    pub fn header(&self) -> String {
        format!("{}\r\n", self.headers)
    }
}

pub fn json(contents: &str) -> Response {
    let contents = String::from(contents);

    let headers = format!(
        "{}Content-Length: {}\r\n{}\r\n",
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

            Response::new(headers, v)
        }
        Err(e) => {
            let error = e.to_string();

            let headers = format!("{}Content-Length: {}\r\n", NF_404, error.len());

            Response::new(headers, error)
        }
    }
}

pub fn file(path: String, stream: &TcpStream) -> Response {
    let mut stream = stream.try_clone().unwrap();

    let parts: Vec<&str> = path.split(".").collect();

    let extension = match parts.get(parts.len() - 1) {
        Some(v) => *v,
        None => "",
    };

    let mime = guessMimeByExtension(extension);

    let mut file = match File::open(path) {
        Ok(v) => v,
        Err(_e) => panic!("FILE ERROR"),
    };

    let headers = format!(
        "{}{}\r\n{}\r\n",
        OK_200,
        format!("Content-Length: {}", file.metadata().unwrap().len()),
        format!("Content-Type: {}", mime)
    );

    let response = Response::new(headers, "".to_string());

    let mut buf = [0; 4096];

    let header = response.header();

    stream.write_all(header.as_bytes()).unwrap();

    loop {
        let n = match file.read(&mut buf) {
            Ok(v) => v,
            Err(_) => 0,
        };

        if n == 0 {
            break;
        }

        stream.write_all(&buf[..n]).unwrap();
    }

    response
}

fn guessMimeByExtension(extension: &str) -> String {
    match extension {
        "jpg" => String::from("image/jpeg"),
        "js" => String::from("text/javascript"),
        "pdf" => String::from("application/pdf"),
        _ => String::from(""),
    }
}
