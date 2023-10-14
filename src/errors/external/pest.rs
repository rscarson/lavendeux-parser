use crate::Error;
use crate::Token;

use std::fmt::{self, Display};

/// An error caused by a problem in parsing the syntax of an expression
#[derive(Debug, Clone)]
pub struct PestError {
    src: ErrorSource,
}
impl PestError {
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

impl Display for PestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid syntax {}", self.src)?;
        fmt::Result::Ok(())
    }
}
