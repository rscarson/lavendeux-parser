use crate::Error;
use crate::Token;

use std::fmt::{self, Display};

/// An error caused by a recursive function going too deep
#[derive(Debug, Clone)]
pub struct StackError {
    src: ErrorSource,
}
impl StackError {
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

impl Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "recursive function went too deep {}", self.src)?;
        fmt::Result::Ok(())
    }
}
