use crate::Error;
use crate::Token;

use std::fmt::{self, Display};

/// An error caused by using a decorator in the wrong place
#[derive(Debug, Clone)]
pub struct UnexpectedDecoratorError {
    src: ErrorSource,
}
impl UnexpectedDecoratorError {
    /// Create a new instance of this error
    ///
    /// # Arguments
    /// * `src` - Token causing the error
    pub fn new(src: &Token) -> Self {
        Self {
            src: ErrorSource::new(src),
        }
    }

    /// Describes the location and text of the bad token
    pub fn source(&self) -> &ErrorSource {
        &self.src
    }
}

impl Display for UnexpectedDecoratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "@decorators must be at the end of a statement {}",
            self.src
        )?;
        fmt::Result::Ok(())
    }
}
