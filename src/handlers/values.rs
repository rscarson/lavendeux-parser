use std::collections::HashMap;

use super::RuleHandler;
use crate::{
    state::ParserState,
    token::{OutputFormat, Rule, Token},
    value::ObjectType,
    Error, ExpectedTypes, FloatType, IntegerType, Value,
};

/// Parse a string as an integer of a given base
///
/// # Arguments
/// * `input` - Source string
/// * `prefix` - Number prefix to remove from the string
/// * `base` - Numeric base
fn parse_radix(
    input: &str,
    prefix: &[&str],
    base: u32,
) -> Result<IntegerType, std::num::ParseIntError> {
    let mut trimmed = input.to_string();
    for p in prefix {
        trimmed = trimmed.trim_start_matches(p).to_string();
    }

    IntegerType::from_str_radix(&trimmed, base)
}

pub fn handler_table() -> HashMap<Rule, RuleHandler> {
    HashMap::from([
        (Rule::atomic_value, rule_atomic_value as RuleHandler),
        (Rule::object, rule_object as RuleHandler),
        (Rule::array, rule_array as RuleHandler),
        (Rule::variable, rule_variable as RuleHandler),
        (Rule::string, rule_string as RuleHandler),
        (Rule::int, rule_int as RuleHandler),
        (Rule::currency, rule_currency as RuleHandler),
        (Rule::boolean, rule_boolean as RuleHandler),
        (Rule::float, rule_float as RuleHandler),
        (Rule::sci, rule_float as RuleHandler),
        (Rule::oct, rule_oct as RuleHandler),
        (Rule::bin, rule_bin as RuleHandler),
        (Rule::hex, rule_hex as RuleHandler),
        (Rule::index_expression, rule_index_expression as RuleHandler),
    ])
}

/// A single value
fn rule_atomic_value(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    token.set_value(token.child(0).unwrap().value());
    if matches!(token.value(), Value::None) {
        return Some(Error::VariableName {
            name: token.text().to_string(),
            token: token.clone(),
        });
    }
    None
}

/// Array value
/// [5,2,'test']
fn rule_array(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    let child_container = token.child(1).unwrap().clone();
    if matches!(child_container.rule(), Rule::expression_list) {
        token.set_value(Value::Array(
            child_container
                .children()
                .iter()
                .filter(|e| !matches!(e.rule(), Rule::comma))
                .map(|e| e.value())
                .collect::<Vec<Value>>(),
        ));
    } else if matches!(child_container.rule(), Rule::rbracket) {
        token.set_value(Value::Array(vec![]));
    } else {
        token.set_value(Value::Array(vec![child_container.value()]));
    }

    None
}

/// Object value
/// ['test': 1, 3: 5]
fn rule_object(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    let child_container = token.child(1).unwrap().clone();
    if matches!(child_container.rule(), Rule::property_list) {
        let mut object = ObjectType::new();
        let mut buffer: Vec<Value> = vec![];
        for child in child_container.children() {
            if child.text() == "," {
                object.insert(buffer[0].clone(), buffer[1].clone());
                buffer.clear();
            } else {
                buffer.push(child.value());
            }
        }

        if !buffer.is_empty() {
            object.insert(buffer[0].clone(), buffer[1].clone());
        }

        token.set_value(Value::Object(object));
    } else if matches!(child_container.rule(), Rule::rbrace) {
        token.set_value(Value::Object(HashMap::new()));
    }

    None
}

/// An identifier
/// x
/// pi
fn rule_variable(token: &mut Token, state: &mut ParserState) -> Option<Error> {
    if let Some(v) = state.constants.get(token.text()) {
        token.set_value(v.clone());
    } else if let Some(v) = state.variables.get(token.text()) {
        token.set_value(v.clone());
    } else {
        token.set_value(Value::Identifier(token.text().to_string()));
    }

    None
}

/// String value
/// "test"
/// 'test\n'
fn rule_string(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    // Remove the first and last characters - the quotes around our string
    // This would not work great with graphemes like é, but we know that it's
    // either ' or " so this should be safe
    let mut c = token.text().chars();
    c.next();
    c.next_back();

    // Now we split along our \\ backslash escapes, and rejoin after
    // to prevent going over them twice. This method isn't super
    // neat, there's likely a better way
    let string = c
        .as_str()
        .split("\\\\")
        .map(|s| {
            s.replace("\\'", "\'")
                .replace("\\\"", "\"")
                .replace("\\n", "\n")
                .replace("\\r", "\r")
                .replace("\\t", "\t")
        })
        .collect::<Vec<String>>()
        .join("\\");

    token.set_value(Value::String(string));
    None
}

