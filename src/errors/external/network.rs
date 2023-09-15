use crate::Token;
use crate::errors::*;

use std::error::Error;
use std::fmt::{self, Display};

/// An error caused by network issues
#[derive(Debug, Clone)]
pub struct NetworkError {
    cause: String,
    src: ParserErrorSource
}
impl NetworkError {
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

    /// Create a new instance of this error from an existing error
    /// 
    /// # Arguments
    /// * `src` - Token causing the error
    /// * `error`- source error
    pub fn from_reqwesterror(src: &Token, error: reqwest::Error) -> Self {
        Self::new(src, &error.to_string())
    }
}

impl Error for NetworkError {}
impl Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "network error: {} {}", self.cause, self.src)?;
        fmt::Result::Ok(())
    }
}

impl Into<ParserError> for NetworkError {
    fn into(self) -> ParserError {
        ParserError::Network(self)
    }
}