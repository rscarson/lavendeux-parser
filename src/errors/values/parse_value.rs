use crate::Token;
use crate::errors::*;

use std::error::Error;
use std::fmt::{self, Display};

/// An error caused by attempting to parse an value
#[derive(Debug, Clone)]
pub struct ParseValueError {
    cause: String,
    variant: ExpectedTypes,
    src: ParserErrorSource
}
impl ParseValueError {
    /// Create a new instance of this error
    /// 
    /// # Arguments
    /// * `src` - Token causing the error
    /// * `cause` - Reason for the error
    /// * `variant` - Type of value attempting to be parsed
    pub fn new(src: &Token, cause: &str, variant: ExpectedTypes) -> Self {
        Self {
            cause: cause.to_string(),
            variant: variant,
            src: ParserErrorSource::new(src)
        }
    }

    /// Return the cause of the error
    pub fn cause(&self) -> &str {
        &self.cause
    }

    /// Return the type of value requested
    pub fn variant(&self) -> &ExpectedTypes {
        &self.variant
    }

    /// Describes the location and text of the bad token
    pub fn source(&self) -> &ParserErrorSource {
        &self.src
    }
}

impl Error for ParseValueError {}
impl Display for ParseValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant = format!("{}", self.variant);
        let suffix = if ['a','e','i','o','u'].contains(&variant.chars().next().unwrap()) {"n"} else {" "};
        write!(f, "could not parse {} as a{suffix} {} {}", self.cause, self.variant, self.src)?;
        fmt::Result::Ok(())
    }
}

impl Into<ParserError> for ParseValueError {
    fn into(self) -> ParserError {
        ParserError::ParseValue(self)
    }
}