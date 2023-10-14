use crate::Error;
use crate::Token;

use std::fmt::{self, Display};

/// An error caused by using a postfix operator without an operand
#[derive(Debug, Clone)]
pub struct UnterminatedArrayError {
    src: ErrorSource,
}
impl UnterminatedArrayError {
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

impl Display for UnterminatedArrayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "missing ']' {}", self.src)?;
        fmt::Result::Ok(())
    }
}
