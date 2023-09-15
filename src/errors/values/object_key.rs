use crate::Token;
use crate::errors::*;

use std::error::Error;
use std::fmt::{self, Display};

/// An error caused by attempting to use an invalid object key
#[derive(Debug, Clone)]
pub struct ObjectKeyError {
    cause: String,
    src: ParserErrorSource
}
impl ObjectKeyError {
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
}

impl Error for ObjectKeyError {}
impl Display for ObjectKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "object key {} not found {}", self.cause, self.src)?;
        fmt::Result::Ok(())
    }
}

impl Into<ParserError> for ObjectKeyError {
    fn into(self) -> ParserError {
        ParserError::ObjectKey(self)
    }
}