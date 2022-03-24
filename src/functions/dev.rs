use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::prelude::*;
use crate::value::{AtomicValue, IntegerType};
use crate::errors::*;

pub fn builtin_choose(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    let mut rng = rand::thread_rng();

    if args.is_empty() {
        Err(ParserError::FunctionNArg(FunctionNArgError::new("choose(..)", 1, 100)))
    } else {
        let arg = rng.gen_range(0..(args.len() - 1));
        Ok(args[arg].clone())
    }
}

pub fn builtin_rand(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    let mut rng = rand::thread_rng();
    if args.is_empty() {
        // Generate a float between 0 and 1
        Ok(AtomicValue::Float(rng.gen()))
    } else if args.len() == 2 {
        if !matches!(args[0], AtomicValue::Integer(_)) || !matches!(args[1], AtomicValue::Integer(_)) {
            Err(ParserError::FunctionArgType(FunctionArgTypeError::new("rand([start], [end])", 1, ExpectedTypes::Int)))
        } else {
            Ok(AtomicValue::Integer(rng.gen_range(args[0].as_int().unwrap()..args[1].as_int().unwrap())))
        }
    } else {
        Err(ParserError::FunctionNArg(FunctionNArgError::new("rand([start, end])", 0, 2)))
    }
}

pub fn builtin_time(_args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => Ok(AtomicValue::Integer(n.as_secs() as IntegerType)),
        Err(_) => Ok(AtomicValue::Integer(0))
    }
}

pub fn builtin_tail(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
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

    Ok(AtomicValue::String(lines.join("\n")))
}

#[cfg(test)]
mod test_builtin_table {
    use super::*;
    const WAS_NOW : IntegerType = 1647531435;
    
    #[test]
    fn test_choose() {
        let result = builtin_choose(&[AtomicValue::String("test".to_string()), AtomicValue::Integer(5)]).unwrap();
        assert_eq!(true, result == AtomicValue::String("test".to_string()) || result == AtomicValue::Integer(5));
    }
    
    #[test]
    fn test_rand() {
        let mut result = builtin_rand(&[]).unwrap();
        assert_eq!(true, result.as_float().unwrap() >= 0.0 && result.as_float().unwrap() <= 1.0);

        result = builtin_rand(&[AtomicValue::Integer(5), AtomicValue::Integer(10)]).unwrap();
        assert_eq!(true, result.as_int().unwrap() >= 5 && result.as_int().unwrap() <= 10);
    }
    
    #[test]
    fn test_time() {
        let result = builtin_time(&[]).unwrap();
        assert_eq!(true, result.as_int().unwrap() > WAS_NOW);
    }
    
    #[test]
    fn test_tail() {
        let result = builtin_tail(&[AtomicValue::String("readme.md".to_string()), AtomicValue::Integer(5)]).unwrap();
        assert_eq!(4, result.as_string().matches("\n").count());
    }
}