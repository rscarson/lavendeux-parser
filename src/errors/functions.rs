use std::error::Error;
use std::fmt;

use super::ExpectedTypes;
use super::error_macro;

/// Occurs when a JS extension throws an unexpected error
#[derive(Debug, Clone)]
pub struct ScriptError {error: String}
error_macro::error_type!(ScriptError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `error` - Underlying error
    pub fn new (error: &str) -> Self {
        Self { error: error.to_string() }
    }

    /// Error source 
    pub fn error(&self) -> &str {
        &self.error
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error executing extension: {}", self.error)
    }
});

/// Occurs when a decorator was not found
#[derive(Debug, Clone)]
pub struct DecoratorNameError {name: String}
error_macro::error_type!(DecoratorNameError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `name` - Decorator name
    pub fn new (name: &str) -> Self {
        Self { name: name.to_string() }
    }

    /// Decorator name
    pub fn name(&self) -> &str {
        &self.name
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no such decorator {}", self.name)
    }
});

/// Occurs when a function was not found
#[derive(Debug, Clone)]
pub struct FunctionNameError {name: String}
error_macro::error_type!(FunctionNameError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `name` - Function name
    pub fn new (name: &str) -> Self {
        Self { name: name.to_string() } 
    }

    /// Function name 
    pub fn name(&self) -> &str {
        &self.name
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no such function {}", self.name)
    }
});

/// Occurs when supplying a function with the wrong type of argument
#[derive(Debug, Clone)]
pub struct FunctionArgTypeError {arg: usize, expected: ExpectedTypes, signature: String}
error_macro::error_type!(FunctionArgTypeError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `signature` - Function call signature
    /// * `arg` - 1-indexed argument number
    /// * `expected` - Expected type of argument
    pub fn new (signature: &str, arg: usize, expected: ExpectedTypes) -> Self {
        Self { arg, expected, signature: signature.to_string() } 
    }

    /// Function call signature 
    pub fn signature(&self) -> &str {
        &self.signature
    }

    /// Expected value type
    pub fn expected(&self) -> ExpectedTypes {
        self.expected.clone()
    }

    /// Offending argument number
    pub fn arg(&self) -> usize {
        self.arg
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: invalid type for argument {} (expected {})", self.signature, self.arg, self.expected)
    }
});

/// Occurs when a function argument causes an overflow
#[derive(Debug, Clone)]
pub struct FunctionArgOverFlowError {arg: usize, signature: String}
error_macro::error_type!(FunctionArgOverFlowError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `signature` - Function call signature
    /// * `arg` - 1-indexed argument number
    pub fn new (signature: &str, arg: usize) -> Self {
        Self { arg, signature: signature.to_string() }
    }

    /// Function call signature 
    pub fn signature(&self) -> &str {
        &self.signature
    }

    /// Offending argument number
    pub fn arg(&self) -> usize {
        self.arg
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: overflow in argument {}", self.signature, self.arg)
    }
});

/// Occurs when a function is called with the wrong number of arguments
#[derive(Debug, Clone)]
pub struct FunctionNArgError {min: usize, max: usize, signature: String}
error_macro::error_type!(FunctionNArgError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `signature` - Function call signature
    /// * `min` - Smallest allowed number of arguments
    /// * `max` - Largest allowed number of arguments
    pub fn new (signature: &str, min: usize, max: usize) -> Self {
        Self { min, max, signature: signature.to_string() }
    }

    /// Function call signature 
    pub fn signature(&self) -> &str {
        &self.signature
    }

    /// Smallest allowed number of arguments 
    pub fn min(&self) -> usize {
        self.min
    }

    /// Largest allowed number of arguments 
    pub fn max(&self) -> usize {
        self.max
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.min == self.max {
            write!(f, "{}: expected {} args", self.signature, self.min)
        } else {
            write!(f, "{}: expected {}-{} args", self.signature, self.min, self.max)
        }
    }
});