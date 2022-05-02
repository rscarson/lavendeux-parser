use super::{FunctionDefinition, FunctionArgument, FunctionTable};
use crate::value::{Value, IntegerType};
use crate::errors::*;

use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::prelude::*;

const CHOOSE : FunctionDefinition = FunctionDefinition {
    name: "choose",
    description: "Returns any one of the provided arguments at random",
    arguments: || vec![
        FunctionArgument::new_plural("option", ExpectedTypes::Any, false)
    ],
    handler: |_, args: &[Value]| {
        let mut rng = rand::thread_rng();
        let arg = rng.gen_range(0..args.len());
        Ok(args[arg].clone())
    }
};

const RAND : FunctionDefinition = FunctionDefinition {
    name: "rand",
    description: "With no arguments, return a float from 0 to 1. Otherwise return an integer from 0 to m, or m to n",
    arguments: || vec![
        FunctionArgument::new_optional("m", ExpectedTypes::Int),
        FunctionArgument::new_optional("n", ExpectedTypes::Int)
    ],
    handler: |_, args: &[Value]| {
        let mut rng = rand::thread_rng();
        if args.is_empty() {
            // Generate a float between 0 and 1
            Ok(Value::Float(rng.gen()))
        } else if args.len() == 1 {
            // Generate an int between 0 and n
            let n = args[0].as_int().unwrap();
            if n < 0 {
                Ok(Value::Integer(rng.gen_range(n..0)))
            } else {
                Ok(Value::Integer(rng.gen_range(0..n)))
            }
        } else {
            // Generate an int between n and m
            let n = args[0].as_int().unwrap();
            let m = args[1].as_int().unwrap();
            if n < m {
                Ok(Value::Integer(rng.gen_range(n..m)))
            } else {
                Ok(Value::Integer(rng.gen_range(m..n)))
            }
        }
    }
};

const TIME : FunctionDefinition = FunctionDefinition {
    name: "time",
    description: "Returns a unix timestamp for the current system time",
    arguments: Vec::new,
    handler: |_, _: &[Value]| {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => Ok(Value::Integer(n.as_secs() as IntegerType)),
            Err(_) => Ok(Value::Integer(0))
        }
    }
};

const TAIL : FunctionDefinition = FunctionDefinition {
    name: "tail",
    description: "Returns the last [lines] lines from a given file",
    arguments: || vec![
        FunctionArgument::new_required("filename", ExpectedTypes::String),
        FunctionArgument::new_optional("lines", ExpectedTypes::Int),
    ],
    handler: |_, args: &[Value]| {
        let n_lines = if args.len() == 2 { args[1].as_int().unwrap().abs()} else { 1 };

        let f = File::open(args[0].as_string())?;
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
    table.register(CHOOSE);
    table.register(RAND);
    table.register(TIME);
    table.register(TAIL);
}

#[cfg(test)]
mod test_builtin_table {
    use super::*;
    const WAS_NOW : IntegerType = 1647531435;
    
    #[test]
    fn test_choose() {
        let mut result;
        for _ in 0..30 {
            result = (CHOOSE.handler)(&CHOOSE, &[Value::String("test".to_string()), Value::Integer(5)]).unwrap();
            assert_eq!(true, result.is_string() || result == Value::Integer(5).is_int());
        }
    }
    
    #[test]
    fn test_rand() {
        let mut result;

        for _ in 0..30 {
            result = (RAND.handler)(&RAND, &[]).unwrap();
            assert_eq!(true, result.as_float().unwrap() >= 0.0 && result.as_float().unwrap() <= 1.0);
        }

        for _ in 0..30 {
            result = (RAND.handler)(&RAND, &[Value::Integer(5)]).unwrap();
            assert_eq!(true, result.as_int().unwrap() >= 0 && result.as_int().unwrap() <= 5);
        }

        for _ in 0..30 {
            result = (RAND.handler)(&RAND, &[Value::Integer(5), Value::Integer(10)]).unwrap();
            assert_eq!(true, result.as_int().unwrap() >= 5 && result.as_int().unwrap() <= 10);
        }
    }
    
    #[test]
    fn test_time() {
        let result = (TIME.handler)(&TIME, &[]).unwrap();
        assert_eq!(true, result.as_int().unwrap() > WAS_NOW);
    }
    
    #[test]
    fn test_tail() {
        let result = (TAIL.handler)(&TAIL, &[Value::String("README.md".to_string()), Value::Integer(5)]).unwrap();
        assert_eq!(4, result.as_string().matches("\n").count());
    }
}
