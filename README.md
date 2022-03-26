
# Lavendeux Parser - Extensible inline parser engine
[![Crates.io](https://img.shields.io/crates/v/lavendeux-parser.svg)](https://crates.io/crates/lavendeux-parser)
[![Build Status](https://github.com/rscarson/lavendeux-parser/workflows/CI/badge.svg)](https://github.com/rscarson/lavendeux-parser/actions?workflow=Rust)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/rscarson/lavendeux-parser/master/LICENSE)

lavendeux-parser  is  an  exensible  parsing  engine  for  mathematical  expressions.
It  supports  variable  and  function  assignments, a  variety  of  datatypes, and  can
be  extended  easily  at  runtime  through  extensions  written  in  javascript.

Extensions  are  run  in  a  sandboxed  environment  with  no  host  or  network  access.
This  project  is  the  engine  behind [Lavendeux](https://rscarson.github.io/lavendeux/).

## Getting  Started
To  use it, create a `ParserState` object, and use it to tokenize input with `Token::new`:
```rust
use lavendeux_parser::{ParserState, ParserError, Token, Value};
 
fn main() -> Result<(), ParserError> {
    // Create a new parser, and tokenize 2 lines
    let mut state : ParserState = ParserState::new();
    let lines = Token::new("x=9\nsqrt(x) @bin", &mut state)?;
 
    // The resulting token contains the resulting values and text
    assert_eq!(lines.text(), "9\n0b11");
    assert_eq!(lines.child(1).unwrap().value(), Value::Integer(3));
     
    Ok(())
}
```
The result will be a `Token` object:
```rust
use lavendeux_parser::{ParserState, ParserError, Token, Value};
 
fn main() -> Result<(), ParserError> {
    let mut state : ParserState = ParserState::new();
    let lines = Token::new("x=9\nsqrt(x) @bin", &mut state)?;
 
    // String representation of the full result
    assert_eq!(lines.text(), "9\n0b11"); 
 
    // String representation of the first line's result
    assert_eq!(lines.child(0).unwrap().text(), "9");
 
    // Actual value of the first line's result
    // Values are integers, floats, booleans or strings
    let value = lines.child(0).unwrap().value();
    assert_eq!(value.as_int().unwrap(), 9);
    assert_eq!(true, matches!(value, Value::Integer(_)));
 
    Ok(())
}
```

A number of functions and @decorators are available for expressions to use - add more using the state:
```rust
use lavendeux_parser::{ParserState, ParserError, Value};
 
// Functions take in an array of values, and return a single value
fn new_function_handler(args: &[Value]) -> Result<Value, ParserError> {
    Ok(Value::Integer(0))
}
 
// Decorators take in a single value, and return a string representation
fn new_decorator_handler(arg: &Value) -> Result<String, ParserError> {
    Ok(arg.as_string())
}
 
let mut state : ParserState = ParserState::new();
state.decorators.register("new_decorator", new_decorator_handler);
state.functions.register("new_function", new_function_handler);
 
// Expressions being parsed can now call new_function(), and use the @new_decorator
```rust
use lavendeux_parser::{ParserState, ParserError, Value, Token};
 
fn main() -> Result<(), ParserError> {
    let mut state : ParserState = ParserState::new();
 
    // Load one extension
    state.extensions.load("example_extensions/colour_utils.js")?;
 
    // Load a whole directory
    state.extensions.load_all("./example_extensions")?;
 
    // Once loaded, functions and @decorators decribed in the extensions
    // can be called in expressions being parsed
    let token = Token::new("complement(0xFF0000) @colour", &mut state)?;
    assert_eq!(token.text(), "#ffff00");
    Ok(())
}
```
Extensions give a more flexible way of adding functionality at runtime. Extensions are written in javascript.

## Syntax
Expressions can be composed of integers, floats, strings, as well as numbers of various bases:
```javascript
// Integer, floating point or scientific notation numbers
5 + 5.56 + .2e+3

// Currency values
// Note that no exchange rate is being applied automatically
$1,000.00 == ¥1,000.00

// Scientific numbers can be represented a number of ways
5.6e+7 - .6E7 + .2e-3

// Booleans
in_range = 5 > 3 && 5 < 10
true || false

// Integers can also be represented in base 2, 8 or 16
0xFFA & 0b110 & 0777

// Strings are also supported
concat("foo", "bar")
```

Beyond the simpler operators, the following operations are supported:
```javascript
5 ** 2 // Exponentiation
6 % 2 // Modulo
3! // Factorial

// Bitwise operators AND, OR, and XOR:
0xF & 0xA | 0x2 ^ 0xF

// Bitwise SHIFT, and NOT
0xF << 1
0x1 >> 2
~0xA

// Boolean operators
true || false && true
1 < 2 > 5 // true
```

You can also assign values to variables to be used later:  
They are case sensitive, and can be composed of underscores or alphanumeric characters
```javascript
// You can also assign values to variables to be used later
x = 0xFFA & 0xFF0
x - 55 // The result will be 200

// A few constants are also pre-defined
value = pi * e * tau

// You can also define functions
f(x) = 2*x**2 + 3*x + 5
f(2.3)

// Recursive functions work too!
factorial(x) = x==1 ? x : (x * factorial(x - 1) )
factorial(5)
```

Decorators can be put at the end of a line to change the output format. Valid decorators include:  
**@bin, @oct, @hex, @int, @float, or @sci**
```javascript
255 @hex // The result will be 0xFF
8 @oct // The result will be 0o10
5 @float // The result will be 5.0
5 @usd // Also works with @dollars @cad, @aud, @yen, @pounds, or @euros
1647950086 @utc // 2022-03-22 11:54:46
```

The following functions are supported by default:
```javascript
// String functions
concat("s1", "s2", ...) | strlen("string") | substr("string", start, [length])

// Rounding functions
ceil(n) | floor(n) | round(n, precision)

// Trigonometric functions
tan(r), cos(r), sin(r), atan(r), acos(r), asin(r), tanh(r), cosh(r), sinh(r)

// Rounding functions
ln(n) | log10(n) | log(n, base)
sqrt(n) | root(n, base)

// RNG functions
choose("argument 1", 2, 3.0, ...) | rand() | rand(min, max)

// Networking functions
get(url, ["header-name=value", ...]) | post(url, ["header-name=value", ...]) | resolve(hostname)

// Misc. functions
to_radians(degree_value) | abs(n) | tail(filename, [lines]) | time()
```

Lavendeux can be extended with javascript. Extensions are run in a sandboxed environment, with no network or host access.  
Below is an example of a simple extension:
```javascript
/**
* This function tells Lavendeux about this extension.
* It must return an object similar to the one below.
* @returns Object
*/
function extension() }
    return {
        name: "Extension Name",
        author: "Author's name",
        version: "0.0.0",
        
        functions: {,
            "callable_name": "js_function_name"
        },
        
        decorator: {,
            "callable_name": "js_decorator_name"
        },
    }
}

/**
* This function can be called from Lavendeux as callable_name(...)
* args is an array of value objects with either the key Integer, Float or String
* It must also return an object of that kind, or throw an exception
* @returns Object
*/
function js_function_name(args) }
    return {
        "Integer": 5,
    };
}

/**
* This decorator can be called from Lavendeux as @callable_name
* arg is a value object with either the key Integer, Float or String
* It must return a string, or throw an exception
* @returns String
*/
function js_decorator_name(arg) {
    return "formatted value";
}
```
