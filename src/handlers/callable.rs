use crate::token::{Rule, Token};
use crate::value::{Value};
use crate::state::{ParserState, UserFunction};
use crate::errors::*;

fn user_function_signature(f: &UserFunction) -> String {
    format!("{}({})", f.name, f.arguments.join(", "))
}

fn inline_sort<T>(mut v: Vec<T>) -> Vec<T> where T: std::cmp::Ord {
    v.sort();
    v
}

fn inline_sortby<T>(mut v: Vec<T>, f: fn(&T, &T) -> std::cmp::Ordering) -> Vec<T> {
    v.sort_by(f);
    v
}

pub fn call_expression_handler(token: &mut Token, state: &mut ParserState) -> Option<ParserError> {
    if token.rule() == Rule::call_expression {
        // Get function name and arguments
        let name = token.child(0).unwrap().text();
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

        // Help
        if name == "help" {
            if args.len() == 1 {
                let target_name = if arg_tokens[0].value().is_none() { arg_tokens[0].text().to_string() } else { arg_tokens[0].value().to_string() };

                // Builtin functions
                if let Some(f) = state.functions.get(&target_name) {
                    token.set_value(Value::String(f.help()));
                    return None;
                }

                // Extension functions
                #[cfg(feature = "extensions")]
                if state.extensions.has_function(&target_name) {
                    let signature = format!("{}(...)", target_name);
                    token.set_value(Value::String(signature));
                    return None;
                }
                
                // User-defined functions
                if let Some(f) = state.user_functions.get(&target_name) {
                    token.set_value(Value::String(user_function_signature(f)));
                    return None;
                }

                return Some(ParserError::FunctionName(FunctionNameError::new(&target_name)));
            } else {
                // List all functions and decorators
                let mut output = "".to_string();
                let divider = "===============";
                
                output += format!("Built-in Functions\n{}\n", divider).as_str();
                output += inline_sortby(state.functions.all(), |f1, f2|f1.name.cmp(f2.name)).into_iter().map(|f|
                    f.help()
                ).collect::<Vec<String>>().join("\n").as_str();
                
                output += format!("\n\nBuilt-in Decorators\n{}\n", divider).as_str();
                output += inline_sort(state.decorators.all()).into_iter().map(|f|
                    format!("@{}: {}", f, state.decorators.get(f).unwrap().description())
                ).collect::<Vec<String>>().join("\n").as_str();
                
                #[cfg(feature = "extensions")]
                if !state.extensions.all().is_empty() {
                    for extension in inline_sortby(state.extensions.all(), |a,b|a.name().cmp(b.name())) {
                        output += format!("\n\n{} v{}\nAuthor: {}\n{}\n", 
                            extension.name(), 
                            extension.version(), 
                            extension.author(), 
                            divider
                        ).as_str();
                        let e_functions = inline_sort(extension.functions()).join(", ");
                        let e_decorators = inline_sort(extension.decorators()).into_iter().map(|f|
                            format!("@{}", f)
                        ).collect::<Vec<String>>().join(", ");
                        output += format!("functions: {}\ndecorators: {}\n", e_functions, e_decorators).as_str();
                    }
                }
                
                if !state.user_functions.is_empty() {
                    output += format!("\n\nUser-defined Functions\n{}\n", divider).as_str();
                    output += inline_sort(state.user_functions.values().map(user_function_signature).collect::<Vec<String>>()).join("\n").as_str();
                }

                token.set_value(Value::String(output));
                return None;
            }
        }

        // Builtin functions
        if state.functions.has(name) {
            match state.functions.call(name, &args[..]) {
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
                return Some(ParserError::FunctionNArg(FunctionNArgError::new_with_token(token, &f.name, f.arguments.len(), f.arguments.len())));
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
                return Some(ParserError::Stack(StackError::new_with_token(token)));
            }
        }

        return Some(ParserError::FunctionName(FunctionNameError::new_with_token(token, name)));
    }

    None
}