use crate::Token;
use crate::errors::*;

use std::fmt::{self, Display};

/// An error caused by a function argument overflowing a pre-determined limit
#[derive(Debug, Clone)]
pub struct FunctionOverflowError {
    arg: usize, 
    signature: String,
    src: ParserErrorSource
}
impl FunctionOverflowError {
    /// Create a new instance of this error
    /// 
    /// # Arguments
    /// * `src` - Token causing the error
    /// * `signature` - Function signature
    /// * `arg` - argument index
    pub fn new(src: &Token, signature: &str, arg: usize) -> Self {
        Self {
            arg,
            signature: signature.to_string(),
            src: ParserErrorSource::new(src)
        }
    }

    /// Function call signature 
    pub fn signature(&self) -> &str {
        &self.signature
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

impl Display for FunctionOverflowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: overflow in argument {} {}", self.signature, self.arg, self.src)?;
        fmt::Result::Ok(())
    }
}