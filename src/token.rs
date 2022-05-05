extern crate pest;
extern crate pest_derive;
use pest::Parser;
use pest_derive::Parser;

use super::errors::*;
use super::handlers;
use super::state::{ParserState, UserFunction};
use super::value::Value;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct CalcParser;

#[derive(Copy, Clone, Debug)]
pub enum OutputFormat {
    Unknown = 0,
    Default = 10,
    Dollars = 20, Euros = 21, Pounds = 22, Yen = 23
}

type TokenHandler = fn(&mut Token, &mut ParserState) -> Option<ParserError>;

/// Represents a token tree for a parsed expression
/// The root contains the text result of parsing the expression,
/// as well as one child per line being parsed
/// 
/// So if you were to parse:
/// ```ignore
/// 5 + 5
/// sqrt(5!)
/// ```
/// 
/// The token tree would look like this:
/// ```text
/// script: 5 + 5\nsqrt(5!)
///     line: 5 + 5
///         as_expression: 5 + 5
///             int: 5
///             operator: +
///             int: 5
///     line: sqrt(5!)
///         call_expression: sqrt(5!)
///             prefix_unary_expression: 5!
///                 int: 5
///                 operator: !
/// ```
/// 
/// Each token in the tree stores the text and actual value representations of the result
#[derive(Clone, Debug)]
pub struct Token {
    rule: Rule,
    input: String,
    text: String,
    format: OutputFormat,
    value: Value,
    index: usize,
    children: Vec<Token>
}

impl Token {
    const DEFAULT_HANDLER : TokenHandler = handlers::handler;

    /// Parses an input string, and returns the resulting token tree
    /// 
    /// ```rust
    /// use lavendeux_parser::{ParserState, ParserError, Token, Value};
    /// 
    /// fn main() -> Result<(), ParserError> {
    ///     // Create a new parser, and tokenize 2 lines
    ///     let mut state : ParserState = ParserState::new();
    ///     let lines = Token::new("x=9\nsqrt(x) @bin", &mut state)?;
    /// 
    ///     // The resulting token contains the resulting values and text
    ///     assert_eq!(lines.text(), "9\n0b11");
    ///     assert_eq!(lines.child(1).unwrap().value(), Value::Integer(3));
    ///     
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Arguments
    /// * `input` - Source string
    /// * `state` - The current parser state
    pub fn new(input: &str, state: &mut ParserState) -> Result<Token, ParserError> {
        Self::new_with_handler(input, Self::DEFAULT_HANDLER, state)
    }

    /// Parses an input string, and returns the resulting token tree
    /// Allows a custom handler function
    /// 
    /// ```rust
    /// use lavendeux_parser::{ParserState, ParserError, Token, Value};
    /// 
    /// fn main() -> Result<(), ParserError> {
    ///     // Create a new parser, and tokenize 2 lines
    ///     let mut state : ParserState = ParserState::new();
    ///     let lines = Token::new("x=9\nsqrt(x) @bin", &mut state)?;
    /// 
    ///     // The resulting token contains the resulting values and text
    ///     assert_eq!(lines.text(), "9\n0b11");
    ///     assert_eq!(lines.child(1).unwrap().value(), Value::Integer(3));
    ///     
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Arguments
    /// * `input` - Source string
    /// * `handler` - The handler function that receives the token
    /// * `state` - The current parser state
    pub fn new_with_handler(input: &str, handler: TokenHandler, state: &mut ParserState) -> Result<Token, ParserError> {
        let pairs = CalcParser::parse(Rule::script, input);
        match pairs {
            Ok(mut r) => {
                match r.next() {
                    None => Ok(Token {
                        format: OutputFormat::Default,
                        text: "".to_string(),
                        input: "".to_string(),
                        value: Value::None,
                        children: Vec::new(),
                        index: 0,
                        rule: Rule::script
                    }),
                    Some(p) => Token::from_pair(p, handler, state)
                }
            }
            
            Err(e) => Err(ParserError::Pest(PestError::new(&e.to_string())))
        }
    }
    
    /// Parses an input string, and returns the resulting token tree
    /// 
    /// # Arguments
    /// * `input` - Source string
    /// * `state` - The current parser state
    #[deprecated]
    pub fn from_input(input: &str, state: &mut ParserState) -> Result<Token, ParserError> {
        Self::new(input, state)
    }

