use std::{fmt, error};

#[derive(Debug)]
pub struct MyError {
    pub message: String,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for MyError {}