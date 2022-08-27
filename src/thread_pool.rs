use std::{
    sync::{mpsc, Arc, Mutex, atomic::AtomicBool},
    thread, io,
};
use std::net::{TcpListener, TcpStream};
use crate::handle_connection;

pub struct PoolMaster{
    pool: ThreadPool,
    listener: TcpListener,
}

impl PoolMaster{
    pub fn new(pool: ThreadPool, listener: TcpListener) -> PoolMaster {
        PoolMaster { pool, listener }
    }

    pub fn execute(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(s) => {
                    self.pool.execute(move || {
                        handle_connection(s);
                    }).unwrap();
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    break;
                },
                Err(e) => println!("{e}")
            };
    
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F) -> Result<(), mpsc::SendError<Box<dyn FnOnce() + Send>>>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job)
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        println!("DROP");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    job();
                }
                Err(e) => {
                    println!("Thread {id} paniced: {e}");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
