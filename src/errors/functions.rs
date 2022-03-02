use std::error::Error;
use std::fmt;

use super::ExpectedTypes;
use super::error_macro;

#[derive(Debug)]
pub struct ScriptError {pub error: String}
error_macro::error_type!(ScriptError, {
    pub fn new (error: &str) -> Self {
        Self { error: error.to_string() } }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error executing extension: {}", self.error)
    }
});

#[derive(Debug)]
pub struct DecoratorNameError {pub name: String}
error_macro::error_type!(DecoratorNameError, {
    pub fn new (name: &str) -> Self {
        Self { name: name.to_string() } }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no such decorator {}", self.name)
    }
});

#[derive(Debug)]
pub struct FunctionNameError {pub name: String}
error_macro::error_type!(FunctionNameError, {
    pub fn new (name: &str) -> Self {
        Self { name: name.to_string() } }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no such function {}", self.name)
    }
});

#[derive(Debug)]
pub struct FunctionArgTypeError {pub arg: usize, pub expected: ExpectedTypes, pub signature: String}
error_macro::error_type!(FunctionArgTypeError, {
    pub fn new (signature: &str, arg: usize, expected: ExpectedTypes) -> Self {
        Self { arg: arg, expected: expected, signature: signature.to_string() } }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: invalid type for argument {} (expected {})", self.signature, self.arg, self.expected)
    }
});

#[derive(Debug)]
pub struct FunctionArgOverFlowError {pub arg: usize, pub signature: String}
error_macro::error_type!(FunctionArgOverFlowError, {
    pub fn new (signature: &str, arg: usize) -> Self {
        Self { arg: arg, signature: signature.to_string() } }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: overflow in argument {}", self.signature, self.arg)
    }
});

#[derive(Debug)]
pub struct FunctionNArgError {pub min: usize, pub max: usize, pub signature: String}
error_macro::error_type!(FunctionNArgError, {
    pub fn new (signature: &str, min: usize, max: usize) -> Self {
        Self { min: min, max: max, signature: signature.to_string() } }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.min == self.max {
            write!(f, "{}: expected {} args", self.signature, self.min)
        } else {
            write!(f, "{}: expected {}-{} args", self.signature, self.min, self.max)
        }
    }
});