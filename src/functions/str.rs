use regex::Regex;
use super::{FunctionDefinition, FunctionArgument, FunctionTable};
use crate::value::{Value, IntegerType};
use crate::errors::*;

const CONTAINS : FunctionDefinition = FunctionDefinition {
    name: "contains",
    description: "Returns true if [source] contains the substring [s]",
    arguments: || vec![
        FunctionArgument::new_required("source", ExpectedTypes::String),
        FunctionArgument::new_required("s", ExpectedTypes::String)
    ],
    handler: |_, args: &[Value]| {
        Ok(Value::Boolean(args[0].as_string().contains(&args[1].as_string())))
    }
};

const CONCAT : FunctionDefinition = FunctionDefinition {
    name: "concat",
    description: "Concatenate a set of strings",
    arguments: || vec![
        FunctionArgument::new_plural("s", ExpectedTypes::String, true)
    ],
    handler: |_, args: &[Value]| {
        Ok(Value::String(args.iter().map(|v|v.as_string()).collect::<String>()))
    }
};

const STRLEN : FunctionDefinition = FunctionDefinition {
    name: "strlen",
    description: "Returns the length of the string s",
    arguments: || vec![
        FunctionArgument::new_required("s", ExpectedTypes::String)
    ],
    handler: |_, args: &[Value]| {
        Ok(Value::Integer(args[0].as_string().len() as IntegerType))
    }
};

const UPPERCASE : FunctionDefinition = FunctionDefinition {
    name: "uppercase",
    description: "Converts the string s to uppercase",
    arguments: || vec![
        FunctionArgument::new_required("s", ExpectedTypes::String)
    ],
    handler: |_, args: &[Value]| {
        Ok(Value::String(args[0].as_string().to_uppercase()))
    }
};

const LOWERCASE : FunctionDefinition = FunctionDefinition {
    name: "lowercase",
    description: "Converts the string s to lowercase",
    arguments: || vec![
        FunctionArgument::new_required("s", ExpectedTypes::String)
    ],
    handler: |_, args: &[Value]| {
        Ok(Value::String(args[0].as_string().to_lowercase()))
    }
};

const TRIM : FunctionDefinition = FunctionDefinition {
    name: "trim",
    description: "Trim whitespace from a string",
    arguments: || vec![
        FunctionArgument::new_required("s", ExpectedTypes::String)
    ],
    handler: |_, args: &[Value]| {
        Ok(Value::String(args[0].as_string().trim().to_string()))
    }
};

const SUBSTR : FunctionDefinition = FunctionDefinition {
    name: "substr",
    description: "Returns a substring from s, beginning at [start], and going to the end, or for [length] characters",
    arguments: || vec![
        FunctionArgument::new_required("s", ExpectedTypes::String),
        FunctionArgument::new_required("start", ExpectedTypes::IntOrFloat),
        FunctionArgument::new_optional("length", ExpectedTypes::IntOrFloat)
    ],
    handler: |definition, args: &[Value]| {
        let start = args[1].as_int().unwrap();
        let s = args[0].as_string();
        let length = if args.len() == 3 { args[2].as_int().unwrap() } else { s.len() as IntegerType - start };

        if start >= s.len() as IntegerType || start < 0 {
            return Err(ParserError::FunctionArgOverFlow(FunctionArgOverFlowError::new(&definition.signature(), 2)));
        } else if length < 0 || length > (s.len() - start as usize) as IntegerType {
            return Err(ParserError::FunctionArgOverFlow(FunctionArgOverFlowError::new(&definition.signature(), 3)));
        }

        Ok(Value::String(s.chars().skip(start as usize).take(length as usize).collect()))
    }
};

const REGEX : FunctionDefinition = FunctionDefinition {
    name: "regex",
    description: "Returns a regular expression match from [subject], or false",
    arguments: || vec![
        FunctionArgument::new_required("pattern", ExpectedTypes::String),
        FunctionArgument::new_required("subject", ExpectedTypes::IntOrFloat),
        FunctionArgument::new_optional("group", ExpectedTypes::Int)
    ],
    handler: |_, args: &[Value]| {
        let re = Regex::new(&args[0].as_string());
        if let Err(e) = re {
            return Err(ParserError::General(format!("Invalid regular expression: {}", e)));
        }
    
        if let Some(caps) = re.unwrap().captures(&args[1].as_string()) {
            if args.len() == 2 {
                return Ok(Value::String(caps.get(0).unwrap().as_str().to_string()));
            } else {
                let group_index = args[2].as_int().unwrap();
                if let Some(group) = caps.get(group_index as usize) {
                    return Ok(Value::String(group.as_str().to_string()));
                }
            }
        }
        
        Ok(Value::Boolean(false))
    }
};

