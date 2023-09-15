use crate::{Value, Token};

use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[macro_use]
mod error_macro {
    macro_rules! define_parser_error {
        ($(($name:ident, $struct:ident, $docs:expr)),+) => {


            #[derive(Debug)]
            /// Error occuring during parsing
            pub enum ParserError {
                $(
                    #[doc = $docs]
                    $name($struct),
                )+
            }

            impl Display for ParserError {
                fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                    match self {  
                        $(
                            Self::$name(inner) => write!(f, "{}", inner),
                        )+
                    }
                }
            }

            impl Error for ParserError {}

            $(
                impl Error for $struct {}
                impl From<$struct> for ParserError {
                    fn from(val: $struct) -> Self {
                        ParserError::$name(val)
                    }
                }
            )+
        };
    }
}

const MAX_DISPLAY_SRC: usize = 8;
/// Location and cause of the error
#[derive(Debug, Clone)]
pub struct ParserErrorSource {
    src: Token
}
impl ParserErrorSource {
    /// Create a new source for an error
    pub fn new(src: &Token) -> Self {
        Self {
            src: src.clone()
        }
    }

    /// Return a reference to the cause of the error
    pub fn token(&self) -> &Token {
        &self.src
    }


    /// Return the location of the error
    pub fn index(&self) -> usize {
        self.src.index()
    }
}
impl Display for ParserErrorSource {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut token_text = self.src.text().split('\n').next().unwrap_or("").to_string();
        if token_text.len() > MAX_DISPLAY_SRC {
            token_text = token_text[0..MAX_DISPLAY_SRC].to_string();
        }
        write!(f, "at {} (col {})", token_text.trim(), self.src.index())
    }
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
    
    /// Object value
    Object,
    
    /// Any type of value
    Any
}

impl ExpectedTypes {
    /// Returns true if the given value matches expectations
    pub fn matches(&self, value: &Value) -> bool {
        if value.is_compound() {
            true
        } else {
            self.strict_matches(value)
        }
    }

    /// Returns true if the given value matches expectations and count
    pub fn strict_matches(&self, value: &Value) -> bool {
        match self {
            ExpectedTypes::Int => value.is_int(),
            ExpectedTypes::Float => value.is_float(),
            ExpectedTypes::IntOrFloat => value.is_numeric(),
            
            // Can be converted from any type
            _ => true
        }
    }
}

impl fmt::Display for ExpectedTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpectedTypes::Int => write!(f, "integer"),
            ExpectedTypes::Float => write!(f, "float"),
            ExpectedTypes::IntOrFloat => write!(f, "integer or float"),
            ExpectedTypes::String => write!(f, "string"),
            ExpectedTypes::Boolean => write!(f, "boolean"),
            ExpectedTypes::Array => write!(f, "array"),
            ExpectedTypes::Object => write!(f, "object"),
            ExpectedTypes::Any => write!(f, "any"),
        }
    }
}

mod external; pub use external::*;
mod functions; pub use functions::*;
mod arrays; pub use arrays::*;
mod values; pub use values::*;
mod syntax; pub use syntax::*;

mod internal; pub use internal::*;

define_parser_error!(
    // Array errors
    (ArrayEmpty, ArrayEmptyError, "An error caused by attempting to use an empty array"),
    (ArrayIndex, ArrayIndexError, "An error caused by attempting to use an out of bounds index on an array"),
    (ArrayLength, ArrayLengthError, "An error caused by attempting to use arrays of different lengths"),

    // Function errors
    (AmbiguousFunction, AmbiguousFunctionError, "An error caused by attempting to use a function with ambiguous arguments"),
    (DecoratorName, DecoratorNameError, "An error caused by calling a decorator that does not exist"),
    (DecoratorArgType, DecoratorArgTypeError, "An error caused by calling a decorator using an argument of the wrong type"),

    (FunctionArgType, FunctionArgTypeError, "An error caused by calling a function using an argument of the wrong type"),
    (FunctionNArgs, FunctionNArgsError, "An error caused by calling a function using the wrong number of arguments"),
    (FunctionName, FunctionNameError, "An error caused by calling a function that does not exist"),
    (FunctionOverflow, FunctionOverflowError, "An error caused by a function argument overflowing a pre-determined limit"),
    (Stack, StackError, "An error caused by a recursive function going too deep"),
    
    // Value errors
    (ConstantValue, ConstantValueError, "An error caused by attempting to overwrite a constant"),
    (ObjectKey, ObjectKeyError, "An error caused by attempting to use an invalid object key"),
    (Overflow, OverflowError, "An error caused by a calculation that resulted in an overflow"),
    (ParseValue, ParseValueError, "An error caused by attempting to parse an value"),
    (Parsing, ParsingError, "An error caused by attempting to parse an invalid string into a given format"),
    (Range, RangeError, "An error caused by attempting use an out of range value"),
    (Underflow, UnderflowError, "An error caused by a calculation that resulted in an underflow"),
    (ValueType, ValueTypeError, "An error caused by attempting to use a value of the wrong type in a calculation"),
    (VariableName, VariableNameError, "An error caused by attempting to use an unassigned variable"),
    
    // Downstream errors
    (IO, IOError, "An error caused by filesystem issues"),
    (Network, NetworkError, "An error caused by network issues"),
    (Pest, PestError, "An error caused by a problem in parsing the syntax of an expression"),
    (Script, ScriptError, "An error caused by an unknown exception in a javascript extension"),

    // Syntax errors
    (UnexpectedDecorator, UnexpectedDecoratorError, "An error caused by using a decorator in the wrong place"),
    (UnexpectedPostfix, UnexpectedPostfixError, "An error caused by using a postfix operator without an operand"),
    (UnterminatedArray, UnterminatedArrayError, "An error caused by a missing square bracket"),
    (UnterminatedLinebreak, UnterminatedLinebreakError, "An error caused by ending a script on a backslash"),
    (UnterminatedLiteral, UnterminatedLiteralError, "An error caused by a missing quote"),
    (UnterminatedObject, UnterminatedObjectError, "An error caused by a missing curly brace"),
    (UnterminatedParen,UnterminatedParenError, "An error caused by a missing parentheses"),
    
    (Internal, InternalError, "An error caused by a problem with the parser itself")
);