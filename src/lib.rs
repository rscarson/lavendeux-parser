//! lavendeux-parser  is  an  exensible  parsing  engine  for  mathematical  expressions.
//! It  supports  variable  and  function  assignments, a  variety  of  datatypes, and  can
//! be  extended  easily  at  runtime  through  extensions  written  in  javascript.
//! 
//! Extensions  are  run  in  a  sandboxed  environment  with  no  host  or  network  access.
//! This  project  is  the  engine  behind [Lavendeux](https://rscarson.github.io/lavendeux/).
//! 
//! For help on the syntax of expressions, visit <https://rscarson.github.io/lavendeux>
//! 
//! ## Getting  Started
//! To  use it, create a `ParserState` object, and use it to tokenize input with `Token::new`:
//! ```rust
//! use lavendeux_parser::{ParserState, ParserError, Token, Value};
//! 
//! fn main() -> Result<(), ParserError> {
//!     // Create a new parser, and tokenize 2 lines
//!     let mut state : ParserState = ParserState::new();
//!     let lines = Token::new("x=9\nsqrt(x) @bin", &mut state)?;
//! 
//!     // The resulting token contains the resulting values and text
//!     assert_eq!(lines.text(), "9\n0b11");
//!     assert_eq!(lines.child(1).unwrap().value(), Value::Float(3.0));
//!     
//!     Ok(())
//! }
//! ```
//! The result will be a `Token` object:
//! ```rust
//! use lavendeux_parser::{ParserState, ParserError, Token, Value};
//! 
//! fn main() -> Result<(), ParserError> {
//!     let mut state : ParserState = ParserState::new();
//!     let lines = Token::new("x=9\nsqrt(x) @bin", &mut state)?;
//! 
//!     // String representation of the full result
//!     assert_eq!(lines.text(), "9\n0b11"); 
//! 
//!     // String representation of the first line's result
//!     assert_eq!(lines.child(0).unwrap().text(), "9");
//! 
//!     // Actual value of the first line's result
//!     // Values are integers, floats, booleans or strings
//!     let value = lines.child(0).unwrap().value();
//!     assert_eq!(value.as_int().unwrap(), 9);
//!     assert_eq!(true, matches!(value, Value::Integer(_)));
//! 
//!     Ok(())
//! }
//! ```
//! 
//! A number of functions and @decorators are available for expressions to use - add more using the state:
//! ```rust
//! use lavendeux_parser::{ParserState, ParserError, DecoratorDefinition, FunctionDefinition, FunctionArgument, Value};
//! use lavendeux_parser::errors::*;
//!  
//! let mut state : ParserState = ParserState::new();
//! state.decorators.register(DecoratorDefinition {
//!     name: &["upper", "uppercase"],
//!     description: "Outputs an uppercase version of the input",
//!     argument: ExpectedTypes::Any,
//!     handler: |_, input| Ok(input.as_string().to_uppercase())
//! });
//! 
//! // Functions take in an array of values, and return a single value
//! state.functions.register(FunctionDefinition {
//!     name: "echo",
//!     description: "Echo back the provided input",
//!     arguments: || vec![
//!         FunctionArgument::new_required("input", ExpectedTypes::String),
//!     ],
//!     handler: |_, args: &[Value]| {
//!         Ok(Value::String(args[0].as_string()))
//!     }
//! });
//! 
//! // Expressions being parsed can now call new_function(), and use the @new_decorator
//! ```
//! 
//! ## Using Extensions
//! Extensions give a more flexible way of adding functionality at runtime. Extensions are written in javascript.
//! 
//! Extensions are enabled by default, and can be excluded by disabling the crate's "extensions" feature
//! 
//! Extensions can be loaded as follows:
//! ```rust
//! use lavendeux_parser::{ParserState, ParserError, Value, Token};
//! 
//! fn main() -> Result<(), ParserError> {
//!     let mut state : ParserState = ParserState::new();
//! 
//!     // Load one extension
//!     state.extensions.load("example_extensions/colour_utils.js")?;
//! 
//!     // Load a whole directory
//!     state.extensions.load_all("./example_extensions")?;
//! 
//!     // Once loaded, functions and @decorators decribed in the extensions
//!     // can be called in expressions being parsed
//!     let token = Token::new("complement(0xFF0000) @colour", &mut state)?;
//!     assert_eq!(token.text(), "#ffff00");
//!     Ok(())
//! }
//! ```
//! Extensions give a more flexible way of adding functionality at runtime. Extensions are written in javascript.
#![doc(html_root_url = "https://docs.rs/lavendeux-parser/0.7.0")]
#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

mod handlers;
mod token;
mod value;
mod state;

mod functions;
pub use functions::{FunctionTable, FunctionDefinition, FunctionArgument};

mod decorators;
pub use decorators::{DecoratorTable, DecoratorDefinition};

#[cfg(feature = "extensions")]
mod extensions;

#[cfg(feature = "extensions")]
pub use extensions::Extension;

/// Module defining errors that can occur during parsing
pub mod errors;
pub use errors::ParserError;
pub use token::Token;
pub use state::ParserState;
pub use value::Value;
pub use value::IntegerType;
pub use value::FloatType;

#[cfg(test)]
mod test_token {
    #[test]
    fn test_readme_deps() {
        version_sync::assert_markdown_deps_updated!("README.md");
    }

    #[test]
    fn test_html_root_url() {
        version_sync::assert_html_root_url_updated!("src/lib.rs");
    }
}