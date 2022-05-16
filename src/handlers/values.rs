use crate::token::{Rule, Token, OutputFormat};
use crate::value::{Value, IntegerType, FloatType};
use crate::state::ParserState;
use crate::errors::*;

/// Parse a string as an integer of a given base
/// 
/// # Arguments
/// * `input` - Source string
/// * `prefix` - Number prefix to remove from the string
/// * `base` - Numeric base
fn parse_radix(input: &str, prefix: &[&str], base: u32) -> Result<IntegerType, std::num::ParseIntError> {
    let mut trimmed = input.to_string();
    for p in prefix {
        trimmed = trimmed.trim_start_matches(p).to_string();
    }
    
    match IntegerType::from_str_radix(&trimmed, base) {
        Ok(n) => Ok(n),
        Err(e) => Err(e)
    }
}

pub fn value_handler(token: &mut Token, state: &mut ParserState) -> Option<ParserError> {
    match token.rule() {
        Rule::hex => {
            match parse_radix(token.text(), &["0x","0X"], 16) {
                Ok(n) => token.set_value(Value::Integer(n)),
                Err(e) => return Some(ParserError::ParseInt(ParseIntegerError::new_with_index(Some(token.index()), &e.to_string())))
            }
        },

        Rule::bin => {
            match parse_radix(token.text(), &["0b","0B"], 2) {
                Ok(n) => token.set_value(Value::Integer(n)),
                Err(e) => return Some(ParserError::ParseInt(ParseIntegerError::new_with_index(Some(token.index()), &e.to_string())))
            }
        },

        Rule::oct => {
            match parse_radix(token.text(), &["0o","0O"], 8) {
                Ok(n) => token.set_value(Value::Integer(n)),
                Err(e) => return Some(ParserError::ParseInt(ParseIntegerError::new_with_index(Some(token.index()), &e.to_string())))
            }
        },

        Rule::sci|Rule::float => match token.text().replace(',', "").parse::<FloatType>() {
            Ok(n) => token.set_value(Value::Float(n)),
            Err(e) => return Some(ParserError::ParseFloat(ParseFloatingPointError::new_with_index(Some(token.index()), &e.to_string()))),
        },

        Rule::boolean => {
            if token.text().to_lowercase() == *"true" {
                token.set_value(Value::Boolean(true));
            } else if token.text().to_lowercase() == *"false" {
                token.set_value(Value::Boolean(false));
            }
        },

        Rule::currency => match token.text().chars().skip(1).take(token.text().len()-1).collect::<String>().replace(',', "").parse::<FloatType>() {
            Ok(n) => {
                token.set_value(Value::Float(n));
                if token.text().starts_with('$') {
                    token.set_format(OutputFormat::Dollars);
                } else if token.text().starts_with('€') {
                    token.set_format(OutputFormat::Euros);
                } else if token.text().starts_with('£') {
                    token.set_format(OutputFormat::Pounds);
                } else if token.text().starts_with('¥') {
                    token.set_format(OutputFormat::Yen);
                }
            },
            Err(e) => return Some(ParserError::ParseFloat(ParseFloatingPointError::new_with_index(Some(token.index()), &e.to_string()))),
        },

        Rule::int => match token.text().replace(',', "").parse::<IntegerType>() {
            Ok(n) => token.set_value(Value::Integer(n)),
            Err(e) => return Some(ParserError::ParseInt(ParseIntegerError::new_with_index(Some(token.index()), &e.to_string()))),
        },

        Rule::string => {
            token.set_value(Value::String(
                token.text()[1..token.text().len()-1].to_string()
                .replace("\\'", "\'")
                .replace("\\\"", "\"")
                .replace("\\n", "\n")
                .replace("\\r", "\r")
                .replace("\\t", "\t")
            ));
        },

        Rule::identifier => {
            match state.constants.get(token.text()) {
                Some(v) => token.set_value(v.clone()),
                None => if let Some(v) = state.variables.get(token.text()) {
                    token.set_value(v.clone());
                }
            }
        },
        
        Rule::atomic_value => {
            token.set_value(token.child(0).unwrap().value());
            if matches!(token.value(), Value::None) {
                return Some(ParserError::VariableName(VariableNameError::new(token.text())));
            }
        },

        _ => { }
    }

    None
}

#[cfg(test)]
mod test_token {
    use super::*;

