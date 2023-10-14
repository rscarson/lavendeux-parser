use crate::Error;
use crate::ExpectedTypes;
use crate::Token;

use std::fmt::{self, Display};

/// An error caused by attempting to use a value of the wrong type in a calculation
#[derive(Debug, Clone)]
pub struct ValueTypeError {
    expected: ExpectedTypes,
    src: ErrorSource,
}
impl ValueTypeError {
    /// Create a new instance of this error
    ///
    /// # Arguments
    /// * `src` - Token causing the error
    /// * `expected` - The type of value required
    pub fn new(src: &Token, expected: ExpectedTypes) -> Self {
        Self {
            expected,
            src: ErrorSource::new(src),
        }
    }

    /// Return the type that was expected
    pub fn expected(&self) -> &ExpectedTypes {
        &self.expected
    }

    /// Describes the location and text of the bad token
    pub fn source(&self) -> &ErrorSource {
        &self.src
    }
}

impl Display for ValueTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invalid type for value, expected {} {}",
            self.expected, self.src
        )?;
        fmt::Result::Ok(())
    }
}
