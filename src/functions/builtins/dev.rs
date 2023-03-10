//! Builtin functions that don't fit nicely into other categories

use super::*;
use crate::value::{Value, IntegerType};

use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::{BufRead, BufReader};

const TIME : FunctionDefinition = FunctionDefinition {
    name: "time",
    category: None,
    description: "Returns a unix timestamp for the current system time",
    arguments: Vec::new,
    handler: |_function, _state, _args| {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => Ok(Value::Integer(n.as_secs() as IntegerType)),
            Err(_) => Ok(Value::Integer(0))
        }
    }
};

const DEFAULT_TAIL_LINES: IntegerType = 1;
const TAIL : FunctionDefinition = FunctionDefinition {
    name: "tail",
    category: None,
    description: "Returns the last [lines] lines from a given file",
    arguments: || vec![
        FunctionArgument::new_required("filename", ExpectedTypes::String),
        FunctionArgument::new_optional("lines", ExpectedTypes::Int),
    ],
    handler: |_function, _state, args| {
        let n_lines: IntegerType = args.get("lines").optional_or(Value::Integer(DEFAULT_TAIL_LINES))
            .as_int().unwrap_or(DEFAULT_TAIL_LINES);

        let f = File::open(args.get("filename").required().as_string())?;
        let mut lines : Vec<String> = Vec::new();
        for line in BufReader::new(f).lines() {
            lines.push(line?);
            if lines.len() as IntegerType > n_lines {
                lines.remove(0);
            }
        }

        Ok(Value::String(lines.join("\n")))
    }
};

/// Register developper functions
pub fn register_functions(table: &mut FunctionTable) {
    table.register(TIME);
    table.register(TAIL);
}

#[cfg(test)]
mod test_builtin_table {
    use super::*;
    const WAS_NOW : IntegerType = 1647531435;
    
    #[test]
    fn test_time() {
        let mut state = ParserState::new();

        let result = TIME.call(&mut state, &[]).unwrap();
        assert_eq!(true, result.as_int().unwrap() > WAS_NOW);
    }
    
    #[test]

    fn test_tail() {
        let mut state = ParserState::new();

        let result = TAIL.call(&mut state, &[Value::String("README.md".to_string()), Value::Integer(5)]).unwrap();
        assert_eq!(4, result.as_string().matches("\n").count());
    }
}
