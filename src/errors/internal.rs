use super::ErrorSource;
use crate::Token;

use std::fmt::{self, Display};

const BUG_REPORT_URL : &str = "https://github.com/rscarson/lavendeux-parser/issues/new?assignees=&labels=&template=bug_report.md&title=";

/// An error caused by a problem with the parser itself
#[derive(Debug, Clone)]
pub struct InternalError {
    src: ErrorSource,
}
impl InternalError {
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

impl Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "internal parser issue {}.\nPlease report this problem at {}",
            self.src, BUG_REPORT_URL
        )?;
        fmt::Result::Ok(())
    }
}
