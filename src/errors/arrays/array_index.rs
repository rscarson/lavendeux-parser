use crate::Token;
use crate::errors::*;

use std::error::Error;
use std::fmt::{self, Display};

/// An error caused by attempting to use an out of bounds index on an array
#[derive(Debug, Clone)]
pub struct ArrayIndexError {
    cause: usize,
    src: ParserErrorSource
}
impl ArrayIndexError {
    /// Create a new instance of this error
    /// 
    /// # Arguments
    /// * `src` - Token causing the error
    /// * `cause` - Reason for the error
    pub fn new(src: &Token, cause: usize) -> Self {
        Self {
            cause: cause,
            src: ParserErrorSource::new(src)
        }
    }

    /// Return the cause of the error
    pub fn cause(&self) -> &usize {
        &self.cause
    }

    /// Describes the location and text of the bad token
    pub fn source(&self) -> &ParserErrorSource {
        &self.src
    }
}

impl Error for ArrayIndexError {}
impl Display for ArrayIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "array index {} out of bounds {}", self.cause, self.src)?;
        fmt::Result::Ok(())
    }
}

impl Into<ParserError> for ArrayIndexError {
    fn into(self) -> ParserError {
        ParserError::ArrayIndex(self)
    }
}