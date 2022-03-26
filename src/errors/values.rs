extern crate pest;

use std::error::Error;
use std::fmt;

use super::ExpectedTypes;
use super::error_macro;

#[derive(Debug, Clone)]
pub struct UnderflowError {}
error_macro::error_type!(UnderflowError, {
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

#[derive(Debug, Clone)]
pub struct OverflowError {}
error_macro::error_type!(OverflowError, {
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

#[derive(Debug, Clone)]
pub struct ValueTypeError {pub expected: ExpectedTypes}
error_macro::error_type!(ValueTypeError, {
    pub fn new (expected: ExpectedTypes) -> Self {
        Self {expected}
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid type for value (expected {})", self.expected)
    }
});

#[derive(Debug, Clone)]
pub struct ConstantValueError {pub name: String}
error_macro::error_type!(ConstantValueError, {
    pub fn new (name: String) -> Self {
        Self {name}
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot assign to constant '{}'", self.name)
    }
});

#[derive(Debug, Clone)]
pub struct VariableNameError {pub name: String}
error_macro::error_type!(VariableNameError, {
    pub fn new (name: String) -> Self {
        Self {name}
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "uninitialized variable '{}' referenced", self.name)
    }
});
