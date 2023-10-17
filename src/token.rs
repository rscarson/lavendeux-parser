use std::fmt::Display;

use crate::{Error, ParserState, Value};

extern crate pest;
extern crate pest_derive;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LavendeuxParser;

#[derive(Copy, Clone, Debug)]
pub enum OutputFormat {
    Unknown = 0,
    Default = 10,
    Dollars = 20,
    Euros = 21,
    Pounds = 22,
    Yen = 23,
}

/// Represents a token tree for a parsed expression
/// The root contains the text result of parsing the expression,
/// as well as one child per line being parsed
///
/// So if you were to parse:
/// ```text
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
    children: Vec<Token>,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            format: OutputFormat::Default,
            text: String::default(),
            input: String::default(),
            value: Value::None,
            children: Vec::new(),
            index: 0,
            rule: Rule::script,
        }
    }
}

pub trait LavendeuxHandler {
    fn handle_tree(&self, token: &mut Token, state: &mut ParserState) -> Result<(), Error>;
}

impl Token {
    /// Parses an input string, and returns the resulting token tree
    ///
    /// ```rust
    /// use lavendeux_parser::{ParserState, Error, Token, Value};
    ///
    /// fn main() -> Result<(), Error> {
    ///     // Create a new parser, and tokenize 2 lines
    ///     let mut state : ParserState = ParserState::new();
    ///     let lines = Token::new("x=9\nsqrt(x) @bin", &mut state)?;
    ///
    ///     // The resulting token contains the resulting values and text
    ///     assert_eq!(lines.text(), "9\n0b11");
    ///     assert_eq!(lines.child(1).unwrap().value(), Value::Float(3.0));
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Arguments
    /// * `input` - Source string
    /// * `state` - The current parser state
    pub fn new(input: &str, state: &mut ParserState) -> Result<Token, Error> {
        Self::parse(input, crate::handlers::Handler::default(), state)
    }

    /// Convert one pair into a token
    /// Does not process child tokens
    ///
    /// # Arguments
    /// * `input` - Source
    fn from_pair(input: pest::iterators::Pair<Rule>) -> Token {
        Self {
            rule: input.as_rule(),
            input: input.as_str().to_string(),
            text: input.as_str().to_string(),
            format: OutputFormat::Unknown,
            value: Value::None,
            index: input.as_span().start(),
            children: Vec::new(),
        }
    }

    /// Convert raw input into a dummy Token
    /// Usually for error handling
    /// Does not process child tokens
    ///
    /// # Arguments
    /// * `input` - Source input
    pub fn dummy(input: &str) -> Token {
        Self {
            rule: Rule::script,
            input: input.to_string(),
            text: input.to_string(),
            format: OutputFormat::Unknown,
            value: Value::None,
            index: 0,
            children: Vec::new(),
        }
    }

    /// Parses an input string, and returns the resulting token tree
    ///
    /// # Arguments
    /// * `input` - Source string
    /// * `handler` - A LavendeuxHandler
    /// * `state` - The current parser state
    fn parse<A>(input: &str, handler: A, state: &mut ParserState) -> Result<Self, Error>
    where
        A: LavendeuxHandler,
    {
        let pairs = LavendeuxParser::parse(Rule::script, input);
        match pairs {
            Ok(mut r) => match r.next() {
                None => Ok(Self::default()),
                Some(p) => {
                    let mut token = Self::build_tree(p);
                    handler.handle_tree(&mut token, state)?;
                    Ok(token)
                }
            },
            Err(e) => Err(Error::Pest(e, Token::dummy(input))),
        }
    }

