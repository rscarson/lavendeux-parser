use crate::Token;
use crate::errors::*;

use std::fmt::{self, Display};

/// An error caused by filesystem issues
#[derive(Debug, Clone)]
pub struct IOError {
    cause: String,
    src: ParserErrorSource
}
impl IOError {
    /// Create a new instance of this error
    /// 
    /// # Arguments
    /// * `src` - Token causing the error
    /// * `cause` - Reason for the error
    pub fn new(src: &Token, cause: &str) -> Self {
        Self {
            cause: cause.to_string(),
            src: ParserErrorSource::new(src)
        }
    }

    /// Return the cause of the error
    pub fn cause(&self) -> &str {
        &self.cause
    }

    /// Describes the location and text of the bad token
    pub fn source(&self) -> &ParserErrorSource {
        &self.src
    }

    /// Create a new instance of this error from an existing error
    /// 
    /// # Arguments
    /// * `src` - Token causing the error
    /// * `error`- source error
    pub fn from_ioerror(src: &Token, error: std::io::Error) -> Self {
        Self::new(src, &error.to_string())
    }
}

impl Display for IOError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IO error: {} {}", self.cause, self.src)?;
        fmt::Result::Ok(())
    }
}