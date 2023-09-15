//! Builtin functions for lower level ops

use crate::{Token, help::Help};
use super::*;
const HELP : FunctionDefinition = FunctionDefinition {
    name: "help",
    category: None,
    description: "Display a help message",
    arguments: || vec![
        FunctionArgument::new_optional("function_name", ExpectedTypes::String),
    ],
    handler: |_function, token, state, args| {
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
        
                Err(FunctionNameError::new(token, &target).into())
            },

            None => {
                let mut help = Help::new();

                help.add_block("Syntax Examples")
                    .add_entry("Assigns the result of '60*60*24' to a variable, and outputs the result as a float:")
                    .add_entry("    one_day = 60 * 60 * 24 @float // A comment")
                    .add_entry("Creates a function called 'factorial' taking 1 argument:")
                    .add_entry("    factorial(x) = x==0 ? 1 : (x * factorial(x - 1) )")
                    .add_entry("Creates a function called that uses arrays:")
                    .add_entry("    sum(a) = element(a, 0) + ( len(a)>1 ? sum(dequeue(a)) : 0 )")
                    .add_entry("Performs arithmetic between arrays, and scalars: ")
                    .add_entry("   [10, 12] + 2 * [1.2, 1.3]");
                
                help.add_block("Operators")
                    .add_entry("   Bitwise: AND (0xF & 0xA), OR (0xA | 0xF), XOR (0xA ^ 0xF), NOT (~0xA), SHIFT (0xF >> 1, 0xA << 1)")
                    .add_entry("   Boolean: AND (true && false), OR (true || false), CMP (1 < 2, 4 >= 5), EQ (1 == 1, 2 != 5)")
                    .add_entry("Arithmetic: Add/Sub (+, -), Mul/Div (*, /), Exponentiation (**), Modulo (%), Implied Mul ((5)(5), 5x)")
                    .add_entry("     Unary: Factorial (5!!), Negation (-1, -(1+1))");
                
                help.add_block("Data Types")
                    .add_entry("  String: Text delimited by 'quotes' or \"double-quotes\"")
                    .add_entry(" Boolean: A truth value (true or false)")
                    .add_entry(" Integer: A whole number. Can also be base2 (0b111), base8 (0o777), or base16 (0xFF)")
                    .add_entry("   Float: A decimal number. Can also be in scientific notation(5.3e+4, 4E-2)")
                    .add_entry("Currency: A decimal number - does not apply any exhange rates ($5.00)")
                    .add_entry("   Array: A comma separated list of values in square brackets; [1, 'test']")
                    .add_entry("  Object: A comma separated list of key/value pairs in curly braces; {'test': 5}")
                    .add_entry("Variable: An identifier representing a value. Set it with x=5, then use it in an expression (5x)")
                    .add_entry(" Contant: A preset read-only variable representing a common value, such as pi, e, and tau");

                help.add_std(state);
                Ok(Value::String(help.to_string()))
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
    handler: |_function, _token, state, args| {
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
    handler: |_function, token, state, args| {
        let filename = args.get("filename").required().as_string();
        match std::fs::read_to_string(filename) {
            Ok(script) => {
                match Token::new(&script, state) {
                    Ok(t) => Ok(t.value()),
                    Err(e) => Err(e)
                }
            },
            Err(e) => Err(IOError::new(token, &e.to_string()).into())
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

    #[cfg(feature = "extensions")]
    use crate::Extension;
    
    use std::path::PathBuf;

    #[test]
    fn test_call() {
        let mut state = ParserState::new();
        
        // Get test script location
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("example_scripts");
        d.push("populate_state.lav");
        let path = d.display().to_string().replace("\\", "\\\\");

        assert_eq!(true, CALL.call(&Token::dummy(""), &mut state, &[
            Value::String("not a real path.oops".to_string())
        ]).is_err());

        assert_eq!(false, CALL.call(&Token::dummy(""), &mut state, &[
            Value::String(path)
        ]).is_err());

        assert_eq!(true, state.variables.contains_key("important_value"));
        assert_eq!(true, state.user_functions.contains_key("factorial"));
    }

    #[test]
    fn test_run() {
        let mut state = ParserState::new();

        assert_eq!(Value::Boolean(false), RUN.call(&Token::dummy(""), &mut state, &[
            Value::String("contains(help(), 'foobar(')".to_string())
        ]).unwrap());

        assert_eq!(Value::String("0".to_string()), RUN.call(&Token::dummy(""), &mut state, &[
            Value::String("foobar(x) = 0".to_string())
        ]).unwrap());

        assert_eq!(Value::Boolean(true), RUN.call(&Token::dummy(""), &mut state, &[
            Value::String("contains(help(), 'foobar(')".to_string())
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

        assert_eq!(true, HELP.call(&Token::dummy(""), &mut state, &[
        ]).unwrap().as_string().contains("Math Functions"));

        assert_eq!(true, HELP.call(&Token::dummy(""), &mut state, &[
        ]).unwrap().as_string().contains("Built-in Decorators"));
        
        #[cfg(feature = "extensions")]

        assert_eq!(true, HELP.call(&Token::dummy(""), &mut state, &[
        ]).unwrap().as_string().contains("Unnamed Extension v0.0.0"));

        assert_eq!("strlen(s): Returns the length of the string s", HELP.call(&Token::dummy(""), &mut state, &[
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