
# Lavendeux Parser - Extensible inline parser engine

Extensible parsing library for expression evaluation.
Acts as the engine for [Lavendeux](https://rscarson.github.io/Lavendeux/)

## Syntax
For detailed syntax documentation, visit https://rscarson.github.io/Lavendeux/

## How to use it
Simple example below:

```rust
let mut state : ParserState = ParserState::new();
let lines = Token::new("x=9\nsqrt(x) @bin", &mut state)?;

// The resulting token contains the resulting values and text
assert_eq!(lines.text, "9\n0b11");
assert_eq!(lines.children[1].value, AtomicValue::Integer(3));
```

Extensions can be loaded as follows:
```rust
// Load one extension
let extension = Extension::new("filename.js")?;
state.extensions.push(extension);

// Load a whole directory
extensions = Extension::load_all("./directory")?;
state.extensions = extensions;

// Once loaded, functions and @decorators decribed in the extensions
// can be called in expressions being parsed
```

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