use crate::Token;
use crate::errors::*;

use std::error::Error;
use std::fmt::{self, Display};

/// An error caused by attempting to use a function with ambiguous arguments
#[derive(Debug, Clone)]
pub struct AmbiguousFunctionError {
    function: String,
    cause: String,
    src: ParserErrorSource
}
impl AmbiguousFunctionError {
    /// Create a new instance of this error
    /// 
    /// # Arguments
    /// * `src` - Token causing the error
    /// * `function` - name of the ambiguous function
    /// * `cause` - Reason for the error
    pub fn new(src: &Token, function: &str, cause: &str) -> Self {
        Self {
            function: function.to_string(),
            cause: cause.to_string(),
            src: ParserErrorSource::new(src)
        }
    }

    /// Return the function causing the error
    pub fn function(&self) -> &str {
        &self.function
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

impl Error for AmbiguousFunctionError {}
impl Display for AmbiguousFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ambiguous arguments in {}(): {} {}", self.function, self.cause, self.src)?;
        fmt::Result::Ok(())
    }
}

impl Into<ParserError> for AmbiguousFunctionError {
    fn into(self) -> ParserError {
        ParserError::AmbiguousFunction(self)
    }
}