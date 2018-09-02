#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

type Job = Box<FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {

        let handle = thread::spawn(|| {
            receiver;
            });

        Worker {
            id,
            handle,
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}


impl ThreadPool {
    /// Создаем экземпляры ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool
    {
        assert!(size>0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for wid in 0..size {
            workers.push(Worker::new(wid, Arc::clone(&receiver)))
        }

        ThreadPool {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
        where
        F: FnOnce() + Send + 'static
    {

    }
}
