use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::prelude::*;
use super::FunctionTable;
use crate::value::{Value, IntegerType};
use crate::errors::*;

/// Register developper functions
pub fn register_functions(table: &mut FunctionTable) {
    table.register("choose", builtin_choose);
    table.register("rand", builtin_rand);
    table.register("time", builtin_time);
    table.register("tail", builtin_tail);
}

fn builtin_choose(args: &[Value]) -> Result<Value, ParserError> {
    let mut rng = rand::thread_rng();

    if args.is_empty() {
        Err(ParserError::FunctionNArg(FunctionNArgError::new("choose(..)", 1, 100)))
    } else {
        let arg = rng.gen_range(0..args.len());
        Ok(args[arg].clone())
    }
}

fn builtin_rand(args: &[Value]) -> Result<Value, ParserError> {
    let mut rng = rand::thread_rng();
    if args.is_empty() {
        // Generate a float between 0 and 1
        Ok(Value::Float(rng.gen()))
    } else if args.len() == 2 {
        if !matches!(args[0], Value::Integer(_)) || !matches!(args[1], Value::Integer(_)) {
            Err(ParserError::FunctionArgType(FunctionArgTypeError::new("rand([start], [end])", 1, ExpectedTypes::Int)))
        } else {
            Ok(Value::Integer(rng.gen_range(args[0].as_int().unwrap()..args[1].as_int().unwrap())))
        }
    } else {
        Err(ParserError::FunctionNArg(FunctionNArgError::new("rand([start, end])", 0, 2)))
    }
}

fn builtin_time(_args: &[Value]) -> Result<Value, ParserError> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => Ok(Value::Integer(n.as_secs() as IntegerType)),
        Err(_) => Ok(Value::Integer(0))
    }
}

fn builtin_tail(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 1 && args.len() != 2 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("tail(file, [n_lines])", 1, 2)));
    }

    let mut n_lines = 1;
    if args.len() == 2 {
        match args[1].as_int() {
            Some(n) => n_lines = n,
            None => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("tail(file, [n_lines])", 1, ExpectedTypes::IntOrFloat)))
        }
    }

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

#[cfg(test)]
mod test_builtin_table {
    use super::*;
    const WAS_NOW : IntegerType = 1647531435;
    
    #[test]
    fn test_choose() {
        let mut result;
        for _ in 0..30 {
            result = builtin_choose(&[Value::String("test".to_string()), Value::Integer(5)]).unwrap();
            assert_eq!(true, result.is_string() || result == Value::Integer(5).is_int());
        }
    }
    
    #[test]
    fn test_rand() {
        let mut result;

        for _ in 0..30 {
            result = builtin_rand(&[]).unwrap();
            println!("{}", result);
            assert_eq!(true, result.as_float().unwrap() >= 0.0 && result.as_float().unwrap() <= 1.0);
        }

        for _ in 0..30 {
            result = builtin_rand(&[Value::Integer(5), Value::Integer(10)]).unwrap();
            println!("{}", result);
            assert_eq!(true, result.as_int().unwrap() >= 5 && result.as_int().unwrap() <= 10);
        }
    }
    
    #[test]
    fn test_time() {
        let result = builtin_time(&[]).unwrap();
        assert_eq!(true, result.as_int().unwrap() > WAS_NOW);
    }
    
    #[test]
    fn test_tail() {
        let result = builtin_tail(&[Value::String("README.md".to_string()), Value::Integer(5)]).unwrap();
        assert_eq!(4, result.as_string().matches("\n").count());
    }
}
