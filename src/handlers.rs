use crate::token::{Rule, Token, OutputFormat};
use crate::state::ParserState;
use crate::errors::*;

mod boolean;
pub use boolean::*;

mod math;
pub use math::*;

mod callable;
pub use callable::*;

mod values;
pub use values::*;

fn expression_handler(token: &mut Token, state: &mut ParserState) -> Option<ParserError> {
    match token.rule {
        Rule::script => {
            token.text = token.children.clone().into_iter().map(|t| t.text).collect::<Vec<String>>().join("");

            if token.children.len() == 1 {
                token.value = token.children[0].value.clone();
            }
        },

        Rule::line => {
            token.value = token.children[0].value.clone();
            if matches!(token.format, OutputFormat::Unknown) {
                token.format = token.children[0].format.clone();
            }
            
            if token.children.len() > 2 {
                let name = &token.children[2].text;
                match state.decorators.call(name, &token.value) {
                    Ok(s) => token.text = s,
                    Err(e) => {
                        for extension in &mut state.extensions {
                            if extension.has_decorator(name) {
                                match extension.call_decorator(name, &token.value) {
                                    Ok(s) => {
                                        token.text = s;
                                        return None;
                                    },
                                    Err(e) => return Some(e)
                                }
                            }
                        }

                        return Some(e);
                    }
                }
            } else {
                match token.format {
                    OutputFormat::Dollars => match state.decorators.call("dollars", &token.value) {
                        Ok(s) => token.text = s,
                        Err(e) => return Some(e)
                    },
                    _ => {
                        match state.decorators.call("default", &token.value) {
                            Ok(s) => token.text = s,
                            Err(e) => return Some(e)
                        }
                    }
                }
            }

            token.text = token.text.clone() + &token.children.last().unwrap().text;
        },

        Rule::term => {
            if token.children.len() == 3 {
                token.value = token.children[1].value.clone();
            }
        },

        Rule::assignment_expression => {
            if state.constants.contains_key(&token.children[0].text.to_string()) {
                return Some(ParserError::ContantValue(ConstantValueError {
                    name: token.children[0].text.clone()
                }))
            } else {
                state.variables.insert(token.children[0].text.to_string(), token.children[2].value.clone());
                token.value = token.children[2].value.clone();
            }
        },

        _ => { }
    }

    None
}


pub fn handler(token: &mut Token, state: &mut ParserState) -> Option<ParserError> {
    // Bubble up output format
    for child in token.children.clone() {
        if child.format.clone() as i32 / 10 > token.format.clone() as i32 / 10 {
            token.format = child.format.clone();
        }
    }

    if let Some(e) = atomicvalue_handler(token, state) {
       return Some(e);
    }

    if let Some(e) = expression_handler(token, state) {
       return Some(e);
    }

    if let Some(e) = bool_expression_handler(token, state) {
       return Some(e);
    }

    if let Some(e) = call_expression_handler(token, state) {
       return Some(e);
    }

    if let Some(e) = bitwise_expression_handler(token, state) {
       return Some(e);
    }

    if let Some(e) = math_expression_handler(token, state) {
       return Some(e);
    }

    None
}