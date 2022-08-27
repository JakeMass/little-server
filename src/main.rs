pub mod constants;
pub mod handlers;
pub mod request;
pub mod response;
pub mod route;
pub mod thread_pool;

use constants::BUFFER_SIZE;

use request::Request;
use thread_pool::{ThreadPool, PoolMaster};

use std::{io::prelude::*, sync::atomic::AtomicBool};
use text_io::read;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.set_nonblocking(true).unwrap();
    
    let main_pool = ThreadPool::new(4);
    let input_pool = ThreadPool::new(1);
    
    let pool_master = PoolMaster::new(main_pool, listener);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    input_pool.execute(move || {
        while r.load(std::sync::atomic::Ordering::SeqCst){
            let line: String = read!();
    
            println!("$: {}", line);
    
            if line == "quit" {
                r.store(false, std::sync::atomic::Ordering::SeqCst);
            }
        }
    }).unwrap();

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        pool_master.execute();
    }

    println!("Shutting down main thread");

    drop(pool_master);

    println!("Dropped PoolMaster,  Byeeeee");
}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; BUFFER_SIZE];
    stream.read(&mut buffer).unwrap();

    let request = Request::new(&buffer);

    let response = request.respond();

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
