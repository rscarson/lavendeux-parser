use crate::token::{Rule, Token, OutputFormat};
use crate::value::AtomicValue;
use crate::state::ParserState;
use crate::errors::*;

pub fn bool_expression_handler(token: &mut Token, _state: &mut ParserState) -> Option<ParserError> {
    match token.rule {
        Rule::bool_cmp_expression => {
            let mut i = 0;
            token.value = token.children[i].value.clone();
            while i < token.children.len() - 2 {
                let l = token.value.clone();
                let r = token.children[i+2].value.clone();

                if l.is_string() && r.is_string() {
                    match token.children[i+1].rule {
                        Rule::lt => token.value = AtomicValue::Boolean(l.as_string() < r.as_string()),
                        Rule::gt => token.value = AtomicValue::Boolean(l.as_string() > r.as_string()),
                        Rule::eq => token.value = AtomicValue::Boolean(l.as_string() == r.as_string()),
                        Rule::ne => token.value = AtomicValue::Boolean(l.as_string() != r.as_string()),
                        _ => {}
                    }
                } else if l.is_bool() && r.is_bool() {
                    match token.children[i+1].rule {
                        Rule::lt => token.value = AtomicValue::Boolean(!l.as_bool() & r.as_bool()),
                        Rule::gt => token.value = AtomicValue::Boolean(l.as_bool() & !r.as_bool()),
                        Rule::eq => token.value = AtomicValue::Boolean(l.as_bool() == r.as_bool()),
                        Rule::ne => token.value = AtomicValue::Boolean(l.as_bool() != r.as_bool()),
                        _ => {}
                    }
                } else if l.is_int() && r.is_int() {
                    match token.children[i+1].rule {
                        Rule::lt => token.value = AtomicValue::Boolean(l.as_int().unwrap() < r.as_int().unwrap()),
                        Rule::gt => token.value = AtomicValue::Boolean(l.as_int().unwrap() > r.as_int().unwrap()),
                        Rule::eq => token.value = AtomicValue::Boolean(l.as_int().unwrap() == r.as_int().unwrap()),
                        Rule::ne => token.value = AtomicValue::Boolean(l.as_int().unwrap() != r.as_int().unwrap()),
                        _ => {}
                    }
                } else if l.is_numeric() && r.is_numeric() {
                    match token.children[i+1].rule {
                        Rule::lt => token.value = AtomicValue::Boolean(l.as_float().unwrap() < r.as_float().unwrap()),
                        Rule::gt => token.value = AtomicValue::Boolean(l.as_float().unwrap() > r.as_float().unwrap()),
                        Rule::eq => token.value = AtomicValue::Boolean(l.as_float().unwrap() == r.as_float().unwrap()),
                        Rule::ne => token.value = AtomicValue::Boolean(l.as_float().unwrap() != r.as_float().unwrap()),
                        _ => {}
                    }
                }

                i += 2;
            }

            token.format = OutputFormat::Default; // Revert to boolean type
        },
        
        Rule::bool_and_expression => {
            let mut i = 0;
            token.value = token.children[i].value.clone();
            while i < token.children.len() - 2 {
                token.value = AtomicValue::Boolean(token.value.as_bool() && token.children[i+2].value.as_bool());
                i += 2
            }

            token.format = OutputFormat::Default; // Revert to boolean type
        },
        
        Rule::bool_or_expression => {
            let mut i = 0;
            token.value = token.children[i].value.clone();
            while i < token.children.len() - 2 {
                token.value = AtomicValue::Boolean(token.value.as_bool() || token.children[i+2].value.as_bool());
                i += 2
            }

            token.format = OutputFormat::Default; // Revert to boolean type
        },

        _ => { }
    }

    None
}