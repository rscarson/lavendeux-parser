# lavendeux-parser

## Extensible inline parser engine
[![Crates.io](https://img.shields.io/crates/v/lavendeux-parser.svg)](https://crates.io/crates/lavendeux-parser)
[![Build Status](https://github.com/rscarson/lavendeux-parser/workflows/Rust/badge.svg)](https://github.com/rscarson/lavendeux-parser/actions?workflow=Rust)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/rscarson/lavendeux-parser/master/LICENSE)

lavendeux-parser is rust library that provides an extensible parsing engine for mathematical expressions.
It enables the parsing of user-supplied expressions to operate on a variety of types of data.
It supports variable and function assignments, a variety of datatypes, and can
be extended easily at runtime through extensions written in javascript.

Extensions are run in a sandboxed environment with no host or network access.
This project is the engine behind [Lavendeux](https://rscarson.github.io/lavendeux/).

### Getting Started
To use it, create a `ParserState` object, and use it to tokenize input with `Token::new`:
```rust
use lavendeux_parser::{ParserState, Error, Token, Value};

fn main() -> Result<(), Error> {
    // Create a new parser, and tokenize 2 lines
    let mut state : ParserState = ParserState::new();
    let lines = Token::new("x=9\nsqrt(x) @bin", &mut state)?;

    // The resulting token contains the resulting values and text
    assert_eq!(lines.text(), "9\n0b11");
    assert_eq!(lines.child(1).unwrap().value(), Value::Float(3.0));

    Ok(())
}
```
The result will be a `Token` object:
```rust
use lavendeux_parser::{ParserState, Error, Token, Value};

fn main() -> Result<(), Error> {
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
use lavendeux_parser::{ParserState, Error, DecoratorDefinition, FunctionDefinition, FunctionArgument, Value, ExpectedTypes};

let mut state : ParserState = ParserState::new();
state.decorators.register(DecoratorDefinition {
    name: &["upper", "uppercase"],
    description: "Outputs an uppercase version of the input",
    argument: ExpectedTypes::Any,
    handler: |_, _token, input| Ok(input.as_string().to_uppercase())
});

// Functions take in an array of values, and return a single value
state.functions.register(FunctionDefinition {
    name: "echo",
    category: None,
    description: "Echo back the provided input",
    arguments: || vec![
        FunctionArgument::new_required("input", ExpectedTypes::String),
    ],
    handler: |_function, _token, _state, args| {
        Ok(Value::String(args.get("input").required().as_string()))
    }
});

// Expressions being parsed can now call new_function(), and use the @new_decorator
```

Javascript extensions give a flexible way of adding functionality at runtime.
Extensions are run in a sandboxed environment, with no network or host access.
An extension must implement an extension() function taking no arguments and returning an object describing the extension - see example below

Extensions can also access parser variables through getState, and mutate the state with setState
Always check if getState is defined prior to use, to maintain compatibility with older versions of the parser.

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
            "callable_name": "js_function_name",
            "stateful_function": "js_stateful_fn"
        },

        decorators: {,
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
* Functions can also be stateful, gaining access to the parser's variables
* It takes in arguments and a state, a hash of strings and values
* @returns a single value, or a [value, state] pair to mutate the parser state
*/
function js_stateful_fn(args, state) }
    state.foobar = {"Integer": 5};
    return [state.foobar, state];
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

### Using Extensions
Extensions are enabled by default, and can be excluded by disabling the crate's "extensions" feature

Extensions can be loaded as follows:
```rust
use lavendeux_parser::{ParserState, Error, Value, Token};

fn main() -> Result<(), Error> {
    let mut state : ParserState = ParserState::new();

    // Load one extension
    state.extensions.load("example_extensions/simple_extension.js");

    // Load a whole directory - this will return a vec of Result<Extension, Error>
    state.extensions.load_all("./example_extensions");

    // Once loaded, functions and @decorators decribed in the extensions
    // can be called in expressions being parsed
    let token = Token::new("add(1, 2) @colour", &mut state)?;
    assert_eq!(token.text(), "#300000");
    Ok(())
}
```

### Syntax
Expressions can be composed of integers, floats, strings, as well as numbers of various bases:
```
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

[1, 2, "test"] // Arrays can be composed of any combination of types
[10, 12] + [1.2, 1.3] // Operations can be performed between arrays of the same size
2 * [10, 5] // Operations can also be applied between scalar values and arrays
[false, 0, true] == true // An array evaluates to true if any element is true
a = [1, 2, "test"]
a[1] // You can use indexing on array elements

// Objects are also supported:
b = {3: "test", "plumbus": true}
b["plumbus"]
```

Beyond the simpler operators, the following operations are supported:
```
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
```
// You can also assign values to variables to be used later
x = 0xFFA & 0xFF0
x - 55 // The result will be 200

// A few constants are also pre-defined
value = pi * e * tau

// You can also define functions
f(x) = 2*x**2 + 3*x + 5
f(2.3)

// Functions work well with arrays
sum(a) = element(a, 0) + ( len(a)>1 ? sum(dequeue(a)) : 0 )
sum([10, 10, 11])

// Recursive functions work too!
factorial(x) = x==0 ? 1 : (x * factorial(x - 1) )
factorial(5)
```

Decorators can be put at the end of a line to change the output format. Valid decorators include:
```
255 @hex // The result will be 0xFF
8 @oct // The result will be 0o10
5 @float // The result will be 5.0
5 @usd // Also works with @dollars @cad, @aud, @yen, @pounds, or @euros
1647950086 @utc // 2022-03-22 11:54:46
```

Full list of built-in types, operators and functions:
```
Operators
=========
Bitwise: AND (0xF & 0xA), OR (0xA | 0xF), XOR (0xA ^ 0xF), NOT (~0xA), SHIFT (0xF >> 1, 0xA << 1)
Boolean: AND (true && false), OR (true || false), CMP (1 < 2, 4 >= 5), EQ (1 == 1, 2 != 5)
Arithmetic: Add/Sub (+, -), Mul/Div (*, /), Exponentiation (**), Modulo (%), Implied Mul ((5)(5), 5x)
Unary: Factorial (5!!), Negation (-1, -(1+1))

Data Types
==========
String: Text delimited by 'quotes' or "double-quotes"
Boolean: A truth value (true or false)
Integer: A whole number. Can also be base2 (0b111), base8 (0o777), or base16 (0xFF)
Float: A decimal number. Can also be in scientific notation(5.3e+4, 4E-2)
Currency: A decimal number - does not apply any exhange rates ($5.00)
Array: A comma separated list of values in square brackets; [1, 'test']
Object: A comma separated list of key/value pairs in curly braces; {'test': 5}
Variable: An identifier representing a value. Set it with x=5, then use it in an expression (5x)
Contant: A preset read-only variable representing a common value, such as pi, e, and tau

Misc Functions
==============
atob(input): Convert a string into a base64 encoded string
btoa(input): Convert a base64 encoded string to an ascii encoded string
call(filename): Run the contents of a file as a script
help([function_name]): Display a help message
prettyjson(input): Beautify a JSON input string
run(expression): Run a string as an expression
tail(filename, [lines]): Returns the last [lines] lines from a given file
time(): Returns a unix timestamp for the current system time
urldecode(input): Decode urlencoded character escape sequences in a string
urlencode(input): Escape characters in a string for use in a URL

Network Functions
=================
api(name, [endpoint]): Make a call to a registered API
api_delete(name): Remove a registered API from the list
api_list(): List all registered APIs
api_register(name, base_url, [api_key]): Register a new API for quick usage
get(url, [headers]): Return the resulting text-format body of an HTTP GET call
post(url, body, [headers]): Return the resulting text-format body of an HTTP POST call
resolve(hostname): Returns the IP address associated to a given hostname

Arrays Functions
================
dequeue(array): Remove the first element from an array
element(input, index): Return an element from a location in an array or object
enqueue(array, element): Add an element to the end of an array
is_empty(input): Returns true if the given array or object is empty
keys(input): Get a list of keys in the object or array
len(input): Returns the length of the given array or object
merge(target, inputs1, inputs2): Merge all given arrays or objects
pop(array): Remove the last element from an array
push(array, element): Add an element to the end of an array
remove(input, index): Removes an element from an array
values(input): Get a list of values in the object or array

Strings Functions
=================
concat([s1, s2]): Concatenate a set of strings
contains(source, s): Returns true if array or string [source] contains [s]
lowercase(s): Converts the string s to lowercase
regex(pattern, subject, [group]): Returns a regular expression match from [subject], or false
strlen(s): Returns the length of the string s
substr(s, start, [length]): Returns a substring from s, beginning at [start], and going to the end, or for [length] characters
trim(s): Trim whitespace from a string
uppercase(s): Converts the string s to uppercase

Cryptography Functions
======================
choose(option1, option2): Returns any one of the provided arguments at random
md5(input1, input2): Returns the MD5 hash of a given string
rand([m], [n]): With no arguments, return a float from 0 to 1. Otherwise return an integer from 0 to m, or m to n
sha256(input1, input2): Returns the SHA256 hash of a given string

Math Functions
==============
abs(n): Returns the absolute value of n
acos(n): Calculate the arccosine of n
array(n): Returns a value as an array
asin(n): Calculate the arcsine of n
atan(n): Calculate the arctangent of n
bool(n): Returns a value as a boolean
ceil(n): Returns the nearest whole integer larger than n
cos(n): Calculate the cosine of n
cosh(n): Calculate the hyperbolic cosine of n
float(n): Returns a value as a float
floor(n): Returns the nearest whole integer smaller than n
int(n): Returns a value as an integer
ln(n): Returns the natural log of n
log(n, base): Returns the logarithm of n in any base
log10(n): Returns the base 10 log of n
max(n1, n2): Returns the largest numeric value from the supplied arguments
min(n1, n2): Returns the smallest numeric value from the supplied arguments
root(n, base): Returns a root of n of any base
round(n, [precision]): Returns n, rounded to [precision] decimal places
sin(n): Calculate the sine of n
sinh(n): Calculate the hyperbolic sine of n
sqrt(n): Returns the square root of n
tan(n): Calculate the tangent of n
tanh(n): Calculate the hyperbolic tangent of n
to_degrees(n): Convert the given radian value into degrees
to_radians(n): Convert the given degree value into radians

Built-in Decorators
===================
@array: Format a number as an array
@bin: Base 2 number formatting, such as 0b11
@bool/@boolean: Format a number as a boolean
@bool/@boolean: Format a number as a boolean
@default: Default formatter, type dependent
@dollar/@dollars/@usd/@aud/@cad: Format a number as a dollar amount
@dollar/@dollars/@usd/@aud/@cad: Format a number as a dollar amount
@dollar/@dollars/@usd/@aud/@cad: Format a number as a dollar amount
@dollar/@dollars/@usd/@aud/@cad: Format a number as a dollar amount
@dollar/@dollars/@usd/@aud/@cad: Format a number as a dollar amount
@euro/@euros: Format a number as a euro amount
@euro/@euros: Format a number as a euro amount
@float: Format a number as floating point
@hex: Base 16 number formatting, such as 0xFF
@int/@integer: Format a number as an integer
@int/@integer: Format a number as an integer
@object: Format a number as an object
@oct: Base 8 number formatting, such as 0b77
@percentage/@percent: Format a floating point number as a percentage
@percentage/@percent: Format a floating point number as a percentage
@pound/@pounds: Format a number as a pound amount
@pound/@pounds: Format a number as a pound amount
@roman: Format an integer as a roman numeral
@sci: Scientific number formatting, such as 1.2Ee-3
@utc: Interprets an integer as a timestamp, and formats it in UTC standard
@yen: Format a number as a yen amount
```


License: MIT OR Apache-2.0
