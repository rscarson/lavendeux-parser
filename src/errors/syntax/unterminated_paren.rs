use crate::Token;
use crate::errors::*;

use std::error::Error;
use std::fmt::{self, Display};

/// An error caused by a missing parentheses
#[derive(Debug, Clone)]
pub struct UnterminatedParenError {
    src: ParserErrorSource
}
impl UnterminatedParenError {
    /// Create a new instance of this error
    /// 
    /// # Arguments
    /// * `src` - Token causing the error
    pub fn new(src: &Token) -> Self {
        Self {
            src: ParserErrorSource::new(src)
        }
    }

    /// Describes the location and text of the bad token
    pub fn source(&self) -> &ParserErrorSource {
        &self.src
    }
}

impl Error for UnterminatedParenError {}
impl Display for UnterminatedParenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "missing ')' {}", self.src)?;
        fmt::Result::Ok(())
    }
}

impl Into<ParserError> for UnterminatedParenError {
    fn into(self) -> ParserError {
        ParserError::UnterminatedParen(self)
    }
}