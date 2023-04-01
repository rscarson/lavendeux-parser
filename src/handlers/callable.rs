use crate::token::{Rule, Token};
use crate::value::{Value};
use crate::state::{ParserState};
use crate::errors::*;

pub fn call_expression_handler(token: &mut Token, state: &mut ParserState) -> Option<ParserError> {
    if token.rule() == Rule::call_expression {
        // Get function name and arguments
        let name = &token.child(0).unwrap().text().to_string();
        let mut arg_tokens = Vec::<&Token>::new();

        let mut args : Vec<Value> = Vec::new();
        match token.child(2).unwrap().rule() {
            Rule::rparen => { },
            Rule::expression_list => {
                let mut i = 0;
                while i < token.child(2).unwrap().children().len() {
                    let t = token.child(2).unwrap().child(i).unwrap();
                    args.push(t.value());
                    arg_tokens.push(t);
                    i += 2;
                }
            },
            _ => {
                let t = token.child(2).unwrap();
                args.push(t.value());
                arg_tokens.push(t);
            }
        }

        // Builtin functions
        if state.functions.has(name) {
            let functions = state.functions.clone();
            let result = functions.call(name, state, &args[..]);
            match result {
                Ok(v) => {
                    token.set_value(v);
                    return None;
                },
                Err(e) => { return Some(e); }
            }
        }

        // Extension functions
        #[cfg(feature = "extensions")]
        if state.extensions.has_function(name) {
            match state.extensions.call_function(name, &args[..], &mut state.variables) {
                Ok(v) => {
                    token.set_value(v);
                    return None;
                },
                Err(e) => return Some(e)
            }
        }
        
        // User-defined functions
        if let Some(f) = state.user_functions.get(name) {
            if args.len() != f.arguments().len() {
                return Some(ParserError::FunctionNArg(FunctionNArgError::new_with_token(token, f.name(), f.arguments().len(), f.arguments().len())));
            }

            if let Some(mut inner_state) = state.spawn_inner() {
                // Populate arguments
                for (i, arg) in f.arguments().clone().into_iter().enumerate() {
                    inner_state.variables.insert(arg, args[i].clone());
                }

                // Run the function as an expression
                match Token::new(f.definition(), &mut inner_state) {
                    Ok(t) => {
                        token.set_value(t.value());
                        token.set_text(t.text());
                        return None;
                    },
                    Err(e) => { return Some(e); }
                }
            } else {
                return Some(ParserError::Stack(StackError::new_with_token(token)));
            }
        }

        return Some(ParserError::FunctionName(FunctionNameError::new_with_token(token, name)));
    }

    None
}

#[cfg(test)]
mod test_token {
    use super::*;

    #[test]
    fn test_builtin_function_call() {
        let mut state: ParserState = ParserState::new();
        
        assert_eq!(Value::Float(3.0), Token::new("sqrt(9)", &mut state).unwrap().value());
        assert_eq!(Value::Float(3.0), Token::new("sqrt(3*3)", &mut state).unwrap().value());
        assert_eq!(Value::Float(3.0), Token::new("root(9, 2)", &mut state).unwrap().value());
        assert_eq!(true, Token::new("rooplipp(9)", &mut state).is_err());
    }

    #[test]
    fn test_user_function_call() {
        let mut state: ParserState = ParserState::new();
        
        assert_eq!("10\nx * y\n10", Token::new("5+5\nfn(x, y) = x * y\n5+5", &mut state).unwrap().text());
        assert_eq!(Value::Integer(25), Token::new("fn(5,5)", &mut state).unwrap().value());
        assert_eq!("5x + 10(x * y)\n70", Token::new("fn(x, y) = 5x + 10(x * y)\nfn(2, 3)", &mut state).unwrap().text());
        assert_eq!(true, Token::new("f(x) = f(x)\nf(0)", &mut state).is_err());
    }
}