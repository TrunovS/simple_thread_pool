#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::thread;

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>,
};

impl Worker {
    fn new(id: usize) -> Worker {
        Worker {
            id,
            thread::spawn(|| { }),
        }
    }
};

pub struct ThreadPool {
    workers: Vec<Worker>,
};


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
        let mut workers = Vec::with_capacity(size);

        for wid in 0..size {
            workers.push(Worker::new(wid))
        }

        ThreadPool {
            workers
        }
    }

    pub fn start<F>(&self, f: F)
        where
        F: FnOnce() + Send + 'static
    {

    }
};
