extern crate pest;
extern crate pest_derive;
use pest::Parser;
use pest_derive::Parser;

use super::errors::*;
use super::calculator;
use super::state::ParserState;
use super::value::AtomicValue;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct CalcParser;

type TokenHandler = fn(&mut Token, &mut ParserState) -> Option<ParserError>;

#[derive(Clone)]
pub struct Token {
    pub rule: Rule,
    pub input: String,
    pub text: String,
    pub value: AtomicValue,
    pub index: usize,
    pub children: Vec<Token>
}

impl Token {
    /// Converts a pest pair object into a token, and returns it
    /// 
    /// # Arguments
    /// * `pair` - A pair object returned by pest
    /// * `handler` - The handler function that receives the token
    pub fn new(pair: pest::iterators::Pair<Rule>, handler: TokenHandler, state: &mut ParserState) -> Result<Token, ParserError> {
        // Collect basic properties
        let rule = pair.as_rule();
        let text = pair.as_str();
        let span = pair.as_span();

        // Process token children
        let mut children : Vec<Token> = Vec::new();
        for child in pair.into_inner() {
            let child_token = Token::new(child, handler, state);
            match child_token {
                Err(e) => return Err(e),
                Ok(mut c) => {
                    match handler(&mut c, state) {
                        Some(e) => return Err(e),
                        None => {}
                    }

                    children.push(c);
                }
            }
        }
        
        // Create token
        let mut token = Token {
            rule: rule,
            input: text.to_string(),
            text: text.to_string(),
            value: AtomicValue::None,
            index: span.start(),
            children: children
        };

        // Handle token
        match handler(&mut token, state) {
            Some(e) => return Err(e),
            None => return Ok(token)
        }
    }

    /// Parses an input string, and returns the resulting token
    /// 
    /// # Arguments
    /// * `input` - Source string
    /// * `handler` - The handler function that receives the token
    /// * `state` - The current parser state
    pub fn token_from_input(input: &str, handler: TokenHandler, state: &mut ParserState) -> Result<Token, ParserError> {
        let pairs = CalcParser::parse(Rule::script, input);
        match pairs {
            Ok(mut r) => {
                match r.next() {
                    None => return Ok(Token {
                        text: "".to_string(),
                        input: "".to_string(),
                        value: AtomicValue::None,
                        children: Vec::new(),
                        index: 0,
                        rule: Rule::script
                    }),
                    Some(p) => return Token::new(p, handler, state)
                }
            }
            
            Err(e) => return Err(ParserError::Pest(PestError::new(&e.to_string())))
        }
    }

    /// Parses an input string, and returns the resulting value
    /// 
    /// # Arguments
    /// * `input` - Source string
    /// * `state` - The current parser state
    pub fn from_input(input: &str, state: &mut ParserState) -> Result<Token, ParserError> {
        match Token::token_from_input(input, calculator::handler, state) {
            Ok(t) => Ok(t),
            Err(e) => Err(e)
        }
    }
}

#[cfg(test)]
mod test_token {
    use super::*;

    fn token_does_value_equal(input: &str, expected: AtomicValue, state: &mut ParserState) {
        assert_eq!(expected, Token::from_input(input, state).unwrap().children[0].value);
    }

    fn token_does_text_equal(input: &str, expected: &str, state: &mut ParserState) {
        assert_eq!(expected, Token::from_input(input, state).unwrap().children[0].text);
    }

    #[test]
    fn test_token_from_input() {
        let mut state: ParserState = ParserState::new();
        assert_eq!("5+5", Token::token_from_input("5+5", |_, _| None, &mut state).unwrap().text);
    }

    #[test]
    fn test_from_input() {
        let mut state: ParserState = ParserState::new();
        assert_eq!(AtomicValue::Integer(10), Token::from_input("5+5", &mut state).unwrap().children[0].value);
    }