/// Integer value
/// 10
/// 10,000
fn rule_int(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    match token.text().replace(',', "").parse::<IntegerType>() {
        Ok(n) => token.set_value(Value::Integer(n)),
        Err(e) => {
            return Some(Error::ValueParsing {
                input: e.to_string(),
                expected_type: ExpectedTypes::Int,
                token: token.clone(),
            });
        }
    }
    None
}

/// Currency value
/// <symbol><float>
fn rule_currency(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    for child in token.clone().children() {
        if child.rule() == Rule::currency_symbol {
            token.set_format(match child.text() {
                "$" => OutputFormat::Dollars,
                "€" => OutputFormat::Euros,
                "£" => OutputFormat::Pounds,
                "¥" => OutputFormat::Yen,
                &_ => return Some(Error::Internal(token.clone())),
            });
        } else {
            token.set_value(child.value());
        }
    }

    None
}

/// Boolean value
/// true
/// false
fn rule_boolean(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    if token.text().to_lowercase() == *"true" {
        token.set_value(Value::Boolean(true));
    } else if token.text().to_lowercase() == *"false" {
        token.set_value(Value::Boolean(false));
    }
    None
}

/// Floating point value
/// 8.3
/// 8.3e-10
fn rule_float(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    match token.text().replace(',', "").parse::<FloatType>() {
        Ok(n) => token.set_value(Value::Float(n)),
        Err(e) => {
            return Some(Error::ValueParsing {
                input: e.to_string(),
                expected_type: ExpectedTypes::Float,
                token: token.clone(),
            });
        }
    }
    None
}

/// Base 8 value
/// 0x77
fn rule_oct(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    match parse_radix(token.text(), &["0o", "0O"], 8) {
        Ok(n) => token.set_value(Value::Integer(n)),
        Err(e) => {
            return Some(Error::ValueParsing {
                input: e.to_string(),
                expected_type: ExpectedTypes::Int,
                token: token.clone(),
            });
        }
    }
    None
}

/// Base 2 value
/// 0b11
fn rule_bin(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    match parse_radix(token.text(), &["0b", "0B"], 2) {
        Ok(n) => token.set_value(Value::Integer(n)),
        Err(e) => {
            return Some(Error::ValueParsing {
                input: e.to_string(),
                expected_type: ExpectedTypes::Int,
                token: token.clone(),
            });
        }
    }
    None
}

/// Base 16 value
/// 0xFF
fn rule_hex(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    match parse_radix(token.text(), &["0x", "0X"], 16) {
        Ok(n) => token.set_value(Value::Integer(n)),
        Err(e) => {
            return Some(Error::ValueParsing {
                input: e.to_string(),
                expected_type: ExpectedTypes::Int,
                token: token.clone(),
            });
        }
    }
    None
}

