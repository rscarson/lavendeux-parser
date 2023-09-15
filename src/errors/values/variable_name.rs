use crate::Token;
use crate::errors::*;

use std::error::Error;
use std::fmt::{self, Display};

/// An error caused by attempting to use an unassigned variable
#[derive(Debug, Clone)]
pub struct VariableNameError {
    cause: String,
    src: ParserErrorSource
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
            src: ParserErrorSource::new(src)
        }
    }

    /// Return the cause of the error
    pub fn cause(&self) -> &str {
        &self.cause
    }

    /// Describes the location and text of the bad token
    pub fn source(&self) -> &ParserErrorSource {
        &self.src
    }
}

impl Error for VariableNameError {}
impl Display for VariableNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unrecognized variable {} {}", self.cause, self.src)?;
        fmt::Result::Ok(())
    }
}

impl Into<ParserError> for VariableNameError {
    fn into(self) -> ParserError {
        ParserError::VariableName(self)
    }
}