pub mod constants;
pub mod handlers;
pub mod request;
pub mod response;
pub mod route;
pub mod thread_pool;

use constants::BUFFER_SIZE;

use request::Request;
use thread_pool::{PoolMaster, ThreadPool};

use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    sync::{atomic::AtomicBool, Arc},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    listener.set_nonblocking(true).unwrap();

    let main_pool = ThreadPool::new(4);

    let pool_master = PoolMaster::new(main_pool, listener);

    let running = Arc::new(AtomicBool::new(true));

    let running_clone = running.clone();
    ctrlc::set_handler(move || {
        running_clone.store(false, std::sync::atomic::Ordering::SeqCst);
    })
    .unwrap();

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        pool_master.execute(handle_listener);
    }

    println!("Shutting down main thread");

    drop(pool_master);

    println!("Dropped PoolMaster,  Byeeeee");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; BUFFER_SIZE];
    stream.read(&mut buffer).unwrap();

    let request = Request::new(&buffer, stream.try_clone().unwrap());

    let response = request.respond();

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_listener(pm: &PoolMaster) {
    for stream in pm.listener.incoming() {
        match stream {
            Ok(s) => {
                pm.pool
                    .execute(move || {
                        handle_connection(s);
                    })
                    .unwrap();
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                //Do something other
                thread::sleep(Duration::new(0, 10));
                break;
            }
            Err(e) => println!("{e}"),
        };
    }
}
