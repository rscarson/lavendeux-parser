use crate::Error;
use crate::Token;

use std::fmt::{self, Display};

/// An error caused by an unknown exception in a javascript extension
#[derive(Debug, Clone)]
pub struct ScriptError {
    filename: String,
    cause: String,
    src: ErrorSource,
}
impl ScriptError {
    /// Create a new instance of this error
    ///
    /// # Arguments
    /// * `src` - Token causing the error
    /// * `filename` - Reason for the error
    /// * `cause` - Reason for the error
    pub fn new(src: &Token, filename: &str, cause: &str) -> Self {
        Self {
            filename: filename.to_string(),
            cause: cause.to_string(),
            src: ErrorSource::new(src),
        }
    }

    /// Return the filename causing the error
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Return the cause of the error
    pub fn cause(&self) -> &str {
        &self.cause
    }

    /// Describes the location and text of the bad token
    pub fn source(&self) -> &ErrorSource {
        &self.src
    }

    /// Create a new instance of this error from an existing error
    ///
    /// # Arguments
    /// * `src` - Token causing the error
    /// * `name` - File or function name
    /// * `error`- source error
    #[cfg(feature = "extensions")]
    pub fn from_jserror(src: &Token, name: &str, error: js_playground::Error) -> Self {
        if matches!(error, js_playground::Error::JsonDecode(_)) {
            Self::new(src, name, &format!("{}: {}", name, &error.to_string()))
        } else {
            Self::new(src, name, &error.to_string())
        }
    }
}

impl Display for ScriptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.cause, self.src)?;
        fmt::Result::Ok(())
    }
}