/// indexing operator
/// x[5]
fn rule_index_expression(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    let mut source = token.child(0).unwrap().value();
    for child in token.children().iter().skip(2) {
        if child.rule() == Rule::lbracket || child.rule() == Rule::rbracket {
            continue;
        }

        let index = child.value();
        match source {
            Value::Object(v) => match v.get(&index) {
                Some(v) => source = v.clone(),
                None => {
                    return Some(Error::Index {
                        key: index,
                        token: token.clone(),
                    })
                }
            },

            _ => match index.as_int() {
                Some(i) => {
                    let array = source.as_array();
                    if i as usize >= array.len() || i < 0 {
                        return Some(Error::Index {
                            key: index,
                            token: token.clone(),
                        });
                    }

                    source = array[i as usize].clone();
                }
                None => {
                    return Some(Error::ValueType {
                        value: index,
                        expected_type: ExpectedTypes::Int,
                        token: token.clone(),
                    })
                }
            },
        }
    }

    token.set_value(source);
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
        assert_eq!(
            Value::Integer(255),
            Token::new("0xFF", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Integer(255),
            Token::new("0XFF", &mut state).unwrap().value()
        );
    }

    #[test]
    fn test_value_handler_bin() {
        let mut state = ParserState::new();
        assert_eq!(
            Value::Integer(3),
            Token::new("0b11", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Integer(3),
            Token::new("0B11", &mut state).unwrap().value()
        );
    }

    #[test]
    fn test_value_handler_oct() {
        let mut state = ParserState::new();
        assert_eq!(
            Value::Integer(7),
            Token::new("07", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Integer(7),
            Token::new("0o7", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Integer(7),
            Token::new("0O7", &mut state).unwrap().value()
        );
    }

    #[test]
    fn test_value_handler_sci() {
        let mut state = ParserState::new();
        assert_eq!(
            Value::Float(5.0),
            Token::new("5e+0", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Float(50.0),
            Token::new("5e+1", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Float(50.0),
            Token::new("5e1", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Float(52.0),
            Token::new("5.2e+1", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Float(0.52),
            Token::new("5.2e-1", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Float(0.020000000000000004),
            Token::new("1e-1.2", &mut state).unwrap().value()
        );
    }

    #[test]
    fn test_value_handler_float() {
        let mut state = ParserState::new();
        assert_eq!(
            Value::Float(10000.0),
            Token::new("10,000.0", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Float(1.0),
            Token::new("1.00000", &mut state).unwrap().value()
        );
    }

    #[test]
    fn test_value_handler_boolean() {
        let mut state = ParserState::new();
        assert_eq!(
            Value::Boolean(true),
            Token::new("true", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Boolean(false),
            Token::new("false", &mut state).unwrap().value()
        );
    }

    #[test]
    fn test_value_handler_currency() {
        let mut state = ParserState::new();
        assert_eq!(
            Value::Float(10000.0),
            Token::new("$10,000.00", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Float(1.0),
            Token::new("$1.0", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Integer(1),
            Token::new("£1", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Integer(1),
            Token::new("€1", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Integer(1),
            Token::new("¥1", &mut state).unwrap().value()
        );
    }

    #[test]
    fn test_value_handler_int() {
        let mut state = ParserState::new();
        assert_eq!(
            Value::Integer(10000),
            Token::new("10,000", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Integer(99),
            Token::new("99", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Integer(0),
            Token::new("0", &mut state).unwrap().value()
        );
    }

    #[test]
    fn test_value_handler_string() {
        let mut state = ParserState::new();
        assert_eq!(
            Value::String("".to_string()),
            Token::new("''", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::String("\"".to_string()),
            Token::new("'\"'", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::String("'".to_string()),
            Token::new("\"'\"", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::String("test".to_string()),
            Token::new("'test'", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::String("test".to_string()),
            Token::new("\"test\"", &mut state).unwrap().value()
        );
    }

    #[test]
    fn test_string_escapes() {
        let mut state = ParserState::new();
        assert_eq!(
            Value::String("\"".to_string()),
            Token::new("'\\\"'", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::String("\'".to_string()),
            Token::new("'\\''", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::String("\\".to_string()),
            Token::new("'\\\\'", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::String("\n".to_string()),
            Token::new("'\\n'", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::String("\n".to_string()),
            Token::new("'\\n'", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::String("\r".to_string()),
            Token::new("'\\r'", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::String("\t".to_string()),
            Token::new("'\\t'", &mut state).unwrap().value()
        );
    }

    #[test]
    fn test_value_handler_identifier() {
        let mut state = ParserState::new();
        Token::new("x=4", &mut state).unwrap();
        assert_eq!(
            Value::Integer(4),
            Token::new("x", &mut state).unwrap().value()
        );
        assert_eq!(true, Token::new("y + 1", &mut state).is_err());
    }

    #[test]
    fn test_value_handler_array() {
        let mut state = ParserState::new();
        assert_eq!(
            Value::Array(vec![
                Value::Integer(5),
                Value::Float(2.0),
                Value::String("test".to_string())
            ]),
            Token::new("[5, 2.0, 'test']", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Array(vec![Value::Integer(5)]),
            Token::new("[5]", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Array(vec![]),
            Token::new("[]", &mut state).unwrap().value()
        );
    }

    #[test]
    fn test_rule_index_expression() {
        let mut state = ParserState::new();
        Token::new("array = [1,2,3]", &mut state).unwrap();
        assert_eq!(
            Value::Integer(3),
            Token::new("array[2]", &mut state).unwrap().value()
        );
        assert_eq!(true, Token::new("array[-1]", &mut state).is_err());
        assert_eq!(true, Token::new("array['test']", &mut state).is_err());
        assert_eq!(true, Token::new("array[3]", &mut state).is_err());
    }
}
