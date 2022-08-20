pub mod constants;
pub mod handlers;
pub mod response;
pub mod request;
pub mod route;
pub mod thread_pool;

use constants::BUFFER_SIZE;

use request::Request;
use thread_pool::ThreadPool;

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

    let request = Request::new(&buffer);

    let response = request.respond_as_bytes();
    
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
