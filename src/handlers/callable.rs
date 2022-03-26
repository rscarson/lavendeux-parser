use crate::token::{Rule, Token};
use crate::value::{Value};
use crate::state::ParserState;
use crate::errors::*;

pub fn call_expression_handler(token: &mut Token, state: &mut ParserState) -> Option<ParserError> {
    if token.rule() == Rule::call_expression {
        let name = token.child(0).unwrap().text();
        let mut args : Vec<Value> = Vec::new();
        match token.child(2).unwrap().rule() {
            Rule::rparen => { },
            Rule::expression_list => {
                let mut i = 0;
                while i < token.child(2).unwrap().children().len() {
                    args.push(token.child(2).unwrap().child(i).unwrap().value());
                    i += 2;
                }
            },
            _ => args.push(token.child(2).unwrap().value())
        }

        if state.functions.has(name) {
            // Builtin functions
            match state.functions.call(name, &args[..]) {
                Ok(v) => {
                    token.set_value(v);
                    return None;
                },
                Err(e) => { return Some(e); }
            }
        } else {
            // Extension functions
            if state.extensions.has_function(name) {
                match state.extensions.call_function(name, &args[..]) {
                    Ok(v) => {
                        token.set_value(v);
                        return None;
                    },
                    Err(e) => return Some(e)
                }
            }

            // User-defined functions
            if let Some(f) = state.user_functions.get(name) {
                if args.len() != f.arguments.len() {
                    return Some(ParserError::FunctionNArg(FunctionNArgError::new(&f.name, f.arguments.len(), f.arguments.len())));
                }

                if let Some(mut inner_state) = state.spawn_inner() {
                    // Populate arguments
                    for (i, arg) in f.arguments.clone().into_iter().enumerate() {
                        inner_state.variables.insert(arg, args[i].clone());
                    }

                    // Run the function as an expression
                    match Token::new(&f.definition, &mut inner_state) {
                        Ok(t) => {
                            token.set_value(t.child(0).unwrap().value());
                            token.set_text(t.text());
                            return None;
                        },
                        Err(e) => { return Some(e); }
                    }
                } else {
                    return Some(ParserError::Stack(StackError::new()));
                }
            }
        }

        return Some(ParserError::FunctionName(FunctionNameError::new(name)));
    }

    None
}