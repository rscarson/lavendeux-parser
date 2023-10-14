use crate::Error;
use crate::Token;

use std::fmt::{self, Display};

/// An error caused by attempting to parse an invalid string into a given format
#[derive(Debug, Clone)]
pub struct ParsingError {
    format: String,
    cause: String,
    src: ErrorSource,
}
impl ParsingError {
    /// Create a new instance of this error
    ///
    /// # Arguments
    /// * `src` - Token causing the error
    /// * `format` - Type of formatting
    /// * `cause` - Reason for the error
    pub fn new(src: &Token, format: &str, cause: &str) -> Self {
        Self {
            format: format.to_string(),
            cause: cause.to_string(),
            src: ErrorSource::new(src),
        }
    }

    /// Return the expected format
    pub fn format(&self) -> &str {
        &self.format
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

impl Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} parsing error: {} {}",
            self.format, self.cause, self.src
        )?;
        fmt::Result::Ok(())
    }
}
