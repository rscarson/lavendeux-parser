use crate::Error;
use crate::Token;

use std::fmt::{self, Display};

/// An error caused by attempting to use an unassigned variable
#[derive(Debug, Clone)]
pub struct VariableNameError {
    cause: String,
    src: ErrorSource,
}
impl VariableNameError {
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

impl Display for VariableNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unrecognized variable {} {}", self.cause, self.src)?;
        fmt::Result::Ok(())
    }
}
