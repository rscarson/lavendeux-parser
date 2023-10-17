use crate::{ExpectedTypes, Token, Value};
use thiserror::Error;

const BUG_REPORT_URL : &str = "https://github.com/rscarson/lavendeux-parser/issues/new?assignees=&labels=&template=bug_report.md&title=";

/// Represents the errors that can occur during parsing
#[derive(Error, Debug)]
#[rustfmt::skip]
pub enum Error {
    /// An error caused by a problem with the parser itself
    #[error(
        "internal parser issue at {0}.\nPlease report this problem at {}",
        BUG_REPORT_URL
    )]
    Internal(Token),

    ///////////////////////////////////////////////////////////////////////////
    // Value Errors
    // Mostly deals with variables, and value objects
    ///////////////////////////////////////////////////////////////////////////

    /// An error caused by attempting to overwrite a constant
    #[error("could not overwrite constant value {name} at {token}")]
    ConstantValue {
        /// Name of the constant
        name: String,
        
        /// token at which the error occured
        token: Token
    },

    /// An error caused by a calculation that resulted in an overflow
    #[error("arithmetic overflow at {0}")]
    Overflow(Token),

    /// An error caused by a calculation that resulted in an underflow
    #[error("arithmetic underflow at {0}")]
    Underflow(Token),

    /// An error caused by attempting to parse an value
    #[error("{input} could not be parsed as {expected_type} at {token}")]
    ValueParsing {
        /// Value causing the error
        input: String,
        
        /// Type that was requested
        expected_type: ExpectedTypes,
        
        /// token at which the error occured
        token: Token,
    },

    /// An error caused by attempting to parse an invalid string into a given format
    #[error("string could not be parsed as {expected_format} at (Token)")]
    StringFormat {
        /// Expected format of the string
        expected_format: String,
        
        /// token at which the error occured
        token: Token,
    },

    /// An error caused by attempting use an out of range value
    #[error("value {value} was out of range at {token}")]
    Range {
        /// Value causing the error
        value: Value,
        
        /// token at which the error occured
        token: Token
    },

    /// An error caused by attempting to use a value of the wrong type in a calculation
    #[error("wrong type of value {value} expected {expected_type} at {token}")]
    ValueType {
        /// Value causing the error
        value: Value,
        
        /// Type that was requested
        expected_type: ExpectedTypes,
        
        /// token at which the error occured
        token: Token,
    },

    /// An error caused by attempting to use an unassigned variable
    #[error("undefined variable {name} at {token}")]
    VariableName {
        /// Name of the variable
        name: String,
        
        /// token at which the error occured
        token: Token
    },

    ///////////////////////////////////////////////////////////////////////////
    // Syntax Errors
    // Deals with issues during Pest tree parsing
    ///////////////////////////////////////////////////////////////////////////

    /// An error caused by using a decorator in the wrong place
    #[error("{0} must be at the end of a statement")]
    UnexpectedDecorator(Token),

    /// An error caused by using a postfix operator without an operand
    #[error("missing operand before postfix operator at {0}")]
    UnexpectedPostfix(Token),

    /// An error caused by a missing bracket
    #[error("expected ']' at {0}")]
    UnterminatedArray(Token),

    /// An error caused by a missing brace
    #[error("expected '}}' at {0}")]
    UnterminatedObject(Token),

    /// An error caused by ending a script on a backslash
    #[error("missing linebreak after '\\' at {0}")]
    UnterminatedLinebreak(Token),

    /// An error caused by a missing quote
    #[error("expected ' or \" at {0}")]
    UnterminatedLiteral(Token),

    /// An error caused by a missing parentheses
    #[error("expected ')' at {0}")]
    UnterminatedParen(Token),

    ///////////////////////////////////////////////////////////////////////////
    // Function Errors
    // Deals with issues during builtin, user, or extension function calls
    ///////////////////////////////////////////////////////////////////////////

    /// An error caused by a recursive function going too deep
    #[error("stack overflow at {0}")]
    StackOverflow(Token),

    /// An error caused by attempting to use a function with ambiguous arguments
    #[error("function parameters for {signature} are ambiguous at {token}")]
    AmbiguousFunctionDefinition {
        /// Signature of the function called
        signature: String,
        
        /// token at which the error occured
        token: Token
    },

    /// An error caused by calling a function with an argument of the wrong type
    #[error("argument {arg} of {signature}, expected {expected_type} at {token}")]
    FunctionArgumentType {
        /// Argument number causing the issue (1-based)
        arg: usize,

        /// Type that was requested
        expected_type: ExpectedTypes,
        
        /// Signature of the function called
        signature: String,
        
        /// token at which the error occured
        token: Token,
    },

    /// An error caused by calling a function that does not exist
    #[error("no such function {name} at {token}")]
    FunctionName {
        /// Name of the function
        name: String,
        
        /// token at which the error occured
        token: Token
    },

    /// An error caused by calling a function using the wrong number of arguments
    #[error(
        "{signature} expected {} arguments at {token}",
        if min == max {format!("{}", min)} else {format!("{}-{}", min, max)}
    )]
    FunctionArguments {
        /// Smallest number of arguments accepted by the function
        min: usize,
        
        /// Largest number of arguments accepted by the function
        max: usize, 
        
        
        /// Signature of the function called
        signature: String,
        
        /// token at which the error occured
        token: Token
    },

    /// An error caused by a function argument overflowing a pre-determined limit
    #[error("argument {arg} of {signature} at {token}")]
    FunctionArgumentOverflow {
        /// Argument number causing the issue (1-based)
        arg: usize,
        
        /// Signature of the function called
        signature: String,
        
        /// token at which the error occured
        token: Token
    },

    /// An error caused by calling a decorator with an argument of the wrong type
    #[error("@{name} expected type {expected_type} at {token}")]
    DecoratorArgumentType {
        /// Type that was requested
        expected_type: ExpectedTypes,

        /// Name of the decorator
        name: String,
        
        /// token at which the error occured
        token: Token,
    },

    /// An error caused by calling a decorator that does not exist
    #[error("no such decorator {name} at {token}")]
    DecoratorName {
        /// Name of the decorator
        name: String,
        
        /// token at which the error occured
        token: Token
    },
    
    /// An error caused by attempting to use an API without registering it
    #[error("API {name} was not found. Add it with api_register(\"{name}\", base_url, [optional api key]) at {token}")]
    UnknownApi {
        /// Name of the API
        name: String,
        
        /// token at which the error occured
        token: Token
    },

    ///////////////////////////////////////////////////////////////////////////
    // Array Errors
    // Deals with issues indexing of arrays and objects
    ///////////////////////////////////////////////////////////////////////////

    /// An error caused by attempting to use an invalid object or array key
    #[error("undefined index {key} at {token}")]
    Index {
        /// Index that caused the error
        key: Value,
        
        /// token at which the error occured
        token: Token
    },

    /// An error caused by attempting to index on an empty array
    #[error("could not index empty array at {0}")]
    ArrayEmpty(Token),

    /// An error caused by attempting to operate on a pair of arrays of incompatible lengths
    #[error("array lengths were incompatible at {0}")]
    ArrayLengths(Token),

    ///////////////////////////////////////////////////////////////////////////
    // External Errors
    // Deals with issues inside dependencies
    ///////////////////////////////////////////////////////////////////////////
 
    /// Error dealing with filesystem issues
    #[error("{0} at {1}")]
    Io(std::io::Error, Token),

    /// Error dealing with network issues from the reqwest crate
    #[error("{0} at {1}")]
    Network(reqwest::Error, Token),

    /// Error dealing with pest parsing problems
    #[error("{0} at {1}")]
    Pest(pest::error::Error<crate::token::Rule>, Token),

    /// Error dealing with JS execution issues
    #[error("{0} at {1}")]
    Javascript(rustyscript::Error, Token),
}
