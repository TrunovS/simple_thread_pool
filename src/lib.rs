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
type AtomicMutexJobReciever = Arc<Mutex<mpsc::Receiver<Message>>>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: AtomicMutexJobReciever) -> Worker {

        let handle = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} has a job. Executing...",id);
                        job.call_box();
                        println!("Worker {}: job done.",id);
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.Stop executing.", id);
                        break;
                    }
                }
            }
            });

        Worker {
            id,
            handle: Some(handle),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
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
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self)
    {

        println!("Send Terminate message to all workers.Breaking loop");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("ShutingDown all workers.");

        for wrk in &mut self.workers {
            println!("Shutting down worker {}",wrk.id);
            if let Some(handle) = wrk.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}
