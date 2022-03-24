use crate::token::{Rule, Token, OutputFormat};
use crate::value::{AtomicValue, IntegerType, FloatType};
use crate::state::ParserState;
use crate::errors::*;

/// Parse a string as an integer of a given base
/// 
/// # Arguments
/// * `input` - Source string
/// * `prefix` - Number prefix to remove from the string
/// * `base` - Numeric base
fn parse_radix(input: &str, prefix: &[&str], base: u32) -> Result<IntegerType, ParserError> {
    let mut trimmed = input.to_string();
    for p in prefix {
        trimmed = trimmed.trim_start_matches(p).to_string();
    }
    
    match IntegerType::from_str_radix(&trimmed, base) {
        Ok(n) => Ok(n),
        Err(e) => Err(ParserError::ParseInt(e))
    }
}

pub fn atomicvalue_handler(token: &mut Token, state: &mut ParserState) -> Option<ParserError> {
    match token.rule {
        Rule::hex => {
            match parse_radix(&token.text, &["0x","0X"], 16) {
                Ok(n) => token.value = AtomicValue::Integer(n),
                Err(e) => return Some(e)
            }
        },

        Rule::bin => {
            match parse_radix(&token.text, &["0b","0B"], 2) {
                Ok(n) => token.value = AtomicValue::Integer(n),
                Err(e) => return Some(e)
            }
        },

        Rule::oct => {
            match parse_radix(&token.text, &["0o","0O"], 8) {
                Ok(n) => token.value = AtomicValue::Integer(n),
                Err(e) => return Some(e)
            }
        },

        Rule::sci|Rule::float => match token.text.replace(',', "").parse::<FloatType>() {
            Ok(n) => token.value = AtomicValue::Float(n),
            Err(e) => return Some(ParserError::ParseFloat(e)),
        },

        Rule::boolean => {
            if token.text.to_lowercase() == *"true" {
                token.value = AtomicValue::Boolean(true);
            } else if token.text.to_lowercase() == *"false" {
                token.value = AtomicValue::Boolean(false);
            }
        },

        Rule::currency => match token.text.chars().skip(1).take(token.text.len()-1).collect::<String>().replace(',', "").parse::<FloatType>() {
            Ok(n) => {
                token.value = AtomicValue::Float(n);
                if token.text.starts_with('$') {
                    token.format = OutputFormat::Dollars;
                } else if token.text.starts_with('€') {
                    token.format = OutputFormat::Euros;
                } else if token.text.starts_with('£') {
                    token.format = OutputFormat::Pounds;
                } else if token.text.starts_with('¥') {
                    token.format = OutputFormat::Yen;
                }
            },
            Err(e) => return Some(ParserError::ParseFloat(e)),
        },

        Rule::int => match token.text.replace(',', "").parse::<IntegerType>() {
            Ok(n) => token.value = AtomicValue::Integer(n),
            Err(e) => return Some(ParserError::ParseInt(e)),
        },

        Rule::string => {
            token.value = AtomicValue::String(
                token.text[1..token.text.len()-1].to_string()
                .replace("\\'", "\'")
                .replace("\\\"", "\"")
                .replace("\\n", "\n")
                .replace("\\r", "\r")
                .replace("\\t", "\t")
            );
        },

        Rule::identifier => {
            match state.constants.get(&token.text) {
                Some(v) => token.value = v.clone(),
                None => if let Some(v) = state.variables.get(&token.text) {
                    token.value = v.clone();
                }
            }
        },
        
        Rule::atomic_value => {
            token.value = token.children[0].value.clone();
            if matches!(token.value, AtomicValue::None) {
                return Some(ParserError::VariableName(VariableNameError {
                    name: token.text.clone()
                }));
            }
        },

        _ => { }
    }

    None
}