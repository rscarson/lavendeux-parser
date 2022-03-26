extern crate pest;

use std::error::Error;
use std::fmt;

use super::ExpectedTypes;
use super::error_macro;

/// Occurs when a calculation causes an underflow
#[derive(Debug, Clone)]
pub struct UnderflowError {}
error_macro::error_type!(UnderflowError, {
    /// Create a new instance of the error
    pub fn new () -> Self {
        Self {}
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "arithmetic underflow")
    }
});
impl Default for UnderflowError {
    fn default() -> Self {
        Self::new()
    }
}

/// Occurs when a calculation causes an underflow
#[derive(Debug, Clone)]
pub struct OverflowError {}
error_macro::error_type!(OverflowError, {
    /// Create a new instance of the error
    pub fn new () -> Self {
        Self {}
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "arithmetic overflow")
    }
});
impl Default for OverflowError {
    fn default() -> Self {
        Self::new()
    }
}

/// Occurs when attempting an operation with the wrong type of value
#[derive(Debug, Clone)]
pub struct ValueTypeError {expected: ExpectedTypes}
error_macro::error_type!(ValueTypeError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `expected` - Expected type of value
    pub fn new (expected: ExpectedTypes) -> Self {
        Self {expected}
    }

    /// The expected value type
    pub fn expected(&self) -> ExpectedTypes {
        self.expected.clone()
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid type for value (expected {})", self.expected)
    }
});

/// Occurs when attempting to overwrite a constant
#[derive(Debug, Clone)]
pub struct ConstantValueError {name: String}
error_macro::error_type!(ConstantValueError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `name` - Constant that could not be overwritten
    pub fn new (name: String) -> Self {
        Self {name}
    }

    /// The constant's name
    pub fn name(&self) -> &str {
        &self.name
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot assign to constant '{}'", self.name)
    }
});

/// Occurs when attempting to use an unknown variable
#[derive(Debug, Clone)]
pub struct VariableNameError {name: String}
error_macro::error_type!(VariableNameError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `name` - Variable that was used
    pub fn new (name: String) -> Self {
        Self {name}
    }

    /// The variable's name
    pub fn name(&self) -> &str {
        &self.name
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "uninitialized variable '{}' referenced", self.name)
    }
});
