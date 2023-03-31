use std::error::Error;
use std::fmt;
use crate::Token;
use super::ExpectedTypes;
use super::error_macro;

/// Occurs when a JS extension throws an unexpected error
#[derive(Debug, Clone)]
pub struct ScriptError {pos: Option<usize>, error: String}
error_macro::error_type!(ScriptError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `error` - Underlying error
    pub fn new(error: &str) -> Self {
        Self::new_with_index(None, error)
    }
    
    /// Create a new instance of the error caused by a token
    /// 
    /// # Arguments
    /// * `token` - Token causing the error
    /// * `error` - Underlying error
    pub fn new_with_token(token: &Token, error: &str) -> Self {
        Self::new_with_index(Some(token.index()), error)
    }
    
    /// Create a new instance of the error at a specific position
    /// 
    /// # Arguments
    /// * `pos` - Index at which the error occured
    /// * `error` - Underlying error
    pub fn new_with_index(pos: Option<usize>, error: &str) -> Self {
        Self { pos, error: error.to_string() }
    }

    /// Error source 
    pub fn error(&self) -> &str {
        &self.error
    }
    
    /// Return the location at which the error occured
    pub fn pos(&self) -> Option<usize> {
        self.pos
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)?;
        if let Some(pos) = self.pos {
            write!(f, " at position {}", pos)?;
        }

        fmt::Result::Ok(())
    }
});

/// Occurs when a decorator was not found
#[derive(Debug, Clone)]
pub struct DecoratorNameError {pos: Option<usize>, name: String}
error_macro::error_type!(DecoratorNameError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `name` - Decorator name
    pub fn new(name: &str) -> Self {
        Self::new_with_index(None, name)
    }
    
    /// Create a new instance of the error caused by a token
    /// 
    /// # Arguments
    /// * `token` - Token causing the error
    /// * `name` - Decorator name
    pub fn new_with_token(token: &Token, name: &str) -> Self {
        Self::new_with_index(Some(token.index()), name)
    }
    
    /// Create a new instance of the error at a specific position
    /// 
    /// # Arguments
    /// * `pos` - Index at which the error occured
    /// * `name` - Decorator name
    pub fn new_with_index(pos: Option<usize>, name: &str) -> Self {
        Self { pos, name: name.to_string() }
    }

    /// Decorator name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Return the location at which the error occured
    pub fn pos(&self) -> Option<usize> {
        self.pos
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no such decorator {}", self.name)?;
        if let Some(pos) = self.pos {
            write!(f, " at position {}", pos)?;
        }

        fmt::Result::Ok(())
    }
});

/// Occurs when a function was not found
#[derive(Debug, Clone)]
pub struct FunctionNameError {pos: Option<usize>, name: String}
error_macro::error_type!(FunctionNameError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `name` - Function name
    pub fn new(name: &str) -> Self {
        Self::new_with_index(None, name)
    }
    
    /// Create a new instance of the error caused by a token
    /// 
    /// # Arguments
    /// * `token` - Token causing the error
    /// * `name` - Function name
    pub fn new_with_token(token: &Token, name: &str) -> Self {
        Self::new_with_index(Some(token.index()), name)
    }

    /// Create a new instance of the error at a specific position
    /// 
    /// # Arguments
    /// * `pos` - Index at which the error occured
    /// * `name` - Function name
    pub fn new_with_index(pos: Option<usize>, name: &str) -> Self {
        Self { pos, name: name.to_string() } 
    }

    /// Function name 
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Return the location at which the error occured
    pub fn pos(&self) -> Option<usize> {
        self.pos
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no such function {}", self.name)?;
        if let Some(pos) = self.pos {
            write!(f, " at position {}", pos)?;
        }

        fmt::Result::Ok(())
    }
});

