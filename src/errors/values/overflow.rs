use crate::Error;
use crate::Token;

use std::fmt::{self, Display};

/// An error caused by a calculation that resulted in an overflow
#[derive(Debug, Clone)]
pub struct OverflowError {
    src: ErrorSource,
}
impl OverflowError {
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

impl Display for OverflowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "arithmetic overflow {}", self.src)?;
        fmt::Result::Ok(())
    }
}