    #[test]
    fn test_grammar_atomic_value() {
        let mut state: ParserState = ParserState::new();

        // Hex
        token_does_value_equal("0x0F", AtomicValue::Integer(15), &mut state);
        token_does_value_equal("0x0f", AtomicValue::Integer(15), &mut state);
        token_does_value_equal("0x0", AtomicValue::Integer(0), &mut state);
        token_does_value_equal("0XFF", AtomicValue::Integer(255), &mut state);

        // Bin
        token_does_value_equal("0b00", AtomicValue::Integer(0), &mut state);
        token_does_value_equal("0B11111111", AtomicValue::Integer(255), &mut state);

        // Oct
        token_does_value_equal("0o00", AtomicValue::Integer(0), &mut state);
        token_does_value_equal("0O777", AtomicValue::Integer(511), &mut state);

        // Oct
        token_does_value_equal("0o00", AtomicValue::Integer(0), &mut state);
        token_does_value_equal("0O777", AtomicValue::Integer(511), &mut state);

        // Sci
        token_does_value_equal(".4e5", AtomicValue::Float(40000.0), &mut state);
        token_does_value_equal("5e5", AtomicValue::Float(500000.0), &mut state);
        token_does_value_equal("5E5", AtomicValue::Float(500000.0), &mut state);
        token_does_value_equal("5e+5", AtomicValue::Float(500000.0), &mut state);
        token_does_value_equal("5e-5", AtomicValue::Float(5e-5), &mut state);

        // Float
        token_does_value_equal(".4", AtomicValue::Float(0.4), &mut state);
        token_does_value_equal("4.4", AtomicValue::Float(4.4), &mut state);

        // Int
        token_does_value_equal("999", AtomicValue::Integer(999), &mut state);
        token_does_value_equal("0", AtomicValue::Integer(0), &mut state);

        // String
        token_does_value_equal("'test'", AtomicValue::String("test".to_string()), &mut state);
        token_does_value_equal("       '  test   '       ", AtomicValue::String("  test   ".to_string()), &mut state);
        token_does_value_equal("'test\"'", AtomicValue::String("test\"".to_string()), &mut state);
        token_does_value_equal("'test\\\"'", AtomicValue::String("test\"".to_string()), &mut state);
        token_does_value_equal("\"test\\\'\"", AtomicValue::String("test\'".to_string()), &mut state);
        token_does_value_equal("\"test\\\'\"", AtomicValue::String("test\'".to_string()), &mut state);

        // Identifier
        state.variables.insert("x".to_string(), AtomicValue::Integer(99));
        state.variables.insert("x_9".to_string(), AtomicValue::Integer(99));
        token_does_value_equal("x", AtomicValue::Integer(99), &mut state);
        token_does_value_equal("x_9", AtomicValue::Integer(99), &mut state);
    }

    #[test]
    fn test_grammar_script() {
        let mut state: ParserState = ParserState::new();

        let token = Token::from_input("5+5\n5+5", &mut state).unwrap();
        assert_eq!("10\n10", token.text);

        // Line
        token_does_value_equal("5", AtomicValue::Integer(5), &mut state);
        token_does_text_equal("5 @bin", "0b101", &mut state);
        token_does_text_equal("5 @int", "5", &mut state);

        // Comments
        token_does_value_equal("5 //test", AtomicValue::Integer(5), &mut state);
        
        // Assignment expression
        token_does_value_equal("x = 5", AtomicValue::Integer(5), &mut state);
        assert_eq!(1, state.variables.len());

        // Term
        token_does_value_equal("(5)", AtomicValue::Integer(5), &mut state);
    }

    #[test]
    fn test_grammar_expression() {
        let mut state: ParserState = ParserState::new();

        // Unary expression
        token_does_value_equal("~0b101", AtomicValue::Integer(2), &mut state);
        token_does_value_equal("~0b11111111", AtomicValue::Integer(0), &mut state);
        token_does_value_equal("~0b0", AtomicValue::Integer(-1), &mut state);
        token_does_value_equal("-1", AtomicValue::Integer(-1), &mut state);
        token_does_value_equal("-0", AtomicValue::Integer(0), &mut state);
        token_does_value_equal("-1.1", AtomicValue::Float(-1.1), &mut state);
        token_does_value_equal("1!", AtomicValue::Integer(1), &mut state);
        token_does_value_equal("5!", AtomicValue::Integer(120), &mut state);
        token_does_value_equal("-5!", AtomicValue::Integer(-120), &mut state);
        token_does_value_equal("-~3!!", AtomicValue::Integer(-303), &mut state);

        // Call expression
        token_does_value_equal("sqrt(9)", AtomicValue::Integer(3), &mut state);
        token_does_value_equal("sqrt(9 | 5)", AtomicValue::Integer(3), &mut state);
        token_does_value_equal("root(9, 2)", AtomicValue::Integer(3), &mut state);

        // Power expression
        token_does_value_equal("2**2", AtomicValue::Integer(4), &mut state);
        token_does_value_equal("2**2**2", AtomicValue::Integer(16), &mut state);
        token_does_value_equal("2**2**(2|2)", AtomicValue::Integer(16), &mut state);

        // multiply / divide expression
        token_does_value_equal("2*2", AtomicValue::Integer(4), &mut state);
        token_does_value_equal("2*2*2", AtomicValue::Integer(8), &mut state);
        token_does_value_equal("2*2/(2|2)", AtomicValue::Integer(2), &mut state);

        // add / sub expression
        token_does_value_equal("2+2", AtomicValue::Integer(4), &mut state);
        token_does_value_equal("2+2+2", AtomicValue::Integer(6), &mut state);
        token_does_value_equal("2+2-2/2", AtomicValue::Integer(3), &mut state);

        // shift expression
        token_does_value_equal("2<2", AtomicValue::Integer(8), &mut state);
        token_does_value_equal("2<2>2", AtomicValue::Integer(2), &mut state);
        token_does_value_equal("2<2>(2+1)", AtomicValue::Integer(1), &mut state);

        // boolean expressions
        token_does_value_equal("0b1100 & 0b0011", AtomicValue::Integer(0), &mut state);
        token_does_value_equal("0b1100 | 0b0011", AtomicValue::Integer(15), &mut state);
        token_does_value_equal("0b1110 ^ 0b0011", AtomicValue::Integer(13), &mut state);
        token_does_value_equal("0b0001 | 0b0011 ^ 0b1111", AtomicValue::Integer(13), &mut state);
    }
}