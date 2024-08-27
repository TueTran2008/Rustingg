use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle()>,
}
enum ThreadPoolError {
    PoolErrorCreation,
}
impl ThreadPool {
    fn build(size: usize) -> Result<ThreadPool, PoolErrorCreation> {}
    pub fn new(size: usize) -> Self {
        Self
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),
        F: Send + 'static,
    {
    }
}
