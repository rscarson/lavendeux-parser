use crate::Token;
use crate::errors::*;

use std::fmt::{self, Display};

/// An error caused by ending a script on a backslash
#[derive(Debug, Clone)]
pub struct UnterminatedLinebreakError {
    src: ParserErrorSource
}
impl UnterminatedLinebreakError {
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

impl Display for UnterminatedLinebreakError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "missing newline after '\\' {}", self.src)?;
        fmt::Result::Ok(())
    }
}