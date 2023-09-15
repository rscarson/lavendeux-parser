use crate::Token;
use crate::errors::*;

use std::error::Error;
use std::fmt::{self, Display};

/// An error caused by a missing quote
#[derive(Debug, Clone)]
pub struct UnterminatedLiteralError {
    src: ParserErrorSource
}
impl UnterminatedLiteralError {
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

impl Error for UnterminatedLiteralError {}
impl Display for UnterminatedLiteralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unterminated string literal {}", self.src)?;
        fmt::Result::Ok(())
    }
}

impl Into<ParserError> for UnterminatedLiteralError {
    fn into(self) -> ParserError {
        ParserError::UnterminatedLiteral(self)
    }
}