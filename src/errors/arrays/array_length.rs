use crate::Token;
use crate::errors::*;

use std::fmt::{self, Display};

/// An error caused by attempting to use arrays of different lengths
#[derive(Debug, Clone)]
pub struct ArrayLengthError {
    src: ParserErrorSource
}
impl ArrayLengthError {
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

impl Display for ArrayLengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "array lengths incompatible {}", self.src)?;
        fmt::Result::Ok(())
    }
}