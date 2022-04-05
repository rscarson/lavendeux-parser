extern crate pest;

use crate::Token;
use std::error::Error;
use std::fmt;

use super::ExpectedTypes;
use super::error_macro;

/// Occurs when attempting to parse an invalid integer
#[derive(Debug, Clone)]
pub struct ParseIntegerError {pos: Option<usize>, cause: String}
error_macro::error_type!(ParseIntegerError, {
    /// Create a new instance of the error
    pub fn new(cause: &str) -> Self {
        Self::new_with_index(None, cause)
    }
    
    /// Create a new instance of the error caused by a token
    /// 
    /// # Arguments
    /// * `token` - Token causing the error
    /// * `cause` - Underlying parsing error
    pub fn new_with_token(token: &Token, cause: &str) -> Self {
        Self::new_with_index(Some(token.index()), cause)
    }
    
    /// Create a new instance of the error at a specific position
    /// 
    /// # Arguments
    /// * `pos` - Index at which the error occured
    pub fn new_with_index(pos: Option<usize>, cause: &str) -> Self {
        Self {pos, cause: cause.to_string()}
    }

    /// Return the cause of the error
    pub fn cause(&self) -> &str {
        &self.cause
    }
    
    /// Return the location at which the error occured
    pub fn pos(&self) -> Option<usize> {
        self.pos
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not parse integer: {}", self.cause)?;
        if let Some(pos) = self.pos {
            write!(f, " at position {}", pos)?;
        }

        fmt::Result::Ok(())
    }
});

/// Occurs when attempting to parse an invalid float
#[derive(Debug, Clone)]
pub struct ParseFloatingPointError {pos: Option<usize>, cause: String}
error_macro::error_type!(ParseFloatingPointError, {
    /// Create a new instance of the error
    pub fn new(cause: &str) -> Self {
        Self::new_with_index(None, cause)
    }
    
    /// Create a new instance of the error caused by a token
    /// 
    /// # Arguments
    /// * `token` - Token causing the error
    /// * `cause` - Underlying parsing error
    pub fn new_with_token(token: &Token, cause: &str) -> Self {
        Self::new_with_index(Some(token.index()), cause)
    }
    
    /// Create a new instance of the error at a specific position
    /// 
    /// # Arguments
    /// * `pos` - Index at which the error occured
    pub fn new_with_index(pos: Option<usize>, cause: &str) -> Self {
        Self {pos, cause: cause.to_string()}
    }

    /// Return the cause of the error
    pub fn cause(&self) -> &str {
        &self.cause
    }
    
    /// Return the location at which the error occured
    pub fn pos(&self) -> Option<usize> {
        self.pos
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not parse float")?;
        if let Some(pos) = self.pos {
            write!(f, " at position {}", pos)?;
        }

        fmt::Result::Ok(())
    }
});

/// Occurs when a calculation causes an underflow
#[derive(Debug, Clone)]
pub struct UnderflowError {pos: Option<usize>}
error_macro::error_type!(UnderflowError, {
    /// Create a new instance of the error
    pub fn new() -> Self {
        Self::new_with_index(None)
    }
    
    /// Create a new instance of the error caused by a token
    /// 
    /// # Arguments
    /// * `token` - Token causing the error
    pub fn new_with_token(token: &Token) -> Self {
        Self::new_with_index(Some(token.index()))
    }
    
    /// Create a new instance of the error at a specific position
    /// 
    /// # Arguments
    /// * `pos` - Index at which the error occured
    pub fn new_with_index(pos: Option<usize>) -> Self {
        Self {pos}
    }
    
    /// Return the location at which the error occured
    pub fn pos(&self) -> Option<usize> {
        self.pos
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "arithmetic underflow")?;
        if let Some(pos) = self.pos {
            write!(f, " at position {}", pos)?;
        }

        fmt::Result::Ok(())
    }
});
impl Default for UnderflowError {
    fn default() -> Self {
        Self::new()
    }
}