/// Occurs when supplying a function with the wrong type of argument
#[derive(Debug, Clone)]
pub struct FunctionArgTypeError {pos: Option<usize>, arg: usize, expected: ExpectedTypes, signature: String}
error_macro::error_type!(FunctionArgTypeError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `signature` - Function call signature
    /// * `arg` - 1-indexed argument number
    /// * `expected` - Expected type of argument
    pub fn new (signature: &str, arg: usize, expected: ExpectedTypes) -> Self {
        Self::new_with_index(None, signature, arg, expected)
    }
    
    /// Create a new instance of the error caused by a token
    /// 
    /// # Arguments
    /// * `token` - Token causing the error
    /// * `signature` - Function call signature
    /// * `arg` - 1-indexed argument number
    /// * `expected` - Expected type of argument
    pub fn new_with_token(token: &Token, signature: &str, arg: usize, expected: ExpectedTypes) -> Self {
        Self::new_with_index(Some(token.index()), signature, arg, expected)
    }

    /// Create a new instance of the error at a specific position
    /// 
    /// # Arguments
    /// * `pos` - Index at which the error occured
    /// * `signature` - Function call signature
    /// * `arg` - 1-indexed argument number
    /// * `expected` - Expected type of argument
    pub fn new_with_index(pos: Option<usize>, signature: &str, arg: usize, expected: ExpectedTypes) -> Self {
        Self { pos, arg, expected, signature: signature.to_string() } 
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
    
    /// Return the location at which the error occured
    pub fn pos(&self) -> Option<usize> {
        self.pos
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: invalid type for argument {} (expected {})", self.signature, self.arg, self.expected)?;
        if let Some(pos) = self.pos {
            write!(f, " at position {}", pos)?;
        }

        fmt::Result::Ok(())
    }
});

/// Occurs when a function argument causes an overflow
#[derive(Debug, Clone)]
pub struct FunctionArgOverFlowError {pos: Option<usize>, arg: usize, signature: String}
error_macro::error_type!(FunctionArgOverFlowError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `signature` - Function call signature
    /// * `arg` - 1-indexed argument number
    pub fn new(signature: &str, arg: usize) -> Self {
        Self::new_with_index(None, signature, arg)
    }
    
    /// Create a new instance of the error caused by a token
    /// 
    /// # Arguments
    /// * `token` - Token causing the error
    /// * `signature` - Function call signature
    /// * `arg` - 1-indexed argument number
    pub fn new_with_token(token: &Token, signature: &str, arg: usize) -> Self {
        Self::new_with_index(Some(token.index()), signature, arg)
    }

    /// Create a new instance of the error at a specific position
    /// 
    /// # Arguments
    /// * `pos` - Index at which the error occured
    /// * `signature` - Function call signature
    /// * `arg` - 1-indexed argument number
    pub fn new_with_index(pos: Option<usize>, signature: &str, arg: usize) -> Self {
        Self { pos, arg, signature: signature.to_string() }
    }

    /// Function call signature 
    pub fn signature(&self) -> &str {
        &self.signature
    }

    /// Offending argument number
    pub fn arg(&self) -> usize {
        self.arg
    }
    
    /// Return the location at which the error occured
    pub fn pos(&self) -> Option<usize> {
        self.pos
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: overflow in argument {}", self.signature, self.arg)?;
        if let Some(pos) = self.pos {
            write!(f, " at position {}", pos)?;
        }

        fmt::Result::Ok(())
    }
});

/// Occurs when a function is called with the wrong number of arguments
#[derive(Debug, Clone)]
pub struct FunctionNArgError {pos: Option<usize>, min: usize, max: usize, signature: String}
error_macro::error_type!(FunctionNArgError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `signature` - Function call signature
    /// * `min` - Smallest allowed number of arguments
    /// * `max` - Largest allowed number of arguments
    pub fn new(signature: &str, min: usize, max: usize) -> Self {
        Self::new_with_index(None, signature, min, max )
    }
    
    /// Create a new instance of the error caused by a token
    /// 
    /// # Arguments
    /// * `token` - Token causing the error
    /// * `signature` - Function call signature
    /// * `min` - Smallest allowed number of arguments
    /// * `max` - Largest allowed number of arguments
    pub fn new_with_token(token: &Token, signature: &str, min: usize, max: usize) -> Self {
        Self::new_with_index(Some(token.index()), signature, min, max)
    }
    
    /// Create a new instance of the error at a specific position
    /// 
    /// # Arguments
    /// * `pos` - Index at which the error occured
    /// * `signature` - Function call signature
    /// * `min` - Smallest allowed number of arguments
    /// * `max` - Largest allowed number of arguments
    pub fn new_with_index(pos: Option<usize>, signature: &str, min: usize, max: usize) -> Self {
        Self { pos, min, max, signature: signature.to_string() }
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
    
    /// Return the location at which the error occured
    pub fn pos(&self) -> Option<usize> {
        self.pos
    }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.min == self.max {
            write!(f, "{}: expected {} args", self.signature, self.min)?;
        } else {
            write!(f, "{}: expected {}-{} args", self.signature, self.min, self.max)?;
        }

        if let Some(pos) = self.pos {
            write!(f, " at position {}", pos)?;
        }

        fmt::Result::Ok(())
    }
});