    /// Build a token tree from a parser pair
    ///
    /// # Arguments
    /// * `root` - Pair that will form the tree's root
    fn build_tree(root: pest::iterators::Pair<Rule>) -> Token {
        // Collapse tree
        let mut next_pair = root;
        let mut children: Vec<_> = next_pair.clone().into_inner().collect();
        while children.len() == 1
            && next_pair.as_rule() != Rule::script
            && next_pair.as_rule() != Rule::line
        {
            next_pair = children[0].clone();
            children = next_pair.clone().into_inner().collect();
        }

        // Collect basic properties
        let mut token = Token::from_pair(next_pair);

        for child in children {
            token.children.push(Self::build_tree(child));
        }

        token
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

    /// Get the nth child token as a mutable reference, if possible
    pub fn mut_child(&mut self, n: usize) -> Option<&mut Token> {
        if n < self.children.len() {
            Some(&mut self.children[n])
        } else {
            None
        }
    }

    /// Get the nth child token, if possible
    pub fn child(&self, n: usize) -> Option<&Token> {
        if n < self.children.len() {
            Some(&self.children[n])
        } else {
            None
        }
    }

    /// Get the token's children as a mutable reference
    pub fn mut_children(&mut self) -> &mut Vec<Token> {
        &mut self.children
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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[cfg(test)]
mod test_token {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_token_from_input() {
        let mut state: ParserState = ParserState::new();
        assert_eq!("10", Token::new("5+5", &mut state).unwrap().text);
    }

    #[test]
    fn test_from_input() {
        let mut state: ParserState = ParserState::new();
        assert_eq!(
            Value::Integer(10),
            Token::new("5+5", &mut state).unwrap().value
        );
    }

    #[test]
    fn test_grammar_atomic_value() {
        let mut state: ParserState = ParserState::new();

        // Hex
        assert_token_value!("0x0F", Value::Integer(15));
        assert_token_value!("0x0F", Value::Integer(15));
        assert_token_value!("0x0f", Value::Integer(15));
        assert_token_value!("0x0", Value::Integer(0));
        assert_token_value!("0XFF", Value::Integer(255));

        // Bin
        assert_token_value!("0b00", Value::Integer(0));
        assert_token_value!("0B11111111", Value::Integer(255));

        // Oct
        assert_token_value!("0o00", Value::Integer(0));
        assert_token_value!("0O777", Value::Integer(511));

        // Oct
        assert_token_value!("0o00", Value::Integer(0));
        assert_token_value!("0O777", Value::Integer(511));

        // Sci
        assert_token_value!("1,000e5", Value::Float(100000000.0));
        assert_token_value!(".4e5", Value::Float(40000.0));
        assert_token_value!("5e5", Value::Float(500000.0));
        assert_token_value!("5E5", Value::Float(500000.0));
        assert_token_value!("5e+5", Value::Float(500000.0));
        assert_token_value!("5e-5", Value::Float(5e-5));

        // Float
        assert_token_value!("10000000.00", Value::Float(10000000.0));
        assert_token_value!("¥10,000,000.00", Value::Float(10000000.0));
        assert_token_value!("$10,000,000.00", Value::Float(10000000.0));
        assert_token_value!("$10,000,000", Value::Integer(10000000));
        assert_token_value!(".4", Value::Float(0.4));
        assert_token_value!("4.4", Value::Float(4.4));

        // Int
        assert_token_value!("1,000", Value::Integer(1000));
        assert_token_value!("999", Value::Integer(999));
        assert_token_value!("0", Value::Integer(0));

        // String
        assert_token_value!("'test'", Value::String("test".to_string()));
        assert_token_value!(
            "       '  test   '       ",
            Value::String("  test   ".to_string())
        );
        assert_token_value!("'test\"'", Value::String("test\"".to_string()));
        assert_token_value!("'test\\\"'", Value::String("test\"".to_string()));
        assert_token_value!("\"test\\\'\"", Value::String("test\'".to_string()));
        assert_token_value!("\"test\\\'\"", Value::String("test\'".to_string()));

        // Identifier
        state.variables.insert("x".to_string(), Value::Integer(99));
        state
            .variables
            .insert("x_9".to_string(), Value::Integer(99));
        assert_token_value_stateful!("x", Value::Integer(99), &mut state);
        assert_token_value_stateful!("x_9", Value::Integer(99), &mut state);
    }

    #[test]
    fn test_grammar_script() {
        let mut state: ParserState;

        assert_token_text!("\n\n", "\n\n");
        assert_token_text!("\n\n5", "\n\n5");
        assert_token_text!("5+5\n5+5", "10\n10");
        assert_token_value!("$1,000.00 == ¥1,000.00", Value::Boolean(true));

        // Empty lines and comments
        assert_token_text!("5+5\n\n\n// Test\n5+5 // test", "10\n\n\n\n10");

        // Line
        assert_token_value!("5", Value::Integer(5));
        assert_token_text!("5 @bin", "0b101");
        assert_token_text!("5 @int", "5");

        // Comments
        assert_token_value!("5 //test", Value::Integer(5));
        assert_token_value!("//test", Value::None);

        // Assignment expression
        state = ParserState::new();
        assert_token_value_stateful!("x = 5", Value::Integer(5), &mut state);
        assert_eq!(1, state.variables.len());

        // Indexed assignment expression
        state = ParserState::new();
        state
            .variables
            .insert("x".to_string(), Value::Array(vec![Value::Integer(5)]));
        assert_token_value_stateful!("x[0] = 3", Value::Integer(3), &mut state);
        assert_token_value_stateful!(
            "x[1] = 'test'",
            Value::String("test".to_string()),
            &mut state
        );
        assert_token_error_stateful!("x[-1] = 5", Index, &mut state);
        assert_token_error_stateful!("x['test'] = 5", ValueType, &mut state);
        assert_token_error_stateful!("x[3] = 5", Index, &mut state);
        assert_eq!(1, state.variables.len());
        assert_eq!(
            Value::Integer(3),
            state.variables.get("x").unwrap().as_array()[0]
        );
        assert_eq!(
            Value::String("test".to_string()),
            state.variables.get("x").unwrap().as_array()[1]
        );

        // Term
        assert_token_value!("(5)", Value::Integer(5));
    }

    #[test]
    fn test_grammar_expression() {
        // Unary expression
        assert_token_value!("~0b101", Value::Integer(2));
        assert_token_value!("~0b11111111", Value::Integer(0));
        assert_token_value!("~0b0", Value::Integer(-1));
        assert_token_value!("-1", Value::Integer(-1));
        assert_token_value!("-0", Value::Integer(0));
        assert_token_value!("-1.1", Value::Float(-1.1));
        assert_token_value!("1!", Value::Integer(1));
        assert_token_value!("5!", Value::Integer(120));
        assert_token_value!("-5!", Value::Integer(-120));
        assert_token_value!("-~3!!", Value::Integer(-303));

        // Overflows and errors
        assert_token_error!("1/0", Overflow);
        assert_token_error!("5+5\n 1/0", Overflow);
        assert_token_error!("99999999999999999999999999999999999999999", ValueParsing);
        assert_token_error!("1+99999999999999999999999999999999999999999", ValueParsing);
        assert_token_error!("999999999999999999*999999999999999999", Overflow);
        assert_token_error!("999!", Overflow);

        // Ternary expression
        assert_token_value!("true ? 1 : 2", Value::Integer(1));
        assert_token_value!("false ? 1 : 2", Value::Integer(2));
        assert_token_value!("false ? 1/0 : 2", Value::Integer(2));

        // arrays
        assert_token_value!(
            "[10, 12] + [1.2, 1.3]",
            Value::Array(vec![Value::Float(11.2), Value::Float(13.3)])
        );
        assert_token_value!(
            "2 * [10, 5]",
            Value::Array(vec![Value::Integer(20), Value::Integer(10)])
        );
        assert_token_value!("[false, 0, true] == true", Value::Boolean(true));
    }
}
