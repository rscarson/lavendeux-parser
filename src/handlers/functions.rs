use std::collections::HashMap;

use super::RuleHandler;
use crate::{
    state::ParserState,
    token::{Rule, Token},
    Error, Value,
};

pub fn handler_table() -> HashMap<Rule, RuleHandler> {
    HashMap::from([(Rule::call_expression, rule_call_expression as RuleHandler)])
}

fn rule_call_expression(token: &mut Token, state: &mut ParserState) -> Option<Error> {
    // Get function name and arguments
    let name = &token.child(0).unwrap().text().to_string();
    let mut arg_tokens = Vec::<&Token>::new();

    let mut args: Vec<Value> = Vec::new();
    match token.child(2).unwrap().rule() {
        Rule::rparen => {}
        Rule::expression_list => {
            let mut i = 0;
            while i < token.child(2).unwrap().children().len() {
                let t = token.child(2).unwrap().child(i).unwrap();
                args.push(t.value());
                arg_tokens.push(t);
                i += 2;
            }
        }
        _ => {
            let t = token.child(2).unwrap();
            args.push(t.value());
            arg_tokens.push(t);
        }
    }

    // Extension functions
    #[cfg(feature = "extensions")]
    if state.extensions.has_function(name) {
        match state
            .extensions
            .call_function(name, token, &args, &mut state.variables)
        {
            Ok(v) => {
                token.set_value(v);
                return None;
            }
            Err(e) => return Some(e),
        }
    }

    // Builtin functions
    if state.functions.has(name) {
        let functions = state.functions.clone();
        match functions.call(name, token, state, &args) {
            Ok(v) => {
                token.set_value(v);
                return None;
            }
            Err(e) => return Some(e),
        }
    }

    // User functions
    if let Some(f) = state.user_functions.get(name) {
        if args.len() != f.arguments().len() {
            return Some(Error::FunctionArguments {
                min: f.arguments().len(),
                max: f.arguments().len(),
                signature: f.signature(),
                token: token.clone(),
            });
        } else if let Some(mut inner_state) = state.spawn_inner() {
            // Populate arguments
            for (i, arg) in f.arguments().clone().into_iter().enumerate() {
                inner_state.variables.insert(arg, args[i].clone());
            }

            // Run the function as an expression
            match Token::new(f.definition(), &mut inner_state) {
                Ok(t) => {
                    token.set_value(t.value());
                    return None;
                }
                Err(e) => return Some(e),
            }
        } else {
            return Some(Error::StackOverflow(token.clone()));
        }
    }

    Some(Error::FunctionName {
        name: name.to_string(),
        token: token.clone(),
    })
}

#[cfg(test)]
mod test_token {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_builtin_function_call() {
        assert_token_error!("rooplipp(9)", FunctionName);
        assert_token_error!("sqrt('string')", FunctionArgumentType);
        assert_token_error!("sqrt()", FunctionArguments);
        assert_token_value!("sqrt(9)", Value::Float(3.0));
        assert_token_value!("sqrt(8 + 1)", Value::Float(3.0));
        assert_token_value!("root(9, 2)", Value::Float(3.0));
    }

    #[test]
    fn test_user_function_call() {
        let mut state: ParserState = ParserState::new();
        assert_token_text_stateful!("5+5\nfn(x, y) = x * y\n5+5", "10\nx * y\n10", &mut state);
        assert_token_value_stateful!("fn(5,5)", Value::Integer(25), &mut state);
        assert_token_text_stateful!(
            "fn(x, y) = 5x + 10(x * y)\nfn(2, 3)",
            "5x + 10(x * y)\n70",
            &mut state
        );
        assert_token_error!("f(x) = f(x)\nf(0)", StackOverflow);
        assert_token_text_stateful!(
            "sum(a) = element(a, 0) + ( len(a)>1 ? sum(dequeue(a)) : 0 )",
            "element(a, 0) + ( len(a)>1 ? sum(dequeue(a)) : 0 )",
            &mut state
        );
        assert_token_value_stateful!("sum([10, 10, 11])", Value::Integer(31), &mut state);
    }

    #[test]
    #[cfg(feature = "extensions")]
    fn test_extension_function_call() {
        let mut state: ParserState = ParserState::new();
        state
            .extensions
            .load("example_extensions/colour_utils.js")
            .ok();
        assert_token_value_stateful!("complement(0xFFAA00)", Value::from(0x00FFFF), &mut state);
    }
}
