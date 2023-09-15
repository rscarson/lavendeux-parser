use crate::Token;
use crate::errors::*;

use std::error::Error;
use std::fmt::{self, Display};

/// An error caused by attempting to parse an invalid string into a given format
#[derive(Debug, Clone)]
pub struct ParsingError {
    format: String,
    cause: String,
    src: ParserErrorSource
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
            src: ParserErrorSource::new(src)
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
    pub fn source(&self) -> &ParserErrorSource {
        &self.src
    }
}

impl Error for ParsingError {}
impl Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} parsing error: {} {}", self.format, self.cause, self.src)?;
        fmt::Result::Ok(())
    }
}

impl Into<ParserError> for ParsingError {
    fn into(self) -> ParserError {
        ParserError::Parsing(self)
    }
}