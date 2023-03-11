//! # Lavendeux Parser - Extensible inline parser engine
//! [![Crates.io](https://img.shields.io/crates/v/lavendeux-parser.svg)](https://crates.io/crates/lavendeux-parser)
//! [![Build Status](https://github.com/rscarson/lavendeux-parser/workflows/Rust/badge.svg)](https://github.com/rscarson/lavendeux-parser/actions?workflow=Rust)
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/rscarson/lavendeux-parser/master/LICENSE)
//! 
//! lavendeux-parser is an exensible parsing engine for mathematical expressions.
//! It supports variable and function assignments, a variety of datatypes, and can
//! be extended easily at runtime through extensions written in javascript.
//! 
//! Extensions are run in a sandboxed environment with no host or network access.
//! This project is the engine behind [Lavendeux](https://rscarson.github.io/lavendeux/).
//! 
//! ## Getting Started
//! To use it, create a `ParserState` object, and use it to tokenize input with `Token::new`:
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
//!     category: None,
//!     description: "Echo back the provided input",
//!     arguments: || vec![
//!         FunctionArgument::new_required("input", ExpectedTypes::String),
//!     ],
//!     handler: |_function, _state, args| {
//!         Ok(Value::String(args.get("input").required().as_string()))
//!     }
//! });
//! 
//! // Expressions being parsed can now call new_function(), and use the @new_decorator
//! ```
//! 
//! Javascript extensions give a flexible way of adding functionality at runtime.
//! Extensions are run in a sandboxed environment, with no network or host access.  
//! An extension must implement an extension() function taking no arguments and returning an object describing the extension - see example below
//!
//! ```javascript
//! /**
//! * This function tells Lavendeux about this extension.
//! * It must return an object similar to the one below.
//! * @returns Object
//! */
//! function extension() }
//!     return {
//!         name: "Extension Name",
//!         author: "Author's name",
//!         version: "0.0.0",
//!         
//!         functions: {,
//!             "callable_name": "js_function_name"
//!         },
//!         
//!         decorators: {,
//!             "callable_name": "js_decorator_name"
//!         },
//!     }
//! }
//! 
//! /**
//! * This function can be called from Lavendeux as callable_name(...)
//! * args is an array of value objects with either the key Integer, Float or String
//! * It must also return an object of that kind, or throw an exception
//! * @returns Object
//! */
//! function js_function_name(args) }
//!     return {
//!         "Integer": 5,
//!     };
//! }
//! 
//! /**
//! * This decorator can be called from Lavendeux as @callable_name
//! * arg is a value object with either the key Integer, Float or String
//! * It must return a string, or throw an exception
//! * @returns String
//! */
//! function js_decorator_name(arg) {
//!     return "formatted value";
//! }
//! ```
//! 
//! ## Using Extensions
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
//! 
//! ## Syntax
//! Expressions can be composed of integers, floats, strings, as well as numbers of various bases:
//! ```text
//! // Integer, floating point or scientific notation numbers
//! 5 + 5.56 + .2e+3
//! 
//! // Currency values
//! // Note that no exchange rate is being applied automatically
//! $1,000.00 == ¥1,000.00
//! 
//! // Scientific numbers can be represented a number of ways
//! 5.6e+7 - .6E7 + .2e-3
//! 
//! // Booleans
//! in_range = 5 > 3 && 5 < 10
//! true || false
//! 
//! // Integers can also be represented in base 2, 8 or 16
//! 0xFFA & 0b110 & 0777
//! 
//! // Strings are also supported
//! concat("foo", "bar")
//! 
//! // Arrays can be composed of any combination of types
//! [10, 12] + [1.2, 1.3]
//! 2 * [10, 5] // Operations can also be applied between scalar values and arrays
//! [false, 0, true] == true // An array evaluates to true if any element is true
//! ```
//! 
//! Beyond the simpler operators, the following operations are supported:
//! ```text
//! 5 ** 2 // Exponentiation
//! 6 % 2 // Modulo
//! 3! // Factorial
//! 
//! // Bitwise operators AND, OR, and XOR:
//! 0xF & 0xA | 0x2 ^ 0xF
//! 
//! // Bitwise SHIFT, and NOT
//! 0xF << 1
//! 0x1 >> 2
//! ~0xA
//! 
//! // Boolean operators
//! true || false && true
//! 1 < 2 > 5 // true
//! ```
//! 
//! You can also assign values to variables to be used later:  
//! They are case sensitive, and can be composed of underscores or alphanumeric characters
//! ```text
//! // You can also assign values to variables to be used later
//! x = 0xFFA & 0xFF0
//! x - 55 // The result will be 200
//! 
//! // A few constants are also pre-defined
//! value = pi * e * tau
//! 
//! // You can also define functions
//! f(x) = 2*x**2 + 3*x + 5
//! f(2.3)
//! 
//! // Functions work well with arrays
//! sum(a) = element(a, 0) + ( len(a)>1 ? sum(dequeue(a)) : 0 )
//! sum([10, 10, 11])
//! 
//! // Recursive functions work too!
//! factorial(x) = x==0 ? 1 : (x * factorial(x - 1) )
//! factorial(5)
//! ```
//! 
//! Decorators can be put at the end of a line to change the output format. Valid decorators include:
//! ```text
//! 255 @hex // The result will be 0xFF
//! 8 @oct // The result will be 0o10
//! 5 @float // The result will be 5.0
//! 5 @usd // Also works with @dollars @cad, @aud, @yen, @pounds, or @euros
//! 1647950086 @utc // 2022-03-22 11:54:46
//! ```
//! 
//! The following functions are supported by default:
//! ```text
//!     Math Functions
//!     ===============
//!     abs(n): Returns the absolute value of n
//!     acos(n): Calculate the arccosine of n
//!     array(n): Returns a value as an array
//!     asin(n): Calculate the arcsine of n
//!     atan(n): Calculate the arctangent of n
//!     bool(n): Returns a value as a boolean
//!     ceil(n): Returns the nearest whole integer larger than n
//!     cos(n): Calculate the cosine of n
//!     cosh(n): Calculate the hyperbolic cosine of n
//!     float(n): Returns a value as a float
//!     floor(n): Returns the nearest whole integer smaller than n
//!     int(n): Returns a value as an integer
//!     ln(n): Returns the natural log of n
//!     log(n, base): Returns the logarithm of n in any base
//!     log10(n): Returns the base 10 log of n
//!     max(n1, n2): Returns the largest numeric value from the supplied arguments
//!     min(n1, n2): Returns the smallest numeric value from the supplied arguments
//!     root(n, base): Returns a root of n of any base
//!     round(n, [precision]): Returns n, rounded to [precision] decimal places
//!     sin(n): Calculate the sine of n
//!     sinh(n): Calculate the hyperbolic sine of n
//!     sqrt(n): Returns the square root of n
//!     tan(n): Calculate the tangent of n
//!     tanh(n): Calculate the hyperbolic tangent of n
//!     to_degrees(n): Convert the given radian value into degrees
//!     to_radians(n): Convert the given degree value into radians
//!     
//!     Misc Functions
//!     ===============
//!     call(filename): Run the contents of a file as a script
//!     help([function_name]): Display a help message
//!     run(expression): Run a string as an expression
//!     tail(filename, [lines]): Returns the last [lines] lines from a given file
//!     time(): Returns a unix timestamp for the current system time
//!     
//!     Network Functions
//!     ===============
//!     api(name, [endpoint]): Make a call to a registered API
//!     api_delete(name): Remove a registered API from the list
//!     api_list(): List all registered APIs
//!     api_register(name, base_url, [api_key]): Register a new API for quick usage
//!     get(url, [headers1, headers2]): Return the resulting text-format body of an HTTP GET call
//!     post(url, body, [header-name=value1, header-name=value2]): Return the resulting text-format body of an HTTP POST call
//!     resolve(hostname): Returns the IP address associated to a given hostname
//!     
//!     Cryptography Functions
//!     ===============
//!     choose(option1, option2): Returns any one of the provided arguments at random
//!     md5(input1, input2): Returns the MD5 hash of a given string
//!     rand([m], [n]): With no arguments, return a float from 0 to 1. Otherwise return an integer from 0 to m, or m to n
//!     sha256(input1, input2): Returns the SHA256 hash of a given string
//!     
//!     Arrays Functions
//!     ===============
//!     dequeue(array): Remove the first element from an array
//!     element(array, index): Return an element from a location in an array
//!     enqueue(array, element): Add an element to the end of an array
//!     is_empty(array): Returns true if the given array is empty
//!     len(array): Returns the length of the given array
//!     merge(arrays1, arrays2): Merge all given arrays
//!     pop(array): Remove the last element from an array
//!     push(array, element): Add an element to the end of an array
//!     remove(array, index): Removes an element from an array
//!     
//!     Strings Functions
//!     ===============
//!     concat([s1, s2]): Concatenate a set of strings
//!     contains(source, s): Returns true if array or string [source] contains [s]
//!     lowercase(s): Converts the string s to lowercase
//!     regex(pattern, subject, [group]): Returns a regular expression match from [subject], or false
//!     strlen(s): Returns the length of the string s
//!     substr(s, start, [length]): Returns a substring from s, beginning at [start], and going to the end, or for [length] characters
//!     trim(s): Trim whitespace from a string
//!     uppercase(s): Converts the string s to uppercase
//!     
//!     Built-in Decorators
//!     ===============
//!     @array: Format a number as an array
//!     @aud: Format a number as a dollar amount
//!     @bin: Base 2 number formatting, such as 0b11
//!     @bool: Format a number as a boolean
//!     @boolean: Format a number as a boolean
//!     @cad: Format a number as a dollar amount
//!     @default: Default formatter, type dependent
//!     @dollar: Format a number as a dollar amount
//!     @dollars: Format a number as a dollar amount
//!     @euro: Format a number as a euro amount
//!     @euros: Format a number as a euro amount
//!     @float: Format a number as floating point
//!     @hex: Base 16 number formatting, such as 0xFF
//!     @int: Format a number as an integer
//!     @integer: Format a number as an integer
//!     @oct: Base 8 number formatting, such as 0b77
//!     @pound: Format a number as a pound amount
//!     @pounds: Format a number as a pound amount
//!     @sci: Scientific number formatting, such as 1.2Ee-3
//!     @usd: Format a number as a dollar amount
//!     @utc: Interprets an integer as a timestamp, and formats it in UTC standard
//!     @yen: Format a number as a yen amount
//! ```
//!
#![doc(html_root_url = "https://docs.rs/lavendeux-parser/0.8.0")]
#![warn(missing_docs)]

mod handlers;
mod token;
mod value;
mod state;

mod network;
pub use network::*;

mod functions;
pub use functions::*;

mod decorators;
pub use decorators::*;

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