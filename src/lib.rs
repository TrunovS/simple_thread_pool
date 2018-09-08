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

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;
type AtomicMutexJobReciever = Arc<Mutex<mpsc::Receiver<Job>>>;

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: AtomicMutexJobReciever) -> Worker {

        let handle = thread::spawn(move || {
            loop {
                let recv: &mpsc::Receiver<Job>  = &*(receiver.lock().unwrap());
                let job = recv.recv().unwrap();
                job.call_box();

                println!("Worker {} has a job",id);
            }

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
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
