use crate::{
    state::{ParserState, UserFunction},
    token::{LavendeuxHandler, OutputFormat, Rule, Token},
    Error, ExpectedTypes, Value,
};
use std::collections::HashMap;

mod utils;
use utils::*;

// Handlers
mod bitwise;
mod boolean;
mod errors;
mod functions;
mod math;
mod values;

#[derive(Default)]
pub struct Handler {}
impl LavendeuxHandler for Handler {
    fn handle_tree(&self, token: &mut Token, state: &mut ParserState) -> Result<(), Error> {
        // Ternary expression handler - enables short-circuit interpretation
        if token.rule() == Rule::ternary_expression {
            let condition = token.mut_child(0).unwrap();
            self.handle_tree(condition, state)?;

            let path_index = if condition.value().as_bool() { 1 } else { 2 };
            self.handle_tree(token.mut_child(path_index).unwrap(), state)?;

            let child = token.child(path_index).unwrap().clone();
            token.set_format(child.format());
            token.set_text(child.text());
            token.set_value(child.value());
            return Ok(());
        }

        // Function assignment handler - prevents prematurely executing the new function
        if token.rule() == Rule::function_assignment {
            let name = token.children().first().unwrap().text();
            let definition = token.children().last().unwrap().text();

            // Compile arguments
            let mut arguments: Vec<String> = Vec::new();
            for child in token.children().iter().skip(2) {
                let s = child.text();
                if s == "," {
                    continue;
                }
                if s == ")" {
                    break;
                }
                arguments.push(s.to_string());
            }

            // Store new function
            state.user_functions.insert(
                name.to_string(),
                UserFunction::new(name.to_string(), arguments, definition.to_string()),
            );

            let def = token.children().last().unwrap().clone();
            token.set_text(def.text());
            token.set_value(Value::String(def.text().to_string()));
            return Ok(());
        }

        // Handle child nodes
        for child in token.mut_children() {
            self.handle_tree(child, state)?;
        }

        // Check for unresolve identifier errors
        for child in token.children() {
            if child.value().is_identifier() {
                // Help function is allowed to have an unresolved identifier
                if !(token.rule() == Rule::call_expression
                    && token.child(0).unwrap().text() == "help")
                {
                    return Err(Error::VariableName {
                        name: child.text().to_string(),
                        token: child.clone(),
                    });
                }
            }
        }

        // Bubble up output format from children
        let format = token.children().iter().fold(OutputFormat::Default, |a, f| {
            if f.format() as i32 / 10 > a as i32 / 10 {
                f.format()
            } else {
                a
            }
        });
        token.set_format(format);

        // Get handler from table
        if let Some(f) = handler_table().get(&token.rule()) {
            if let Some(e) = f(token, state) {
                return Err(e);
            }
        }

        Ok(())
    }
}

type RuleHandler = fn(token: &mut Token, state: &mut ParserState) -> Option<Error>;
fn handler_table() -> HashMap<Rule, RuleHandler> {
    HashMap::from([
        (Rule::script, rule_script as RuleHandler),
        (Rule::line, rule_line as RuleHandler),
        (Rule::term, rule_term as RuleHandler),
        (
            Rule::assignment_expression,
            rule_assignment_expression as RuleHandler,
        ),
    ])
    .into_iter()
    .chain(values::handler_table())
    .chain(functions::handler_table())
    .chain(bitwise::handler_table())
    .chain(boolean::handler_table())
    .chain(math::handler_table())
    .chain(errors::handler_table())
    .collect()
}

/// A series of lines
fn rule_script(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    // Concatenate output from all child tokens (lines)
    token.set_text(
        &token
            .children()
            .iter()
            .map(|t| {
                t.text().to_string()
                    + if !t.children().is_empty() {
                        t.children().last().unwrap().text()
                    } else {
                        ""
                    }
            })
            .collect::<Vec<String>>()
            .join(""),
    );

    // Set script value if there is only one line
    if token.children().len() == 1 {
        token.set_value(token.child(0).unwrap().value());
    }

    None
}

