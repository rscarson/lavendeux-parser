use crate::Token;
use crate::errors::*;

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

impl Display for UnterminatedParenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "missing ')' {}", self.src)?;
        fmt::Result::Ok(())
    }
}