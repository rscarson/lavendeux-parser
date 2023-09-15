use crate::Token;
use crate::errors::*;

use std::fmt::{self, Display};

/// An error caused by calling a function using an argument of the wrong type
#[derive(Debug, Clone)]
pub struct FunctionArgTypeError {
    arg: usize, 
    expected: ExpectedTypes, 
    signature: String,
    src: ParserErrorSource
}
impl FunctionArgTypeError {
    /// Create a new instance of this error
    /// 
    /// # Arguments
    /// * `src` - Token causing the error
    /// * `signature` - Function signature
    /// * `arg` - Argument index
    /// * `expected` - Expected type of value for the argument
    pub fn new(src: &Token, signature: &str, arg: usize, expected: ExpectedTypes) -> Self {
        Self {
            arg, 
            expected, 
            signature: signature.to_string(),
            src: ParserErrorSource::new(src)
        }
    }

    /// Function call signature 
    pub fn signature(&self) -> &str {
        &self.signature
    }

    /// Expected value type
    pub fn expected(&self) -> &ExpectedTypes {
        &self.expected
    }

    /// Offending argument number
    pub fn arg(&self) -> usize {
        self.arg
    }

    /// Describes the location and text of the bad token
    pub fn source(&self) -> &ParserErrorSource {
        &self.src
    }
}

impl Display for FunctionArgTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: invalid type for argument {} (expected {}) {}", self.signature, self.arg, self.expected, self.src)?;
        fmt::Result::Ok(())
    }
}