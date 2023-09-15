use crate::Token;
use crate::errors::*;

use std::fmt::{self, Display};

/// An error caused by attempting to use an empty array
#[derive(Debug, Clone)]
pub struct ArrayEmptyError {
    src: ParserErrorSource
}
impl ArrayEmptyError {
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

impl Display for ArrayEmptyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "array is empty {}", self.src)?;
        fmt::Result::Ok(())
    }
}