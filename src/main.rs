mod thread_pool;
mod constants;
mod request_parser;

use constants::BUFFER_SIZE;

use request_parser::Request;
use thread_pool::ThreadPool;

use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    loop {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
    
            pool.execute(|| {
                handle_connection(stream);
            });

            println!("running loop");
        }
    }

   // println!("Closing server.\nBye.")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; BUFFER_SIZE];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let js = b"GET /test.js HTTP/1.1\r\n";

    let request = Request::new(&buffer);

    println!("{:?}", request);

    let (status_line, filename) = 
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "hello.html")
        } else if buffer.starts_with(js) {
            ("HTTP/1.1 200 OK\r\nX-Content-Type-Options: nosniff\r\nContent-Type: text/javascript", "test.js")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
