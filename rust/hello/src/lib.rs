use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}
pub struct PoolCreationError;

impl ThreadPool {
	/// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {

        }

        ThreadPool { threads }
    }
	
	/// Builds a new ThreadPool
	///
	/// The size is the number of threads in the pool.
	///
	/// # Errors
	/// The `build` function will return a `PoolCreationError` if the size is zero.
	pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
		if size > 0 {
			Ok(ThreadPool::new(size))
		} else {
			Err(PoolCreationError)
		}

	}

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {

    }
}
