use crate::token::{Rule, Token};
use crate::value::{AtomicValue};
use crate::state::ParserState;
use crate::errors::*;

pub fn call_expression_handler(token: &mut Token, state: &mut ParserState) -> Option<ParserError> {
    if token.rule == Rule::call_expression {
        let name = token.children[0].text.to_string();
        let mut args : Vec<AtomicValue> = Vec::new();
        match token.children[2].rule {
            Rule::rparen => { },
            Rule::expression_list => {
                let mut i = 0;
                while i < token.children[2].children.len() {
                    args.push(token.children[2].children[i].value.clone());
                    i += 2;
                }
            },
            _ => args.push(token.children[2].value.clone())
        }

        if state.functions.has(&name) {
            // Builtin functions
            match state.functions.call(&name, &args[..]) {
                Ok(v) => {
                    token.value = v;
                    return None;
                },
                Err(e) => { return Some(e); }
            }
        } else {
            // Extension functions
            for extension in &mut state.extensions {
                if extension.has_function(&name) {
                    match extension.call_function(&name, &args[..]) {
                        Ok(v) => {
                            token.value = v;
                            return None;
                        },
                        Err(e) => return Some(e)
                    }
                }
            }

            // User-defined functions
            if let Some(f) = state.user_functions.get(&name) {
                let mut inner_state = state.clone();
                inner_state.depth = state.depth + 1;
                if args.len() != f.arguments.len() {
                    return Some(ParserError::FunctionNArg(FunctionNArgError::new(&f.name, f.arguments.len(), f.arguments.len())));
                } else if !inner_state.is_depth_ok() {
                    return Some(ParserError::Stack);
                }

                let mut i = 0;
                for arg in f.arguments.clone() {
                    inner_state.variables.insert(arg, args[i].clone());
                    i += 0;
                }

                match Token::new(&f.definition, &mut inner_state) {
                    Ok(t) => {
                        token.value = t.children[0].value.clone();
                        token.text = t.text;
                        return None;
                    },
                    Err(e) => { return Some(e); }
                }
            }
        }

        return Some(ParserError::FunctionName(FunctionNameError::new(&name)));
    }

    None
}