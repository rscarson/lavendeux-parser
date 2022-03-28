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
    match token.rule() {
        Rule::script => {
            token.set_text(
                &token.children().iter().map(|t| {
                    t.text().to_string() + if !t.children().is_empty() {
                        t.children().last().unwrap().text()
                    } else { "" }
                }).collect::<Vec<String>>().join("")
            );

            if token.children().len() == 1 {
                token.set_value(token.child(0).unwrap().value());
            }
        },

        Rule::line => {
            // Bubble up child parameters
            token.set_value(token.child(0).unwrap().value());
            if matches!(token.format(), OutputFormat::Unknown) {
                token.set_format(token.child(0).unwrap().format());
            }
            
            if token.children().len() > 2 {
                // Run specified decorators
                let name = &token.child(2).unwrap().text();
                match state.decorators.call(name, &token.value()) {
                    Ok(s) => token.set_text(&s),
                    Err(e) => {
                        // Extension decorators
                        if state.extensions.has_decorator(name) {
                            match state.extensions.call_decorator(name, &token.value()) {
                                Ok(s) => {
                                    token.set_text(&s);
                                    return None;
                                },
                                Err(e) => return Some(e)
                            }
                        }

                        return Some(e);
                    }
                }
            } else {
                // Run default decorator
                match token.format() {
                    OutputFormat::Dollars => match state.decorators.call("dollars", &token.value()) {
                        Ok(s) => token.set_text(&s),
                        Err(e) => return Some(e)
                    },
                    _ => {
                        match state.decorators.call("default", &token.value()) {
                            Ok(s) => token.set_text(&s),
                            Err(e) => return Some(e)
                        }
                    }
                }
            }
        },

        Rule::term => {
            if token.children().len() == 3 {
                token.set_value(token.child(1).unwrap().value());
            }
        },

        Rule::assignment_expression => {
            if state.constants.contains_key(token.child(0).unwrap().text()) {
                return Some(ParserError::ContantValue(ConstantValueError::new(token.child(0).unwrap().text().to_string())))
            } else {
                state.variables.insert(token.child(0).unwrap().text().to_string(), token.child(2).unwrap().value());
                token.set_value(token.child(2).unwrap().value());
            }
        },

        _ => { }
    }

    None
}


pub fn handler(token: &mut Token, state: &mut ParserState) -> Option<ParserError> {
    // Bubble up output format
    let format = token.children().iter().fold(OutputFormat::Default, |a,f| if f.format() as i32 / 10 > a as i32 / 10 {f.format()} else {a});
    token.set_format(format);

    if let Some(e) = value_handler(token, state) {
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