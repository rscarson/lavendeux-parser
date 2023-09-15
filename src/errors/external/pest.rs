use crate::Token;
use crate::errors::*;

use std::error::Error;
use std::fmt::{self, Display};

/// An error caused by a problem in parsing the syntax of an expression
#[derive(Debug, Clone)]
pub struct PestError {
    src: ParserErrorSource
}
impl PestError {
    /// Create a new instance of this error
    /// 
    /// # Arguments
    /// * `src` - Token causing the error
    pub fn new(src: &Token) -> Self {
        Self {
            src: ParserErrorSource::new(src)
        }
    }

    /// Describes the location and text of the bad token
    pub fn source(&self) -> &ParserErrorSource {
        &self.src
    }
}

impl Error for PestError {}
impl Display for PestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid syntax {}", self.src)?;
        fmt::Result::Ok(())
    }
}

impl Into<ParserError> for PestError {
    fn into(self) -> ParserError {
        ParserError::Pest(self)
    }
}