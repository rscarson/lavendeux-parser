extern crate pest;
use std::error::Error;
use std::fmt;

mod functions;
pub use functions::*;

mod values;
pub use values::*;

#[derive(From, Debug, Clone)]
pub enum ParserError {
    Pest(PestError),
    ParseInt(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
    ValueType(ValueTypeError),
    Overflow(OverflowError),
    Underflow(UnderflowError),
    VariableName(VariableNameError),
    ContantValue(ConstantValueError),

    Script(ScriptError),
    DecoratorName(DecoratorNameError),
    FunctionName(FunctionNameError),
    FunctionArgType(FunctionArgTypeError),
    FunctionArgOverFlow(FunctionArgOverFlowError),
    FunctionNArg(FunctionNArgError)
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
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

#[derive(Debug, Clone)]
pub struct Position{pub line: usize, pub col: usize}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}, col {}", self.line, self.col)
    }
}

#[derive(Debug, Clone)]
pub enum ExpectedTypes {Int, Float, IntOrFloat, String}
impl fmt::Display for ExpectedTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpectedTypes::Int => write!(f, "int"),
            ExpectedTypes::Float => write!(f, "float"),
            ExpectedTypes::IntOrFloat => write!(f, "int or float"),
            ExpectedTypes::String => write!(f, "string"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PestError {pub cause: String}
error_type!(PestError, {
    pub fn new (cause: &str) -> Self {
        Self { cause: cause.to_string() } }
}, {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cause)
    }
});