use crate::Error;
use crate::Token;

use std::fmt::{self, Display};

/// An error caused by attempting to use an out of bounds index on an array
#[derive(Debug, Clone)]
pub struct ArrayIndexError {
    cause: usize,
    src: ErrorSource,
}
impl ArrayIndexError {
    /// Create a new instance of this error
    ///
    /// # Arguments
    /// * `src` - Token causing the error
    /// * `cause` - Reason for the error
    pub fn new(src: &Token, cause: usize) -> Self {
        Self {
            cause,
            src: ErrorSource::new(src),
        }
    }

    /// Return the cause of the error
    pub fn cause(&self) -> &usize {
        &self.cause
    }

    /// Describes the location and text of the bad token
    pub fn source(&self) -> &ErrorSource {
        &self.src
    }
}

impl Display for ArrayIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "array index {} out of bounds {}", self.cause, self.src)?;
        fmt::Result::Ok(())
    }
}
