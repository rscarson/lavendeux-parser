extern crate pest;
extern crate pest_derive;
use pest::Parser;
use pest_derive::Parser;

use super::errors::*;
use super::calculator;
use super::state::{ParserState, UserFunction};
use super::value::AtomicValue;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct CalcParser;

#[derive(Clone, Debug)]
pub enum OutputFormat {
    Default = 0,
    Dollars = 10, Euros = 11, Pounds = 12, Yen = 13
}

type TokenHandler = fn(&mut Token, &mut ParserState) -> Option<ParserError>;

#[derive(Clone)]
pub struct Token {
    pub rule: Rule,
    pub input: String,
    pub text: String,
    pub format: OutputFormat,
    pub value: AtomicValue,
    pub index: usize,
    pub children: Vec<Token>
}

impl Token {
    pub const DEFAULT_HANDLER : TokenHandler = calculator::handler;

    /// Parses an input string, and returns the resulting token tree
    /// 
    /// # Arguments
    /// * `input` - Source string
    /// * `state` - The current parser state
    pub fn new(input: &str, state: &mut ParserState) -> Result<Token, ParserError> {
        Self::new_with_handler(input, Self::DEFAULT_HANDLER, state)
    }

    /// Parses an input string, and returns the resulting token tree
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
                    None => return Ok(Token {
                        format: OutputFormat::Default,
                        text: "".to_string(),
                        input: "".to_string(),
                        value: AtomicValue::None,
                        children: Vec::new(),
                        index: 0,
                        rule: Rule::script
                    }),
                    Some(p) => return Token::from_pair(p, handler, state)
                }
            }
            
            Err(e) => return Err(ParserError::Pest(PestError::new(&e.to_string())))
        }
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
            format: OutputFormat::Default,
            value: AtomicValue::None,
            index: next_pair.as_span().start(),
            children: Vec::new()
        };        

        if token.rule == Rule::ternary_expression && children.len() > 1 {
            // Ternary expression handler - enables short-circuit interpretation
            let condition = Self::from_pair(children[0].clone(), handler, state)?;
            token = Self::from_pair(if condition.value.as_bool() { children[1].clone() } else { children[2].clone() }, handler, state)?;
        } else if children.len() > 0 && children[0].clone().as_rule() == Rule::function_assignment {
            // Function assignment handler - prevents prematurely executing the new function
            let mut function_children: Vec<_> = children[0].clone().into_inner().into_iter().collect();
            let name = function_children.first().unwrap().as_str().clone();
            let definition = function_children.last().unwrap().as_str().clone();

            // Compile arguments
            let mut args : Vec<String> = Vec::new();
            function_children.remove(0); function_children.remove(0);
            for argument in function_children {
                let s = argument.as_str();
                if s == ")" { break; }
                if s == "," { continue; }
                args.push(s.to_string());
            }

            // Store new function
            state.user_functions.insert(name.to_string(), UserFunction {
                name: name.to_string(),
                arguments: args,
                definition: definition.to_string()
            });

            let eol = children.last().unwrap().as_str();
            token.text = definition.to_string() + eol;
            token.value = AtomicValue::String(token.text.clone());
        } else {
            // Default token handler
            for child in children {
                let t = Self::from_pair(child, handler, state)?;
                token.children.push(t);
            }

            // Run token handler to get value
            match handler(&mut token, state) {
                Some(e) => return Err(e),
                None => {}
            }
        }

        Ok(token)
    }
}

#[cfg(test)]
mod test_token {
    use super::*;