    /// Converts a pest pair object into a token tree, and returns it
    /// 
    /// # Arguments
    /// * `pair` - A pair object returned by pest
    /// * `handler` - The handler function that receives the token
    fn from_pair(pair: pest::iterators::Pair<Rule>, handler: TokenHandler, state: &mut ParserState) -> Result<Token, ParserError> {
        // Collapse tree
        let mut next_pair = pair;
        let mut children : Vec<_> = next_pair.clone().into_inner().into_iter().collect();
        while children.len() == 1 && next_pair.as_rule() != Rule::script && next_pair.as_rule() != Rule::line {
            next_pair = children[0].clone();
            children = next_pair.clone().into_inner().into_iter().collect();
        }

        // Collect basic properties
        let mut token = Self{
            rule: next_pair.as_rule(),
            input: next_pair.as_str().to_string(),
            text: next_pair.as_str().to_string(),
            format: OutputFormat::Unknown,
            value: Value::None,
            index: next_pair.as_span().start(),
            children: Vec::new()
        };

        if token.rule == Rule::ternary_expression && children.len() > 1 {
            // Ternary expression handler - enables short-circuit interpretation
            let condition = Self::from_pair(children[0].clone(), handler, state)?;
            token = Self::from_pair(if condition.value.as_bool() { children[1].clone() } else { children[2].clone() }, handler, state)?;
        } else if !children.is_empty() && children[0].clone().as_rule() == Rule::function_assignment {
            // Function assignment handler - prevents prematurely executing the new function
            let mut function_children: Vec<_> = children[0].clone().into_inner().into_iter().collect();
            let name = function_children.first().unwrap().as_str().to_string();
            let definition = function_children.last().unwrap().as_str().to_string();

            // Compile arguments
            let mut arguments : Vec<String> = Vec::new();
            function_children.remove(0); function_children.remove(0);
            for argument in function_children {
                let s = argument.as_str();
                if s == ")" { break; }
                if s == "," { continue; }
                arguments.push(s.to_string());
            }

            // Store new function
            state.user_functions.insert(name.to_string(), UserFunction {
                name, arguments,
                definition: definition.to_string()
            });

            token.children.push(Token {
                rule: Rule::function_assignment,
                input: token.input.clone(),
                text: definition,
                format: OutputFormat::Unknown,
                value: Value::String(token.input.clone()),
                index: token.index,
                children: Vec::new()
            });

            let eol = children.last().unwrap();
            token.children.push(Token {
                rule: Rule::eol,
                input: eol.as_str().to_string(),
                text: eol.as_str().to_string(),
                format: OutputFormat::Unknown,
                value: Value::String(eol.as_str().to_string()),
                index: eol.as_span().start(),
                children: Vec::new()
            });
            
            token.value = token.child(0).unwrap().value();
            token.text = token.child(0).unwrap().text().to_string();
        } else {
            // Default token handler
            for child in children {
                let t = Self::from_pair(child, handler, state)?;
                token.children.push(t);
            }

            // Run token handler to get value
            if let Some(e) = handler(&mut token, state) {
                return Err(e);
            }
        }

        Ok(token)
    }

    /// Return the token's rule
    pub fn rule(&self) -> Rule {
        self.rule
    }

    /// Return the token's position in the input string
    pub fn index(&self) -> usize {
        self.index
    }

    /// Return the token's input string
    pub fn input(&self) -> &str {
        &self.input
    }

    /// Return the token's output string
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Set the token's output string
    pub fn set_text(&mut self, t: &str) {
        self.text = t.to_string()
    }

    /// Return the token's output format
    pub fn format(&self) -> OutputFormat {
        self.format
    }

    /// Get the nth child token, if possible
    pub fn child(&self, n: usize) -> Option<&Token> {
        if n < self.children.len() {
            Some(&self.children[n])
        } else {
            None
        }
    }

    /// Get the token's children
    pub fn children(&self) -> &Vec<Token> {
        &self.children
    }

    /// Set the token's output format
    pub fn set_format(&mut self, f: OutputFormat) {
        self.format = f;
    }