    #[test]
    fn test_parse_radix() {
        assert_eq!(15, parse_radix("0xF", &["0x", "0X"], 16).unwrap());
        assert_eq!(15, parse_radix("0XF", &["0x", "0X"], 16).unwrap());
        assert_eq!(3, parse_radix("0X11", &["0x", "0X"], 2).unwrap());
        assert_eq!(true, parse_radix("0b11", &["0x", "0X"], 2).is_err());
    }

    #[test]
    fn test_value_handler_hex() {
        let mut state = ParserState::new();
        assert_eq!(Value::Integer(255), Token::new("0xFF", &mut state).unwrap().value());
        assert_eq!(Value::Integer(255), Token::new("0XFF", &mut state).unwrap().value());
    }

    #[test]
    fn test_value_handler_bin() {
        let mut state = ParserState::new();
        assert_eq!(Value::Integer(3), Token::new("0b11", &mut state).unwrap().value());
        assert_eq!(Value::Integer(3), Token::new("0B11", &mut state).unwrap().value());
    }

    #[test]
    fn test_value_handler_oct() {
        let mut state = ParserState::new();
        assert_eq!(Value::Integer(7), Token::new("07", &mut state).unwrap().value());
        assert_eq!(Value::Integer(7), Token::new("0o7", &mut state).unwrap().value());
        assert_eq!(Value::Integer(7), Token::new("0O7", &mut state).unwrap().value());
    }

    #[test]
    fn test_value_handler_sci() {
        let mut state = ParserState::new();
        assert_eq!(Value::Float(5.0), Token::new("5e+0", &mut state).unwrap().value());
        assert_eq!(Value::Float(50.0), Token::new("5e+1", &mut state).unwrap().value());
        assert_eq!(Value::Float(50.0), Token::new("5e1", &mut state).unwrap().value());
        assert_eq!(Value::Float(52.0), Token::new("5.2e+1", &mut state).unwrap().value());
        assert_eq!(Value::Float(0.52), Token::new("5.2e-1", &mut state).unwrap().value());
        assert_eq!(Value::Float(0.020000000000000004), Token::new("1e-1.2", &mut state).unwrap().value());
    }

    #[test]
    fn test_value_handler_float() {
        let mut state = ParserState::new();
        assert_eq!(Value::Float(10000.0), Token::new("10,000", &mut state).unwrap().value());
        assert_eq!(Value::Float(1.0), Token::new("1.00000", &mut state).unwrap().value());
    }

    #[test]
    fn test_value_handler_boolean() {
        let mut state = ParserState::new();
        assert_eq!(Value::Boolean(true), Token::new("true", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new("false", &mut state).unwrap().value());
    }

    #[test]
    fn test_value_handler_currency() {
        let mut state = ParserState::new();
        assert_eq!(Value::Float(10000.0), Token::new("$10,000.00", &mut state).unwrap().value());
        assert_eq!(Value::Float(1.0), Token::new("$1.0", &mut state).unwrap().value());
        assert_eq!(Value::Float(1.0), Token::new("£1", &mut state).unwrap().value());
        assert_eq!(Value::Float(1.0), Token::new("€1", &mut state).unwrap().value());
        assert_eq!(Value::Float(1.0), Token::new("¥1", &mut state).unwrap().value());
    }

    #[test]
    fn test_value_handler_int() {
        let mut state = ParserState::new();
        assert_eq!(Value::Integer(10000), Token::new("10,000", &mut state).unwrap().value());
        assert_eq!(Value::Integer(99), Token::new("99", &mut state).unwrap().value());
        assert_eq!(Value::Integer(0), Token::new("0", &mut state).unwrap().value());
    }

    #[test]
    fn test_value_handler_string() {
        let mut state = ParserState::new();
        assert_eq!(Value::String("".to_string()), Token::new("''", &mut state).unwrap().value());
        assert_eq!(Value::String("\"".to_string()), Token::new("'\"'", &mut state).unwrap().value());
        assert_eq!(Value::String("'".to_string()), Token::new("\"'\"", &mut state).unwrap().value());
        assert_eq!(Value::String("test".to_string()), Token::new("'test'", &mut state).unwrap().value());
        assert_eq!(Value::String("test".to_string()), Token::new("\"test\"", &mut state).unwrap().value());
    }

    #[test]
    fn test_value_handler_identifier() {
        let mut state = ParserState::new();
        Token::new("x=4", &mut state).unwrap();
        assert_eq!(Value::Integer(4), Token::new("x", &mut state).unwrap().value());
        assert_eq!(Value::None, Token::new("y", &mut state).unwrap().value());
    }
}