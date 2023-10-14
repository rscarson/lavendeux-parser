use crate::Error;
use crate::ExpectedTypes;
use crate::Token;

use std::fmt::{self, Display};

/// An error caused by calling a decorator using an argument of the wrong type
#[derive(Debug, Clone)]
pub struct DecoratorArgTypeError {
    expected: ExpectedTypes,
    signature: String,
    src: ErrorSource,
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
            src: ErrorSource::new(src),
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
    pub fn source(&self) -> &ErrorSource {
        &self.src
    }
}

impl Display for DecoratorArgTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invalid type for decorator {} (expected {}) {}",
            self.signature, self.expected, self.src
        )?;
        fmt::Result::Ok(())
    }
}