    /// Return the token's output value
    pub fn value(&self) -> Value {
        self.value.clone()
    }

    /// Set the token's output value
    pub fn set_value(&mut self, v: Value) {
        self.value = v;
    }
}

#[cfg(test)]
mod test_token {
    #[cfg(feature = "extensions")]
    use crate::extensions::{Extension};
    use super::*;

    fn token_does_value_equal(input: &str, expected: Value, state: &mut ParserState) {
        let t = Token::new(input, state).unwrap();
        assert_eq!(expected, t.value);
    }

    fn token_does_error(input: &str, state: &mut ParserState) {
        assert_eq!(true, Token::new(input, state).is_err());
    }

    fn token_does_text_equal(input: &str, expected: &str, state: &mut ParserState) {
        let t = Token::new(input, state).unwrap();
        assert_eq!(expected, t.text);
    }

    #[test]
    fn test_token_from_input() {
        let mut state: ParserState = ParserState::new();
        assert_eq!("5+5", Token::new_with_handler("5+5", |_, _| None, &mut state).unwrap().text);
    }

    #[test]
    fn test_from_input() {
        let mut state: ParserState = ParserState::new();
        assert_eq!(Value::Integer(10), Token::new("5+5", &mut state).unwrap().value);
    }

    #[test]
    fn test_grammar_atomic_value() {
        let mut state: ParserState = ParserState::new();

        // Hex
        token_does_value_equal("0x0F", Value::Integer(15), &mut state);
        token_does_value_equal("0x0f", Value::Integer(15), &mut state);
        token_does_value_equal("0x0", Value::Integer(0), &mut state);
        token_does_value_equal("0XFF", Value::Integer(255), &mut state);

        // Bin
        token_does_value_equal("0b00", Value::Integer(0), &mut state);
        token_does_value_equal("0B11111111", Value::Integer(255), &mut state);

        // Oct
        token_does_value_equal("0o00", Value::Integer(0), &mut state);
        token_does_value_equal("0O777", Value::Integer(511), &mut state);

        // Oct
        token_does_value_equal("0o00", Value::Integer(0), &mut state);
        token_does_value_equal("0O777", Value::Integer(511), &mut state);

        // Sci
        token_does_value_equal("1,000e5", Value::Float(100000000.0), &mut state);
        token_does_value_equal(".4e5", Value::Float(40000.0), &mut state);
        token_does_value_equal("5e5", Value::Float(500000.0), &mut state);
        token_does_value_equal("5E5", Value::Float(500000.0), &mut state);
        token_does_value_equal("5e+5", Value::Float(500000.0), &mut state);
        token_does_value_equal("5e-5", Value::Float(5e-5), &mut state);

        // Float
        token_does_value_equal("10000000.00", Value::Float(10000000.0), &mut state);
        token_does_value_equal("¥10,000,000.00", Value::Float(10000000.0), &mut state);
        token_does_value_equal("$10,000,000.00", Value::Float(10000000.0), &mut state);
        token_does_value_equal("$10,000,000", Value::Float(10000000.0), &mut state);
        token_does_value_equal(".4", Value::Float(0.4), &mut state);
        token_does_value_equal("4.4", Value::Float(4.4), &mut state);

        // Int
        token_does_value_equal("1,000", Value::Integer(1000), &mut state);
        token_does_value_equal("999", Value::Integer(999), &mut state);
        token_does_value_equal("0", Value::Integer(0), &mut state);

        // String
        token_does_value_equal("'test'", Value::String("test".to_string()), &mut state);
        token_does_value_equal("       '  test   '       ", Value::String("  test   ".to_string()), &mut state);
        token_does_value_equal("'test\"'", Value::String("test\"".to_string()), &mut state);
        token_does_value_equal("'test\\\"'", Value::String("test\"".to_string()), &mut state);
        token_does_value_equal("\"test\\\'\"", Value::String("test\'".to_string()), &mut state);
        token_does_value_equal("\"test\\\'\"", Value::String("test\'".to_string()), &mut state);

        // Identifier
        state.variables.insert("x".to_string(), Value::Integer(99));
        state.variables.insert("x_9".to_string(), Value::Integer(99));
        token_does_value_equal("x", Value::Integer(99), &mut state);
        token_does_value_equal("x_9", Value::Integer(99), &mut state);
    }

