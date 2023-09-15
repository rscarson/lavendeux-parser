use crate::Token;
use crate::errors::*;

use std::error::Error;
use std::fmt::{self, Display};

/// An error caused by calling a decorator using an argument of the wrong type
#[derive(Debug, Clone)]
pub struct DecoratorArgTypeError {
    expected: ExpectedTypes, 
    signature: String,
    src: ParserErrorSource
}
impl DecoratorArgTypeError {
    /// Create a new instance of this error
    /// 
    /// # Arguments
    /// * `src` - Token causing the error
    /// * `signature` - decorator signature
    /// * `expected` - Expected type of value for the argument
    pub fn new(src: &Token, signature: &str, expected: ExpectedTypes) -> Self {
        Self {
            expected, 
            signature: signature.to_string(),
            src: ParserErrorSource::new(src)
        }
    }

    /// Decorator call signature 
    pub fn signature(&self) -> &str {
        &self.signature
    }

    /// Expected value type
    pub fn expected(&self) -> &ExpectedTypes {
        &self.expected
    }

    /// Describes the location and text of the bad token
    pub fn source(&self) -> &ParserErrorSource {
        &self.src
    }
}

impl Error for DecoratorArgTypeError {}
impl Display for DecoratorArgTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid type for decorator {} (expected {}) {}", self.signature, self.expected, self.src)?;
        fmt::Result::Ok(())
    }
}

impl Into<ParserError> for DecoratorArgTypeError {
    fn into(self) -> ParserError {
        ParserError::DecoratorArgType(self)
    }
}