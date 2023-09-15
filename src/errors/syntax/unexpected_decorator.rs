use crate::Token;
use crate::errors::*;

use std::error::Error;
use std::fmt::{self, Display};

/// An error caused by using a decorator in the wrong place
#[derive(Debug, Clone)]
pub struct UnexpectedDecoratorError {
    src: ParserErrorSource
}
impl UnexpectedDecoratorError {
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

impl Error for UnexpectedDecoratorError {}
impl Display for UnexpectedDecoratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@decorators must be at the end of a statement {}", self.src)?;
        fmt::Result::Ok(())
    }
}

impl Into<ParserError> for UnexpectedDecoratorError {
    fn into(self) -> ParserError {
        ParserError::UnexpectedDecorator(self)
    }
}