    #[test]
    fn test_grammar_script() {
        let mut state: ParserState = ParserState::new();

        token_does_text_equal("\n\n", "\n\n", &mut state);
        token_does_text_equal("\n\n5", "\n\n5", &mut state);
        token_does_text_equal("5+5\n5+5", "10\n10", &mut state);
        token_does_value_equal("$1,000.00 == ¥1,000.00", Value::Boolean(true), &mut state);

        // Empty lines and comments
        token_does_text_equal("5+5\n\n\n// Test\n5+5 // test", "10\n\n\n\n10", &mut state);

        // Line
        token_does_value_equal("5", Value::Integer(5), &mut state);
        token_does_text_equal("5 @bin", "0b101", &mut state);
        token_does_text_equal("5 @int", "5", &mut state);

        // Comments
        token_does_value_equal("5 //test", Value::Integer(5), &mut state);
        token_does_value_equal("//test", Value::None, &mut state);
        
        // Assignment expression
        token_does_value_equal("x = 5", Value::Integer(5), &mut state);
        assert_eq!(1, state.variables.len());

        // Term
        token_does_value_equal("(5)", Value::Integer(5), &mut state);
    }

    #[test]
    fn test_grammar_expression() {
        let mut state: ParserState = ParserState::new();

        // Unary expression
        token_does_value_equal("~0b101", Value::Integer(2), &mut state);
        token_does_value_equal("~0b11111111", Value::Integer(0), &mut state);
        token_does_value_equal("~0b0", Value::Integer(-1), &mut state);
        token_does_value_equal("-1", Value::Integer(-1), &mut state);
        token_does_value_equal("-0", Value::Integer(0), &mut state);
        token_does_value_equal("-1.1", Value::Float(-1.1), &mut state);
        token_does_value_equal("1!", Value::Integer(1), &mut state);
        token_does_value_equal("5!", Value::Integer(120), &mut state);
        token_does_value_equal("-5!", Value::Integer(-120), &mut state);
        token_does_value_equal("-~3!!", Value::Integer(-303), &mut state);

        // Overflows and errors
        token_does_error("1/0", &mut state);
        token_does_error("5+5\n 1/0", &mut state);
        token_does_error("99999999999999999999999999999999999999999", &mut state);
        token_does_error("1+99999999999999999999999999999999999999999", &mut state);
        token_does_error("999999999999999999*999999999999999999", &mut state);
        token_does_error("999!", &mut state);

        // Ternary expression
        token_does_value_equal("true ? 1 : 2", Value::Integer(1), &mut state);
        token_does_value_equal("false ? 1 : 2", Value::Integer(2), &mut state);
        token_does_value_equal("false ? 1/0 : 2", Value::Integer(2), &mut state);

        // Call expression
        token_does_error("rooplipp(9)", &mut state);
        token_does_error("sqrt('string')", &mut state);
        token_does_error("sqrt()", &mut state);
        token_does_value_equal("sqrt(9)", Value::Integer(3), &mut state);
        token_does_value_equal("sqrt(9 | 5)", Value::Integer(3), &mut state);
        token_does_value_equal("root(9, 2)", Value::Integer(3), &mut state);

        // Power expression
        token_does_value_equal("2**2", Value::Integer(4), &mut state);
        token_does_value_equal("2**2**2", Value::Integer(16), &mut state);
        token_does_value_equal("2**2**(2|2)", Value::Integer(16), &mut state);

        // multiply / divide expression
        token_does_value_equal("2*2", Value::Integer(4), &mut state);
        token_does_value_equal("2*2*2", Value::Integer(8), &mut state);
        token_does_value_equal("2*2/(2|2)", Value::Integer(2), &mut state);
        token_does_value_equal("x=4(4)", Value::Integer(16), &mut state);
        token_does_value_equal("4x", Value::Integer(64), &mut state);
        token_does_value_equal("4(x)", Value::Integer(64), &mut state);
        token_does_value_equal("(4)x", Value::Integer(64), &mut state);
        token_does_value_equal("(2)(2)(2)(2)", Value::Integer(16), &mut state);
        token_does_value_equal("(2)(2)(2)(-2)", Value::Integer(-16), &mut state);
        token_does_value_equal("2-2", Value::Integer(0), &mut state);
        token_does_value_equal("-2x", Value::Integer(-32), &mut state);

        // add / sub expression
        token_does_text_equal("2*$2", "$4.00", &mut state);
        token_does_value_equal("2+2", Value::Integer(4), &mut state);
        token_does_value_equal("2+2+2", Value::Integer(6), &mut state);
        token_does_value_equal("2+2-2/2", Value::Integer(3), &mut state);

        // shift expression
        token_does_value_equal("2<<2", Value::Integer(8), &mut state);
        token_does_value_equal("2<<2>>2", Value::Integer(2), &mut state);
        token_does_value_equal("2<<2>>(2+1)", Value::Integer(1), &mut state);

        // bitwise expressions
        token_does_value_equal("0b1100 & 0b0011", Value::Integer(0), &mut state);
        token_does_value_equal("0b1100 | 0b0011", Value::Integer(15), &mut state);
        token_does_value_equal("0b1110 ^ 0b0011", Value::Integer(13), &mut state);
        token_does_value_equal("0b0001 | 0b0011 ^ 0b1111", Value::Integer(13), &mut state);

        // boolean expressions
        token_does_value_equal("2 < 3", Value::Boolean(true), &mut state);
        token_does_value_equal("1 > 2 < true", Value::Boolean(true), &mut state);
        token_does_value_equal("5.0 < 3", Value::Boolean(false), &mut state);
        token_does_value_equal("'test' > 'a'", Value::Boolean(true), &mut state);
        token_does_value_equal("'test' && 'a'", Value::Boolean(true), &mut state);
        token_does_value_equal("true && true && false", Value::Boolean(false), &mut state);
        token_does_value_equal("1 && 1", Value::Boolean(true), &mut state);
        token_does_value_equal("1 && 0", Value::Boolean(false), &mut state);
        token_does_value_equal("false || false || true", Value::Boolean(true), &mut state);
        token_does_value_equal("true || false", Value::Boolean(true), &mut state);
        token_does_value_equal("true == false", Value::Boolean(false), &mut state);
        token_does_value_equal("true == false != true", Value::Boolean(true), &mut state);

        // Function
        let t = Token::new("5+5\nfn(x, y) = x * y\n5+5", &mut state).unwrap();
        assert_eq!("10\nx * y\n10", t.text);
        token_does_value_equal("fn(5,5)", Value::Integer(25), &mut state);
        let t = Token::new("fn(x, y) = 5x + 10(x * y)\nfn(2, 3)", &mut state).unwrap();
        assert_eq!("5x + 10(x * y)\n70", t.text);
        assert_eq!(true, Token::new("f(x) = f(x)\nf(0)", &mut state).is_err());

        // Help
        #[cfg(feature = "extensions")]
        state.extensions.add("test.js", Extension::new_stub(
            None, None, None, 
            vec!["test".to_string(), "test2".to_string()], 
            vec!["test3".to_string(), "test4".to_string()]
        ));

        let t = Token::new("help()", &mut state).unwrap();
        assert_eq!(t.text(), "".to_string());
        assert_eq!(true, t.text.contains("Built-in Functions"));
        assert_eq!(true, t.text.contains("Built-in Decorators"));
        
        #[cfg(feature = "extensions")]
        assert_eq!(true, t.text.contains("Unnamed Extension v0.0.0"));

        assert_eq!(true, t.text.contains("User-defined Functions"));
        token_does_text_equal("help('strlen')", "strlen(s): Returns the length of the string s", &mut state);
        token_does_text_equal("help(strlen)", "strlen(s): Returns the length of the string s", &mut state);
        token_does_text_equal("help('fn')", "fn(x, y)", &mut state);
        token_does_text_equal("help(fn)", "fn(x, y)", &mut state);
        
        #[cfg(feature = "extensions")]
        token_does_text_equal("help('test2')", "test2(...)", &mut state);
        
        #[cfg(feature = "extensions")]
        token_does_text_equal("help(test2)", "test2(...)", &mut state);
    }
}