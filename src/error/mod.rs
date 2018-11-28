use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CouchbaseError {
    pub reason: String
}

impl CouchbaseError {
    pub fn new(reason: String) -> CouchbaseError {
        CouchbaseError {
            reason: reason
        }
    }
}

impl fmt::Display for CouchbaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CouchbaseError: {}", self.reason)
    }
}

impl Error for CouchbaseError {}
