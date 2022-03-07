extern crate pest;

use std::error::Error;
use std::fmt;

use super::ExpectedTypes;

#[derive(Debug, Clone)]
pub struct UnderflowError {}
impl fmt::Display for UnderflowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "arithmetic underflow")
    }
}
impl Error for UnderflowError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct OverflowError {}
impl fmt::Display for OverflowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "arithmetic overflow")
    }
}
impl Error for OverflowError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct ValueTypeError {pub expected: ExpectedTypes}
impl fmt::Display for ValueTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid type for value (expected {})", self.expected)
    }
}
impl Error for ValueTypeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct ConstantValueError {pub name: String}
impl fmt::Display for ConstantValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot assign to constant '{}'", self.name)
    }
}
impl Error for ConstantValueError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}


#[derive(Debug, Clone)]
pub struct VariableNameError {pub name: String}
impl fmt::Display for VariableNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "uninitialized variable '{}' referenced", self.name)
    }
}
impl Error for VariableNameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
