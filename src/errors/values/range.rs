use crate::{Value, Token};
use crate::errors::*;

use std::error::Error;
use std::fmt::{self, Display};

/// An error caused by attempting use an out of range value
#[derive(Debug, Clone)]
pub struct RangeError {
    cause: Value,
    src: ParserErrorSource
}
impl RangeError {
    /// Create a new instance of this error
    /// 
    /// # Arguments
    /// * `src` - Token causing the error
    /// * `cause` - Reason for the error
    pub fn new(src: &Token, cause: &Value) -> Self {
        Self {
            cause: cause.clone(),
            src: ParserErrorSource::new(src)
        }
    }

    /// Return the cause of the error
    pub fn cause(&self) -> &Value {
        &self.cause
    }

    /// Describes the location and text of the bad token
    pub fn source(&self) -> &ParserErrorSource {
        &self.src
    }
}

impl Error for RangeError {}
impl Display for RangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "value out of range: {} {}", self.cause, self.src)?;
        fmt::Result::Ok(())
    }
}

impl Into<ParserError> for RangeError {
    fn into(self) -> ParserError {
        ParserError::Range(self)
    }
}