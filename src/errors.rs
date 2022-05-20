extern crate pest;
use crate::Token;
use std::error::Error;
use std::fmt;

mod functions;
pub use functions::*;

mod values;
pub use values::*;

/// Represents all possible errors during expression handling
#[derive(Debug, Clone)]
pub enum ParserError {
    /// An error with an unknown cause
    General(String),
    
    /// An error caused by a recursive function going too deep
    Stack(StackError),
    
    /// An error caused by a problem in parsing the grammar of an expression
    Pest(PestError),
    
    /// An error caused by attempting to parse an invalid integer value
    ParseInt(ParseIntegerError),
    
    /// An error caused by attempting to parse an invalid floating point value
    ParseFloat(ParseFloatingPointError),
    
    /// An error caused by attempting to use a value of the wrong type in a calculation
    ValueType(ValueTypeError),
    
    /// An error caused by a calculation that resulted in an overflow
    Overflow(OverflowError),
    
    /// An error caused by a calculation that resulted in an underflow
    Underflow(UnderflowError),
    
    /// An error caused by attempting to use an unassigned variable
    VariableName(VariableNameError),

    /// An error caused by attempting to overwrite a constant
    ContantValue(ConstantValueError),
    
    /// An error caused by attempting to use arrays of incompatible lengths
    ArrayLength(ArrayLengthError),

    /// An error caused by attempting to use an out of bounds index on an array
    ArrayIndex(ArrayIndexError),

    /// An error caused by an unknown exception in a javascript extension
    Script(ScriptError),

    /// An error caused by calling a decorator that does not exist
    DecoratorName(DecoratorNameError),

    /// An error caused by calling a function that does not exist
    FunctionName(FunctionNameError),

    /// An error caused by calling a function using an argument of the wrong type
    FunctionArgType(FunctionArgTypeError),

    /// An error caused by a function argument overflowing a pre-determined limit
    FunctionArgOverFlow(FunctionArgOverFlowError),

    /// An error caused by calling a function using the wrong number of arguments
    FunctionNArg(FunctionNArgError)
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::General(e) => write!(f, "{}", e),
            Self::Stack(e) => write!(f, "{}", e),
            Self::Pest(e) => write!(f, "{}", e),
            Self::ParseInt(e) => write!(f, "{}", e),
            Self::ParseFloat(e) => write!(f, "{}", e),
            Self::ValueType(e) => write!(f, "{}", e),
            Self::Overflow(e) => write!(f, "{}", e),
            Self::Underflow(e) => write!(f, "{}", e),
            Self::VariableName(e) => write!(f, "{}", e),
            Self::ContantValue(e) => write!(f, "{}", e),
            Self::ArrayLength(e) => write!(f, "{}", e),
            Self::ArrayIndex(e) => write!(f, "{}", e),
        
            Self::Script(e) => write!(f, "{}", e),
            Self::DecoratorName(e) => write!(f, "{}", e),
            Self::FunctionName(e) => write!(f, "{}", e),
            Self::FunctionArgType(e) => write!(f, "{}", e),
            Self::FunctionArgOverFlow(e) => write!(f, "{}", e),
            Self::FunctionNArg(e) => write!(f, "{}", e)
        }
        //write!(f, "{}", self)
    }
}
impl From<std::io::Error> for ParserError {
    fn from(error: std::io::Error) -> Self {
        Self::General(error.to_string())
    }
}

/// Macro to shorten error type definitions
#[macro_use]
mod error_macro {
    macro_rules! error_type {
        ($a:ident, $b:tt, $c:tt) => {
            impl $a $b
            impl fmt::Display for $a $c
            impl Error for $a {
                fn source(&self) -> Option<&(dyn Error + 'static)> {
                    None
                }
            }
        };
    }

    pub(crate) use error_type;
}

/// Represents a type of value that was expected
#[derive(Debug, Clone)]
pub enum ExpectedTypes {
    /// Integer value
    Int, 

    /// Floating point value
    Float, 

    /// Any numeric value
    IntOrFloat, 
    
    /// String value
    String,
    
    /// Boolean value
    Boolean,
    
    /// Array value
    Array,
    
    /// Any type of value
    Any
}
impl fmt::Display for ExpectedTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpectedTypes::Int => write!(f, "int"),
            ExpectedTypes::Float => write!(f, "float"),
            ExpectedTypes::IntOrFloat => write!(f, "int or float"),
            ExpectedTypes::String => write!(f, "string"),
            ExpectedTypes::Boolean => write!(f, "boolean"),
            ExpectedTypes::Array => write!(f, "array"),
            ExpectedTypes::Any => write!(f, "any"),
        }
    }
}

/// Occurs when parsing the grammar of an expression fails
#[derive(Debug, Clone)]
pub struct PestError {pos: Option<usize>, cause: String}
error_type!(PestError, {
    /// Create a new instance of the error
    /// 
    /// # Arguments
    /// * `cause` - Underlying parsing error
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
    /// * `cause` - Underlying parsing error
    pub fn new_with_index(pos: Option<usize>, cause: &str) -> Self {
        Self { pos, cause: cause.to_string() }
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
        write!(f, "{}", "unexpected token in expression")?;
        if let Some(pos) = self.pos {
            write!(f, " at position {}", pos)?;
        }

        fmt::Result::Ok(())
    }
});

/// Occurs when a recursive function goes too deep
#[derive(Debug, Clone)]
pub struct StackError {pos: Option<usize>}
error_type!(StackError, {
    /// Create a new instance of the error
    pub fn new() -> Self {
        Self::new_with_index(None)
    }
    
    /// Create a new instance of the error caused by a token
    /// 
    /// # Arguments
    /// * `token` - Token causing the error
    /// * `cause` - Underlying parsing error
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
        write!(f, "{}", "recursive function went too deep")?;
        if let Some(pos) = self.pos {
            write!(f, " at position {}", pos)?;
        }

        fmt::Result::Ok(())
    }
});
impl Default for StackError {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test_builtin_functions {
    use super::*;
    
    #[test]
    fn test_error_string() {
        ParserError::Pest(PestError::new("test")).to_string();
    }
}