/// Register string functions
pub fn register_functions(table: &mut FunctionTable) {

    table.register(CONTAINS);
    table.register(CONCAT);
    table.register(STRLEN);
    table.register(UPPERCASE);
    table.register(LOWERCASE);
    table.register(TRIM);
    table.register(SUBSTR);
    table.register(REGEX);
}

#[cfg(test)]
mod test_builtin_functions {
    use super::*;

    #[test]
    fn test_regex() {
        assert_eq!(Value::Boolean(false), (REGEX.handler)(&REGEX, &[
            Value::String("test".to_string()), Value::String("bar".to_string())
        ]).unwrap());
        assert_eq!(Value::String("foo".to_string()), (REGEX.handler)(&REGEX, &[
            Value::String("foo".to_string()), Value::String("foobar".to_string())
        ]).unwrap());
        assert_eq!(Value::String("foobar".to_string()), (REGEX.handler)(&REGEX, &[
            Value::String("foo.*".to_string()), Value::String("foobar".to_string())
        ]).unwrap());
        assert_eq!(Value::String("bar".to_string()), (REGEX.handler)(&REGEX, &[
            Value::String("foo(.*)".to_string()), Value::String("foobar".to_string()), 
            Value::Integer(1)
        ]).unwrap());
    }

    #[test]
    fn test_strlen() {
        assert_eq!(Value::Integer(0), (STRLEN.handler)(&STRLEN, &[Value::String("".to_string())]).unwrap());
        assert_eq!(Value::Integer(3), (STRLEN.handler)(&STRLEN, &[Value::String("   ".to_string())]).unwrap());
    }

    #[test]
    fn test_uppercase() {
        assert_eq!(Value::String("TEST".to_string()), (UPPERCASE.handler)(&UPPERCASE, &[Value::String("test".to_string())]).unwrap());
        assert_eq!(Value::String(" TEST  ".to_string()), (UPPERCASE.handler)(&UPPERCASE, &[Value::String(" test  ".to_string())]).unwrap());
    }


    #[test]
    fn test_lowercase() {
        assert_eq!(Value::String("test".to_string()), (LOWERCASE.handler)(&LOWERCASE, &[Value::String("TEST".to_string())]).unwrap());
        assert_eq!(Value::String(" test  ".to_string()), (LOWERCASE.handler)(&LOWERCASE, &[Value::String(" TEST  ".to_string())]).unwrap());
    }


    #[test]
    fn test_trim() {
        assert_eq!(Value::String("test".to_string()), (TRIM.handler)(&TRIM, &[Value::String("test".to_string())]).unwrap());
        assert_eq!(Value::String("TEST".to_string()), (TRIM.handler)(&TRIM, &[Value::String(" TEST  ".to_string())]).unwrap());
    }

    #[test]
    fn test_concat() {
        assert_eq!(Value::String(" ".to_string()), (CONCAT.handler)(
            &CONCAT, &[Value::String("".to_string()), 
            Value::String(" ".to_string())
        ]).unwrap());
        assert_eq!(Value::String("test4false".to_string()), (CONCAT.handler)(
            &CONCAT, &[Value::String("test".to_string()), 
            Value::Integer(4),
            Value::Boolean(false)
        ]).unwrap());
    }
    
    #[test]
    fn test_substr() {
        assert_eq!(Value::String("t".to_string()), (SUBSTR.handler)(
            &SUBSTR, &[Value::String("test".to_string()), Value::Integer(3)]
            
        ).unwrap());
        assert_eq!(Value::String("tes".to_string()), (SUBSTR.handler)(
            &SUBSTR, &[Value::String("test".to_string()), Value::Integer(0), Value::Integer(3)], 
        ).unwrap());
    }
    
    #[test]
    fn test_contains() {
        assert_eq!(Value::Boolean(true), (CONTAINS.handler)(
            &CONTAINS, &[Value::String("test".to_string()), Value::String("e".to_string())]
            
        ).unwrap());
        assert_eq!(Value::Boolean(false), (CONTAINS.handler)(
            &CONTAINS, &[Value::String("test".to_string()), Value::String("fff".to_string())]
            
        ).unwrap());
    }
}