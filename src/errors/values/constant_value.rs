use crate::Error;
use crate::Token;

use std::fmt::{self, Display};

/// An error caused by attempting to overwrite a constant
#[derive(Debug, Clone)]
pub struct ConstantValueError {
    cause: String,
    src: ErrorSource,
}
impl ConstantValueError {
    /// Create a new instance of this error
    ///
    /// # Arguments
    /// * `src` - Token causing the error
    /// * `cause` - Reason for the error
    pub fn new(src: &Token, cause: &str) -> Self {
        Self {
            cause: cause.to_string(),
            src: ErrorSource::new(src),
        }
    }

    /// Return the cause of the error
    pub fn cause(&self) -> &str {
        &self.cause
    }

    /// Describes the location and text of the bad token
    pub fn source(&self) -> &ErrorSource {
        &self.src
    }
}

impl Display for ConstantValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "could not overwrite constant value {} {}",
            self.cause, self.src
        )?;
        fmt::Result::Ok(())
    }
}