/// Occurs when a calculation causes an overflow
#[derive(Debug, Clone)]
pub struct OverflowError {pos: Option<usize>}
error_macro::error_type!(OverflowError, {
    /// Create a new instance of the error
    pub fn new() -> Self {
        Self::new_with_index(None)
    }
    
    /// Create a new instance of the error caused by a token
    /// 
    /// # Arguments
    /// * `token` - Token causing the error
    pub fn new_with_token(token: &Token) -> Self {
        Self::new_with_index(Some(token.index()))
    }
    
    /// Create a new instance of the error at a specific position
    /// 
    /// # Arguments
    /// * `pos` - Index at which the error occured
    pub fn new_with_index(pos: Option<usize>) -> Self {
        Self {pos}
    }
    
    /// Return the location at which the error occured
    pub fn pos(&self) -> Option<usize> {
        self.pos
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "arithmetic overflow")?;
        if let Some(pos) = self.pos {
            write!(f, " at position {}", pos)?;
        }

        fmt::Result::Ok(())
    }
});
impl Default for OverflowError {
    fn default() -> Self {
        Self::new()
    }
}

/// Occurs when attempting an operation with the wrong type of value
#[derive(Debug, Clone)]
pub struct ValueTypeError {pos: Option<usize>, expected: ExpectedTypes}
error_macro::error_type!(ValueTypeError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `expected` - Expected type of value
    pub fn new(expected: ExpectedTypes) -> Self {
        Self::new_with_index(None, expected)
    }
    
    /// Create a new instance of the error caused by a token
    /// 
    /// # Arguments
    /// * `token` - Token causing the error
    /// * `expected` - Expected type of value
    pub fn new_with_token(token: &Token, expected: ExpectedTypes) -> Self {
        Self::new_with_index(Some(token.index()), expected)
    }
    
    /// Create a new instance of the error at a specific position
    /// 
    /// # Arguments
    /// * `pos` - Index at which the error occured
    /// * `expected` - Expected type of value
    pub fn new_with_index(pos: Option<usize>, expected: ExpectedTypes) -> Self {
        Self {pos, expected}
    }

    /// The expected value type
    pub fn expected(&self) -> ExpectedTypes {
        self.expected.clone()
    }
    
    /// Return the location at which the error occured
    pub fn pos(&self) -> Option<usize> {
        self.pos
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid type for value (expected {})", self.expected)?;
        if let Some(pos) = self.pos {
            write!(f, " at position {}", pos)?;
        }

        fmt::Result::Ok(())
    }
});

/// Occurs when attempting to overwrite a constant
#[derive(Debug, Clone)]
pub struct ConstantValueError {pos: Option<usize>, name: String}
error_macro::error_type!(ConstantValueError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `name` - Constant that could not be overwritten
    pub fn new(name: &str) -> Self {
        Self::new_with_index(None, name)
    }
    
    /// Create a new instance of the error caused by a token
    /// 
    /// # Arguments
    /// * `token` - Token causing the error
    /// * `name` - Constant that could not be overwritten
    pub fn new_with_token(token: &Token, name: &str) -> Self {
        Self::new_with_index(Some(token.index()), name)
    }
    
    /// Create a new instance of the error at a specific position
    /// 
    /// # Arguments
    /// * `pos` - Index at which the error occured
    /// * `name` - Constant that could not be overwritten
    pub fn new_with_index(pos: Option<usize>, name: &str) -> Self {
        Self {pos, name: name.to_string()}
    }

    /// The constant's name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Return the location at which the error occured
    pub fn pos(&self) -> Option<usize> {
        self.pos
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot assign to constant '{}'", self.name)?;
        if let Some(pos) = self.pos {
            write!(f, " at position {}", pos)?;
        }

        fmt::Result::Ok(())
    }
});

/// Occurs when attempting to use an unknown variable
#[derive(Debug, Clone)]
pub struct VariableNameError {pos: Option<usize>, name: String}
error_macro::error_type!(VariableNameError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `name` - Variable that was used
    pub fn new(name: &str) -> Self {
        Self::new_with_index(None, name)
    }
    
    /// Create a new instance of the error caused by a token
    /// 
    /// # Arguments
    /// * `token` - Token causing the error
    /// * `name` - Variable that was used
    pub fn new_with_token(token: &Token, name: &str) -> Self {
        Self::new_with_index(Some(token.index()), name)
    }

    /// Create a new instance of the error at a specific position
    /// 
    /// # Arguments
    /// * `pos` - Index at which the error occured
    /// * `name` - Variable that was used
    pub fn new_with_index(pos: Option<usize>, name: &str) -> Self {
        Self {pos, name: name.to_string()}
    }

    /// The variable's name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Return the location at which the error occured
    pub fn pos(&self) -> Option<usize> {
        self.pos
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "uninitialized variable '{}' referenced", self.name)?;
        if let Some(pos) = self.pos {
            write!(f, " at position {}", pos)?;
        }

        fmt::Result::Ok(())
    }
});
