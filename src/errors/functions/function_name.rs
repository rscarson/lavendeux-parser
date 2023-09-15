use crate::Token;
use crate::errors::*;

use std::fmt::{self, Display};

/// An error caused by calling a function that does not exist
#[derive(Debug, Clone)]
pub struct FunctionNameError {
    cause: String,
    src: ParserErrorSource
}
impl FunctionNameError {
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

impl Display for FunctionNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unrecognized function {}() {}", self.cause, self.src)?;
        fmt::Result::Ok(())
    }
}