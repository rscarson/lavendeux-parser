# Lavendeux Parser - Extensible inline parser engine
Backend parsing engine for Lavendeux.

```
let input = "x=9\nsqrt(x) @bin
let mut state : ParserState = ParserState::new()
let lines = Token::from_input(input, &mut state)?;
// lines will contain the result of the input, as an array of string:
//    9
//    0b11
```

## Data types
**bin:** Base 2 binary, prefixed by 0b, such as 0b11
**oct:** Base 8 octal, prefixed with 0o, or any number begining with 0, such as 0o7, or 07
**hex:** Base 16 hexadecimal, prefixed with 0x, such as 0xFE
**sci:** Scientific notation floating point numbers, such as .3E+5, 2.5E3, or 5E-6
**float:** 64bit floating point number, such as 3.56, or .8
**int:** 64bit integer, such as 956, or 2,
**boolean:** true or false,
**currency:** Currency value - currently dollars, pounds, euros, and yen are supported
**string:** Quoted string, with single or double quotes. Supports a variety of escape sequences

## Variables
Expression values can be assigned to a variable. That variable can then be used in expressions:
```
i = sqrt(9)
i**2
```

Functions can also be defined:
```
f(x) = 2*x**2 + 3*x + 5
f(2)
```

Values for **pi**, **tau** and **e** are defined as constants, and can be used in expressions.

## Operators
|Operation| Format |
|--|--|
| Concatenation | *string* **+** *string* |
| Addition/Substraction | *number* **(+, -)** *number* |
| Multiplication/Division | *number* **(\*, /)** *number* |
| Modulo | *number* **%** *number* |
| Exponentiation | *number* **\*\*** *number* |
| Factorial | *number* **!** |
| Bitwise AND | *int* **&** *int* |
| Bitwise OR | *int* **|** *int* |
| Bitwise XOR | *int* **^** *int* |
| Bitwise Shift | *int* **(<<, >>)** *int* |
| Bitwise NOT | **~** *int* |
| Boolean AND | *boolean* **&&** *boolean* |
| Boolean OR | *boolean* **||** *boolean* |
| Boolean EQ | *boolean* **==** *boolean* |
| less/greater than | *value* **<** *boolean*, *boolean* **>** *value* |
| ternary | *condition* ? *value* : *value* |

## Functions
| Function | Usage |
|--|--|
| ceil(n) | Round n up to the nearest whole integer |
| floor(n) | Round n down to the nearest whole integer |
| round(n, precision=0) | Round n to a given precision |
| abs(n) | Return the absolute value of n |
| to_radians(n) | Convert integer n from degrees to radians |
| tan(n), atan(n), tanh(n) | Calculate the tangent, arctangent or hyperbolic tangent of radian value n |
| cos(n), acos(n), cosh(n) | Calculate the cosine, arccosine or hyperbolic cosine of radian value n |
| sin(n), asin(n), sinh(n) | Calculate the sine, arcsine or hyperbolic sine of radian value n |
| log10(n) | Calculate the base 10 logarithm of n |
| ln(n) | Calculate the base e logarithm of n |
| log(n, base) | Calculate the base 'base' 10 logarithm of n |
| sqrt(n) | Calculate the square root of n |
| root(n, k) | Calculate the Kth root of n |
| strlen(s) | Return the length of string s |
| substr(s, start, [end]) | Return the substring of string s, from start to end. If end is omited, return until end of string |

## Decorators
Decorators can be added to the end of an expression to modify how the resulting value is formatted:
```
16 @hex        ->    0xF
sqrt(9) @bin   ->    0b11
```
| Decorator | Usage |
|--|--|
| @hex | Format a number as hexadecimal - rounding down floats if needed |
| @oct | Format a number as octal - rounding down floats if needed |
| @bin | Format a number as binary - rounding down floats if needed |
| @int | Format a number as an integer - rounding down floats if needed |
| @sci | Format a number in scientific notation |
| @float | Format a number as floating point |
| @dollar, @usd, @cad, @aud, @euro, @pound, @yen | Format a number as a currency amount |
| @utc | Format an integer as a UTC timestamp |