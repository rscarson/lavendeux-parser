//! Builtin functions for lower level ops

use crate::{Token};
use super::*;

fn inline_sort<T>(mut v: Vec<T>) -> Vec<T> where T: std::cmp::Ord {
    v.sort();
    v
}

fn inline_sortby<T>(mut v: Vec<T>, f: fn(&T, &T) -> std::cmp::Ordering) -> Vec<T> {
    v.sort_by(f);
    v
}

const HELP : FunctionDefinition = FunctionDefinition {
    name: "help",
    category: None,
    description: "Display a help message",
    arguments: || vec![
        FunctionArgument::new_optional("function_name", ExpectedTypes::String),
    ],
    handler: |_function, state, args| {
        match args.get("function_name").optional() {
            Some(f) => {
                let target = f.as_string();

                // Builtin functions
                if let Some(f) = state.functions.get(&target) {
                    return Ok(Value::String(f.help()));
                }
        
                // Extension functions
                #[cfg(feature = "extensions")]
                if state.extensions.has_function(&target) {
                    let signature = format!("{}(...)", target);
                    return Ok(Value::String(signature));
                }
                
                // User-defined functions
                if let Some(f) = state.user_functions.get(&target) {
                    return Ok(Value::String(f.signature()));
                }
        
                Err(ParserError::FunctionName(FunctionNameError::new(&target)))
            },

            None => {
                // List all functions and decorators
                let mut output = "".to_string();
                let divider = "===============";
        
                // Builtin functions
                for (category, functions) in state.functions.all_by_category() {
                    let mut c = category.chars();
                    let uppercase: String = c.next().unwrap_or(' ').to_uppercase().chain(c).collect();
                    let functions_help = inline_sortby(functions, |f1, f2|f1.name().cmp(f2.name())).into_iter().map(|f|
                        f.help()
                    ).collect::<Vec<String>>().join("\n");
        
                    output += format!("{} Functions\n{}\n{}\n\n", uppercase, divider, functions_help).as_str();
                }
                
                // Builtin decorators
                output += format!("\n\nBuilt-in Decorators\n{}\n", divider).as_str();
                output += inline_sort(state.decorators.all()).into_iter().map(|f|
                    format!("@{}: {}", f, state.decorators.get(f).unwrap().description())
                ).collect::<Vec<String>>().join("\n").as_str();
                
                // Extension features
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
                    output += inline_sort(state.user_functions.values().map(|f| f.signature()).collect::<Vec<String>>()).join("\n").as_str();
                }
        
                Ok(Value::String(output))
            }
        }
    }
};

const RUN : FunctionDefinition = FunctionDefinition {
    name: "run",
    category: None,
    description: "Run a string as an expression",
    arguments: || vec![
        FunctionArgument::new_required("expression", ExpectedTypes::String),
    ],
    handler: |_function, state, args| {
        let expression = args.get("expression").required().as_string();
        match Token::new(&expression, state) {
            Ok(t) => Ok(t.value()),
            Err(e) => Err(e)
        }
    }
};

const CALL : FunctionDefinition = FunctionDefinition {
    name: "call",
    category: None,
    description: "Run the contents of a file as a script",
    arguments: || vec![
        FunctionArgument::new_required("filename", ExpectedTypes::String),
    ],
    handler: |_function, state, args| {
        let filename = args.get("filename").required().as_string();
        match std::fs::read_to_string(filename) {
            Ok(script) => {
                match Token::new(&script, state) {
                    Ok(t) => Ok(t.value()),
                    Err(e) => Err(e)
                }
            },
            Err(e) => Err(ParserError::General(e.to_string()))
        }
    }
};

/// Register api functions
pub fn register_functions(table: &mut FunctionTable) {
    table.register(HELP);
    table.register(RUN);
    table.register(CALL);
}

#[cfg(test)]
mod test_token {
    use super::*;

    use crate::{Extension};
    use std::path::PathBuf;


    #[test]
    fn test_call() {
        let mut state = ParserState::new();
        
        // Get test script location
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("example_scripts");
        d.push("populate_state.lav");
        let path = d.display().to_string().replace("\\", "\\\\");

        assert_eq!(true, CALL.call(&mut state, &[
            Value::String("not a real path.oops".to_string())
        ]).is_err());

        assert_eq!(false, CALL.call(&mut state, &[
            Value::String(path)
        ]).is_err());

        assert_eq!(true, state.variables.contains_key("important_value"));
        assert_eq!(true, state.user_functions.contains_key("factorial"));
    }

    #[test]
    fn test_run() {
        let mut state = ParserState::new();

        assert_eq!(Value::Boolean(false), RUN.call(&mut state, &[
            Value::String("contains(help(), 'factorial(')".to_string())
        ]).unwrap());

        assert_eq!(Value::String("0".to_string()), RUN.call(&mut state, &[
            Value::String("factorial(x) = 0".to_string())
        ]).unwrap());

        assert_eq!(Value::Boolean(true), RUN.call(&mut state, &[
            Value::String("contains(help(), 'factorial(')".to_string())
        ]).unwrap());
    }

    #[test]
    fn test_help() {
        let mut state = ParserState::new();

        // Help
        #[cfg(feature = "extensions")]
        state.extensions.add("test.js", Extension::new_stub(
            None, None, None, 
            vec!["test".to_string(), "test2".to_string()], 
            vec!["test3".to_string(), "test4".to_string()]
        ));

        assert_eq!(true, HELP.call(&mut state, &[
        ]).unwrap().as_string().contains("Math Functions"));

        assert_eq!(true, HELP.call(&mut state, &[
        ]).unwrap().as_string().contains("Built-in Decorators"));
        
        #[cfg(feature = "extensions")]

        assert_eq!(true, HELP.call(&mut state, &[
        ]).unwrap().as_string().contains("Unnamed Extension v0.0.0"));

        assert_eq!("strlen(s): Returns the length of the string s", HELP.call(&mut state, &[
            Value::String("strlen".to_string())
        ]).unwrap().as_string());

        assert_eq!("strlen(s): Returns the length of the string s", Token::new("help('strlen')", &mut state).unwrap().text());
        assert_eq!("strlen(s): Returns the length of the string s", Token::new("help(strlen)", &mut state).unwrap().text());
        
        Token::new("fn(x, y) = 5x + 10(x * y)", &mut state).unwrap();
        assert_eq!("fn(x, y) = 5x + 10(x * y)", Token::new("help('fn')", &mut state).unwrap().text());
        assert_eq!("fn(x, y) = 5x + 10(x * y)", Token::new("help(fn)", &mut state).unwrap().text());
        
        #[cfg(feature = "extensions")]
        assert_eq!("test2(...)", Token::new("help('test2')", &mut state).unwrap().text());
        
        #[cfg(feature = "extensions")]
        assert_eq!("test2(...)", Token::new("help(test2)", &mut state).unwrap().text());
    }
}