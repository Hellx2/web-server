/*use std::error::Error;
use std::fmt::{ Display, Formatter };
use std::rc::Rc;

#[derive(Debug)]
pub struct PoolCreationError {
    msg: String,
}

impl Display for PoolCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, self.msg)
    }
}

impl Error for PoolCreationError {
    
}

pub struct ThreadPool {
    size: usize
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool { size }
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            Ok(ThreadPool { size })
        } else {
            Err(PoolCreationError)
        }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static, {
        
    }
}*/
