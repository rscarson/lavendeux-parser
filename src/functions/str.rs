use super::FunctionTable;
use crate::value::{Value, IntegerType};
use crate::errors::*;

/// Register string functions
pub fn register_functions(table: &mut FunctionTable) {
    table.register("concat", builtin_concat);
    table.register("uppercase", builtin_uppercase);
    table.register("lowercase", builtin_lowercase);
    table.register("trim", builtin_trim);
    table.register("strlen", builtin_strlen);
    table.register("substr", builtin_substr);
    table.register("contains", builtin_contains);
}

fn builtin_contains(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 2 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("contains(source, s)", 1, 1)));
    }

    Ok(Value::Boolean(args[0].as_string().contains(&args[1].as_string())))
}

fn builtin_strlen(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("strlen(s)", 1, 1)));
    }

    match &args[0] {
        Value::String(s) => Ok(Value::Integer(s.len() as IntegerType)),
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("strlen(s)", 1, ExpectedTypes::String)))
    }
}

fn builtin_concat(args: &[Value]) -> Result<Value, ParserError> {
    if args.is_empty() {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("concat(s, s2, ...)", 1, 1)));
    }

    Ok(Value::String(args.iter().map(|v|v.as_string()).collect::<String>()))
}

fn builtin_uppercase(args: &[Value]) -> Result<Value, ParserError> {
    if args.is_empty() {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("uppercase(s)", 1, 1)));
    }

    Ok(Value::String(args[0].as_string().to_uppercase()))
}

fn builtin_lowercase(args: &[Value]) -> Result<Value, ParserError> {
    if args.is_empty() {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("lowercase(s)", 1, 1)));
    }

    Ok(Value::String(args[0].as_string().to_lowercase()))
}

fn builtin_trim(args: &[Value]) -> Result<Value, ParserError> {
    if args.is_empty() {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("trim(s)", 1, 1)));
    }

    Ok(Value::String(args[0].as_string().trim().to_string()))
}

fn builtin_substr(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 2 && args.len() != 3 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("substr(s, start, [length])", 2, 3)));
    }

    let start = match args[1].as_int() {
        Some(n) => n,
        None => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("substr(s, start, [length])", 2, ExpectedTypes::IntOrFloat)))
    };

    match &args[0] {
        Value::String(s) => {
            let length = if args.len() == 3 { match args[2].as_int() {
                Some(n) => n,
                None => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("substr(s, start, [length])", 3, ExpectedTypes::IntOrFloat)))
            } } else { s.len() as IntegerType - start };
            if start >= s.len() as IntegerType || start < 0 {
                return Err(ParserError::FunctionArgOverFlow(FunctionArgOverFlowError::new("substr(s, start, [length])", 2)));
            } else if length < 0 || length > (s.len() - start as usize) as IntegerType {
                return Err(ParserError::FunctionArgOverFlow(FunctionArgOverFlowError::new("substr(s, start, [length])", 3)));
            }

            Ok(Value::String(s.chars().skip(start as usize).take(length as usize).collect()))
        },
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("substr(s, start, [length])", 1, ExpectedTypes::String)))
    }
}

#[cfg(test)]
mod test_builtin_functions {
    use super::*;

    #[test]
    fn test_strlen() {
        assert_eq!(Value::Integer(0), builtin_strlen(&[Value::String("".to_string())]).unwrap());
        assert_eq!(Value::Integer(3), builtin_strlen(&[Value::String("   ".to_string())]).unwrap());
    }

    #[test]
    fn test_uppercase() {
        assert_eq!(Value::String("TEST".to_string()), builtin_uppercase(&[Value::String("test".to_string())]).unwrap());
        assert_eq!(Value::String(" TEST  ".to_string()), builtin_uppercase(&[Value::String(" test  ".to_string())]).unwrap());
    }


    #[test]
    fn test_lowercase() {
        assert_eq!(Value::String("test".to_string()), builtin_lowercase(&[Value::String("TEST".to_string())]).unwrap());
        assert_eq!(Value::String(" test  ".to_string()), builtin_lowercase(&[Value::String(" TEST  ".to_string())]).unwrap());
    }


    #[test]
    fn test_trim() {
        assert_eq!(Value::String("test".to_string()), builtin_trim(&[Value::String("test".to_string())]).unwrap());
        assert_eq!(Value::String("TEST".to_string()), builtin_trim(&[Value::String(" TEST  ".to_string())]).unwrap());
    }

    #[test]
    fn test_concat() {
        assert_eq!(Value::String(" ".to_string()), builtin_concat(
            &[Value::String("".to_string()), 
            Value::String(" ".to_string())
        ]).unwrap());
        assert_eq!(Value::String("test4false".to_string()), builtin_concat(
            &[Value::String("test".to_string()), 
            Value::Integer(4),
            Value::Boolean(false)
        ]).unwrap());
    }
    
    #[test]
    fn test_substr() {
        assert_eq!(Value::String("t".to_string()), builtin_substr(
            &[Value::String("test".to_string()), Value::Integer(3)]
            
        ).unwrap());
        assert_eq!(Value::String("tes".to_string()), builtin_substr(
            &[Value::String("test".to_string()), Value::Integer(0), Value::Integer(3)], 
        ).unwrap());
    }
    
    #[test]
    fn test_contains() {
        assert_eq!(Value::Boolean(true), builtin_contains(
            &[Value::String("test".to_string()), Value::String("e".to_string())]
            
        ).unwrap());
        assert_eq!(Value::Boolean(false), builtin_contains(
            &[Value::String("test".to_string()), Value::String("fff".to_string())]
            
        ).unwrap());
    }
}