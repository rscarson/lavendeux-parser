use crate::Error;
use crate::Token;

use std::fmt::{self, Display};

/// An error caused by attempting to use arrays of different lengths
#[derive(Debug, Clone)]
pub struct ArrayLengthError {
    src: ErrorSource,
}
impl ArrayLengthError {
    /// Create a new instance of this error
    ///
    /// # Arguments
    /// * `src` - Token causing the error
    pub fn new(src: &Token) -> Self {
        Self {
            src: ErrorSource::new(src),
        }
    }

    /// Describes the location and text of the bad token
    pub fn source(&self) -> &ErrorSource {
        &self.src
    }
}

impl Display for ArrayLengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "array lengths incompatible {}", self.src)?;
        fmt::Result::Ok(())
    }
}