    fn token_does_value_equal(input: &str, expected: AtomicValue, state: &mut ParserState) {
        let t = Token::new(input, state).unwrap();
        assert_eq!(expected, t.value);
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
        assert_eq!(AtomicValue::Integer(10), Token::new("5+5", &mut state).unwrap().value);
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
        token_does_value_equal("1,000e5", AtomicValue::Float(100000000.0), &mut state);
        token_does_value_equal(".4e5", AtomicValue::Float(40000.0), &mut state);
        token_does_value_equal("5e5", AtomicValue::Float(500000.0), &mut state);
        token_does_value_equal("5E5", AtomicValue::Float(500000.0), &mut state);
        token_does_value_equal("5e+5", AtomicValue::Float(500000.0), &mut state);
        token_does_value_equal("5e-5", AtomicValue::Float(5e-5), &mut state);

        // Float
        token_does_value_equal("10000000.00", AtomicValue::Float(10000000.0), &mut state);
        token_does_value_equal("$10,000,000.00", AtomicValue::Float(10000000.0), &mut state);
        token_does_value_equal("$10,000,000", AtomicValue::Float(10000000.0), &mut state);
        token_does_value_equal(".4", AtomicValue::Float(0.4), &mut state);
        token_does_value_equal("4.4", AtomicValue::Float(4.4), &mut state);

        // Int
        token_does_value_equal("1,000", AtomicValue::Integer(1000), &mut state);
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

        token_does_text_equal("5+5\n5+5", "10\n10", &mut state);

        // Empty lines and comments
        token_does_text_equal("5+5\n\n\n// Test\n5+5 // test", "10\n\n\n\n10", &mut state);

        // Line
        token_does_value_equal("5", AtomicValue::Integer(5), &mut state);
        token_does_text_equal("5 @bin", "0b101", &mut state);
        token_does_text_equal("5 @int", "5", &mut state);

        // Comments
        token_does_value_equal("5 //test", AtomicValue::Integer(5), &mut state);
        token_does_value_equal("//test", AtomicValue::None, &mut state);
        
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

        // Ternary expression
        token_does_value_equal("true ? 1 : 2", AtomicValue::Integer(1), &mut state);
        token_does_value_equal("false ? 1 : 2", AtomicValue::Integer(2), &mut state);
        token_does_value_equal("false ? 1/0 : 2", AtomicValue::Integer(2), &mut state);

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
        token_does_text_equal("2*$2", "$4.00", &mut state);
        token_does_value_equal("2+2", AtomicValue::Integer(4), &mut state);
        token_does_value_equal("2+2+2", AtomicValue::Integer(6), &mut state);
        token_does_value_equal("2+2-2/2", AtomicValue::Integer(3), &mut state);

        // shift expression
        token_does_value_equal("2<<2", AtomicValue::Integer(8), &mut state);
        token_does_value_equal("2<<2>>2", AtomicValue::Integer(2), &mut state);
        token_does_value_equal("2<<2>>(2+1)", AtomicValue::Integer(1), &mut state);

        // bitwise expressions
        token_does_value_equal("0b1100 & 0b0011", AtomicValue::Integer(0), &mut state);
        token_does_value_equal("0b1100 | 0b0011", AtomicValue::Integer(15), &mut state);
        token_does_value_equal("0b1110 ^ 0b0011", AtomicValue::Integer(13), &mut state);
        token_does_value_equal("0b0001 | 0b0011 ^ 0b1111", AtomicValue::Integer(13), &mut state);

        // boolean expressions
        token_does_value_equal("2 < 3", AtomicValue::Boolean(true), &mut state);
        token_does_value_equal("1 > 2 < true", AtomicValue::Boolean(true), &mut state);
        token_does_value_equal("5.0 < 3", AtomicValue::Boolean(false), &mut state);
        token_does_value_equal("'test' > 'a'", AtomicValue::Boolean(true), &mut state);
        token_does_value_equal("'test' && 'a'", AtomicValue::Boolean(true), &mut state);
        token_does_value_equal("true && true && false", AtomicValue::Boolean(false), &mut state);
        token_does_value_equal("1 && 1", AtomicValue::Boolean(true), &mut state);
        token_does_value_equal("1 && 0", AtomicValue::Boolean(false), &mut state);
        token_does_value_equal("false || false || true", AtomicValue::Boolean(true), &mut state);
        token_does_value_equal("true || false", AtomicValue::Boolean(true), &mut state);
        token_does_value_equal("true == false", AtomicValue::Boolean(false), &mut state);
        token_does_value_equal("true == false != true", AtomicValue::Boolean(true), &mut state);

        // Function
        let t = Token::new("5+5\nfn(x, y) = x * y\n5+5", &mut state).unwrap();
        assert_eq!("10\nx * y\n10", t.text);
        token_does_value_equal("fn(5,5)", AtomicValue::Integer(25), &mut state);
        assert_eq!(true, Token::new("f(x) = f(x)\nf(0)", &mut state).is_err());
    }
}