/// One line in a script
fn rule_line(token: &mut Token, state: &mut ParserState) -> Option<Error> {
    // Bubble up child value and output format
    token.set_value(token.child(0).unwrap().value());
    if matches!(token.format(), OutputFormat::Unknown) {
        token.set_format(token.child(0).unwrap().format());
    }

    // Get decorator name
    let decorator_name = if token.children().len() > 2 {
        token.child(2).unwrap().text()
    } else if matches!(token.format(), OutputFormat::Dollars) {
        "dollars"
    } else if matches!(token.format(), OutputFormat::Euros) {
        "euros"
    } else if matches!(token.format(), OutputFormat::Pounds) {
        "pounds"
    } else if matches!(token.format(), OutputFormat::Yen) {
        "yen"
    } else {
        "default"
    };

    // Run specified decorator
    match state.decorators.call(decorator_name, token, &token.value()) {
        Ok(s) => token.set_text(&s),
        Err(e) => {
            // Extension decorators
            #[cfg(feature = "extensions")]
            if state.extensions.has_decorator(decorator_name) {
                match state
                    .extensions
                    .call_decorator(decorator_name, token, &mut state.variables)
                {
                    Ok(s) => {
                        token.set_text(&s);
                        return None;
                    }
                    Err(e) => return Some(e),
                }
            }

            return Some(e);
        }
    }

    if token.value().is_identifier() {
        let token = token.child(0).unwrap();
        return Some(Error::VariableName {
            name: token.text().to_string(),
            token: token.clone(),
        });
    }

    None
}

/// Term
/// expression
/// ( expression )
fn rule_term(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    // Unwrap parentheses if needed
    if token.children().len() == 3 {
        token.set_value(token.child(1).unwrap().value());
    }

    None
}

/// Assignment expression
/// identifier[index] = expression
/// identifier = expression
fn rule_assignment_expression(token: &mut Token, state: &mut ParserState) -> Option<Error> {
    if token.child(0).unwrap().rule() == Rule::index_assignment_prefix {
        rule_assignment_expression_indexed(token, state)
    } else {
        rule_assignment_expression_variable(token, state)
    }
}

/// Array indexed assignment expressions
/// identifier[index] = expression
fn rule_assignment_expression_indexed(token: &mut Token, state: &mut ParserState) -> Option<Error> {
    let prefix = token.child(0).unwrap().clone();
    let identifier = prefix.child(0).unwrap().text();
    let index = prefix.child(2).unwrap().value();
    let result = token.children().last().unwrap().value();

    if let Some(value) = state.variables.clone().get(identifier) {
        match value.clone() {
            Value::Object(mut v) => {
                v.insert(index, result.clone());
                state
                    .variables
                    .insert(identifier.to_string(), Value::Object(v));
                token.set_value(result);
            }

            _ => {
                match index.as_int() {
                    Some(i) => {
                        let mut array = value.as_array();
                        if i as usize > array.len() || i < 0 {
                            return Some(Error::Index {
                                key: index,
                                token: token.clone(),
                            });
                        }

                        // Update array
                        if i as usize == array.len() {
                            array.insert(i as usize, result.clone());
                        } else {
                            array[i as usize] = result.clone();
                        }

                        state
                            .variables
                            .insert(identifier.to_string(), Value::Array(array));
                        token.set_value(result);
                    }
                    None => {
                        return Some(Error::ValueType {
                            value: index,
                            expected_type: ExpectedTypes::Int,
                            token: token.clone(),
                        })
                    }
                }
            }
        }
    }

    None
}

/// Variable assignment expressions
/// identifier = expression
fn rule_assignment_expression_variable(
    token: &mut Token,
    state: &mut ParserState,
) -> Option<Error> {
    let identifier = token.child(0).unwrap().child(0).unwrap();

    if state.constants.contains_key(identifier.text()) {
        // Cannot overwrite constant
        return Some(Error::ConstantValue {
            name: identifier.text().to_string(),
            token: token.clone(),
        });
    } else {
        // Update value
        state.variables.insert(
            identifier.text().to_string(),
            token.child(1).unwrap().value(),
        );
        token.set_value(token.child(1).unwrap().value());
    }

    None
}
