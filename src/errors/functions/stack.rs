use crate::Token;
use crate::errors::*;

use std::error::Error;
use std::fmt::{self, Display};

/// An error caused by a recursive function going too deep
#[derive(Debug, Clone)]
pub struct StackError {
    src: ParserErrorSource
}
impl StackError {
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

impl Error for StackError {}
impl Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "recursive function went too deep {}", self.src)?;
        fmt::Result::Ok(())
    }
}

impl Into<ParserError> for StackError {
    fn into(self) -> ParserError {
        ParserError::Stack(self)
    }
}