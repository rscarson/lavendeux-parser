use crate::Token;
use super::{ParserError, ParserErrorSource};

use std::error::Error;
use std::fmt::{self, Display};

const BUG_REPORT_URL : &str = "https://github.com/rscarson/lavendeux-parser/issues/new?assignees=&labels=&template=bug_report.md&title=";

/// An error caused by a problem with the parser itself
#[derive(Debug, Clone)]
pub struct InternalError {
    src: ParserErrorSource
}
impl InternalError {
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

impl Error for InternalError {}
impl Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "internal parser issue {}.\nPlease report this problem at {}", self.src, BUG_REPORT_URL)?;
        fmt::Result::Ok(())
    }
}

impl Into<ParserError> for InternalError {
    fn into(self) -> ParserError {
        ParserError::Internal(self)
    }
}