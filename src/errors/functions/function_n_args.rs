use crate::Token;
use crate::errors::*;

use std::error::Error;
use std::fmt::{self, Display};

/// An error caused by calling a function using the wrong number of arguments
#[derive(Debug, Clone)]
pub struct FunctionNArgsError {
    min: usize, max: usize, 
    signature: String,
    src: ParserErrorSource
}
impl FunctionNArgsError {
    /// Create a new instance of this error
    /// 
    /// # Arguments
    /// * `src` - Token causing the error
    /// * `signature` - Function signature
    /// * `min` - Minimum acceptable number of arguments
    /// * `max` - Maximum acceptable number of arguments
    pub fn new(src: &Token, signature: &str, min: usize, max: usize) -> Self {
        Self {
            min, max, 
            signature: signature.to_string(),
            src: ParserErrorSource::new(src)
        }
    }

    /// Function call signature 
    pub fn signature(&self) -> &str {
        &self.signature
    }

    /// Smallest allowed number of arguments 
    pub fn min(&self) -> usize {
        self.min
    }

    /// Largest allowed number of arguments 
    pub fn max(&self) -> usize {
        self.max
    }

    /// Describes the location and text of the bad token
    pub fn source(&self) -> &ParserErrorSource {
        &self.src
    }
}

impl Error for FunctionNArgsError {}
impl Display for FunctionNArgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n_args = if self.min == self.max {format!("{}", self.min)} else {format!("{}-{}", self.min, self.max)};
        write!(f, "{}: expected {} args {}", self.signature, n_args, self.src)?;
        fmt::Result::Ok(())
    }
}

impl Into<ParserError> for FunctionNArgsError {
    fn into(self) -> ParserError {
        ParserError::FunctionNArgs(self)
    }
}