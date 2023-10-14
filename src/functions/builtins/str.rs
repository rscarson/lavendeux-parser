//! Builtin functions for string manipulation

use regex::Regex;
use super::*;
use crate::value::{Value, IntegerType};
use crate::ExpectedTypes;

const CONTAINS : FunctionDefinition = FunctionDefinition {
    name: "contains",
    category: Some("strings"),
    description: "Returns true if array or string [source] contains [s]",
    arguments: || vec![
        FunctionArgument::new_required("source", ExpectedTypes::Any),
        FunctionArgument::new_required("s", ExpectedTypes::Any)
    ],
    handler: |_function, _token, _state, args| {
        let source = args.get("source").required();
        let s = args.get("s").required();
        match source.is_array() {
            true => Ok(Value::Boolean(source.as_array().contains(&s))),
            false => Ok(Value::Boolean(source.to_string().contains(&s.to_string())))
        }
    }
};

const CONCAT : FunctionDefinition = FunctionDefinition {
    name: "concat",
    category: Some("strings"),
    description: "Concatenate a set of strings",
    arguments: || vec![
        FunctionArgument::new_plural("s", ExpectedTypes::String, true)
    ],
    handler: |_function, _token, _state, args| {
        Ok(Value::String(args.iter().map(|v|v.as_string()).collect::<String>()))
    }
};

const STRLEN : FunctionDefinition = FunctionDefinition {
    name: "strlen",
    category: Some("strings"),
    description: "Returns the length of the string s",
    arguments: || vec![
        FunctionArgument::new_required("s", ExpectedTypes::String)
    ],
    handler: |_function, _token, _state, args| {
        let s = args.get("s").required().as_string();
        Ok(Value::Integer(s.len() as IntegerType))
    }
};

const UPPERCASE : FunctionDefinition = FunctionDefinition {
    name: "uppercase",
    category: Some("strings"),
    description: "Converts the string s to uppercase",
    arguments: || vec![
        FunctionArgument::new_required("s", ExpectedTypes::String)
    ],
    handler: |_function, _token, _state, args| {
        let s = args.get("s").required().as_string();
        Ok(Value::String(s.to_uppercase()))
    }
};

const LOWERCASE : FunctionDefinition = FunctionDefinition {
    name: "lowercase",
    category: Some("strings"),
    description: "Converts the string s to lowercase",
    arguments: || vec![
        FunctionArgument::new_required("s", ExpectedTypes::String)
    ],
    handler: |_function, _token, _state, args| {
        let s = args.get("s").required().as_string();
        Ok(Value::String(s.to_lowercase()))
    }
};

const TRIM : FunctionDefinition = FunctionDefinition {
    name: "trim",
    category: Some("strings"),
    description: "Trim whitespace from a string",
    arguments: || vec![
        FunctionArgument::new_required("s", ExpectedTypes::String)
    ],
    handler: |_function, _token, _state, args| {
        let s = args.get("s").required().as_string();
        Ok(Value::String(s.trim().to_string()))
    }
};

const SUBSTR : FunctionDefinition = FunctionDefinition {
    name: "substr",
    category: Some("strings"),
    description: "Returns a substring from s, beginning at [start], and going to the end, or for [length] characters",
    arguments: || vec![
        FunctionArgument::new_required("s", ExpectedTypes::String),
        FunctionArgument::new_required("start", ExpectedTypes::IntOrFloat),
        FunctionArgument::new_optional("length", ExpectedTypes::IntOrFloat)
    ],
    handler: |function, token, _state, args| {
        let s = args.get("s").required().as_string();
        let start = args.get("start").required().as_int().unwrap_or(0);
        let default_len = s.len() as IntegerType - start;
        let length = match args.get("length").optional() {
            Some(l) => l,
            None => Value::Integer(default_len)
        }.as_int().unwrap_or(default_len);
        
        if start >= s.len() as IntegerType || start < 0 {
            return Err(Error::FunctionArgumentOverflow { 
                arg: 2, 
                signature: function.signature(),
                token: token.clone()
            });
        } else if length < 0 || length > (s.len() - start as usize) as IntegerType {
            return Err(Error::FunctionArgumentOverflow { 
                arg: 3, 
                signature: function.signature(),
                token: token.clone()
            });
        }

        Ok(Value::String(s.chars().skip(start as usize).take(length as usize).collect()))
    }
};

const REGEX : FunctionDefinition = FunctionDefinition {
    name: "regex",
    category: Some("strings"),
    description: "Returns a regular expression match from [subject], or false",
    arguments: || vec![
        FunctionArgument::new_required("pattern", ExpectedTypes::String),
        FunctionArgument::new_required("subject", ExpectedTypes::String),
        FunctionArgument::new_optional("group", ExpectedTypes::Int)
    ],
    handler: |_function, token, _state, args| {
        let pattern = args.get("pattern").required().as_string();
        let subject = args.get("subject").required().as_string();
        let group = match args.get("group").optional() {
            Some(g) => g.as_int(),
            None => None
        };

        let re = Regex::new(&pattern);
        if let Err(_) = re {
            return Err(Error::StringFormat { expected_format: "regex".to_string(), token: token.clone() });
        }
    
        if let Some(caps) = re.unwrap().captures(&subject) {
            match group {
                Some(g) => {
                    let group_index = g;
                    if let Some(group) = caps.get(group_index as usize) {
                        return Ok(Value::String(group.as_str().to_string()));
                    }
                },
                None => {
                    return Ok(Value::String(caps.get(0).unwrap().as_str().to_string()));
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
        let mut state = ParserState::new();

        assert_eq!(Value::Boolean(false), REGEX.call(&Token::dummy(""), &mut state, &[
            Value::String("test".to_string()), Value::String("bar".to_string())
        ]).unwrap());
        assert_eq!(Value::String("foo".to_string()), REGEX.call(&Token::dummy(""), &mut state, &[
            Value::String("foo".to_string()), Value::String("foobar".to_string())
        ]).unwrap());
        assert_eq!(Value::String("foobar".to_string()), REGEX.call(&Token::dummy(""), &mut state, &[
            Value::String("foo.*".to_string()), Value::String("foobar".to_string())
        ]).unwrap());
        assert_eq!(Value::String("bar".to_string()), REGEX.call(&Token::dummy(""), &mut state, &[
            Value::String("foo(.*)".to_string()), Value::String("foobar".to_string()), 
            Value::Integer(1)
        ]).unwrap());
        assert_eq!(Value::String("foobar".to_string()), REGEX.call(&Token::dummy(""), &mut state, &[
            Value::String("foo(.*)".to_string()), Value::String("foobar".to_string()), 
            Value::Integer(0)
        ]).unwrap());
        assert_eq!(Value::Boolean(false), REGEX.call(&Token::dummy(""), &mut state, &[
            Value::String("foo(.*)".to_string()), Value::String("foobar".to_string()), 
            Value::Integer(6)
        ]).unwrap());
    }

    #[test]
    fn test_strlen() {
        let mut state = ParserState::new();

        assert_eq!(Value::Integer(0), STRLEN.call(&Token::dummy(""), &mut state, 
            &[Value::String("".to_string())]).unwrap());
        assert_eq!(Value::Integer(3), STRLEN.call(&Token::dummy(""), &mut state, 
            &[Value::String("   ".to_string())]).unwrap());
    }

    #[test]
    fn test_uppercase() {
        let mut state = ParserState::new();

        assert_eq!(Value::String("TEST".to_string()), UPPERCASE.call(&Token::dummy(""), &mut state, 
            &[Value::String("test".to_string())]).unwrap());
        assert_eq!(Value::String(" TEST  ".to_string()), UPPERCASE.call(&Token::dummy(""), &mut state, 
            &[Value::String(" test  ".to_string())]).unwrap());
    }


    #[test]
    fn test_lowercase() {
        let mut state = ParserState::new();

        assert_eq!(Value::String("test".to_string()), LOWERCASE.call(&Token::dummy(""), &mut state, 
            &[Value::String("TEST".to_string())]).unwrap());
        assert_eq!(Value::String(" test  ".to_string()), LOWERCASE.call(&Token::dummy(""), &mut state, 
            &[Value::String(" TEST  ".to_string())]).unwrap());
    }


    #[test]
    fn test_trim() {
        let mut state = ParserState::new();

        assert_eq!(Value::String("test".to_string()), TRIM.call(&Token::dummy(""), &mut state, 
            &[Value::String("test".to_string())]).unwrap());
        assert_eq!(Value::String("TEST".to_string()), TRIM.call(&Token::dummy(""), &mut state, 
            &[Value::String(" TEST  ".to_string())]).unwrap());
    }

    #[test]
    fn test_concat() {
        let mut state = ParserState::new();

        assert_eq!(Value::String(" ".to_string()), CONCAT.call(
            &Token::dummy(""), 
            &mut state, &[Value::String("".to_string()), 
            Value::String(" ".to_string())
        ]).unwrap());
        assert_eq!(Value::String("test4false".to_string()), CONCAT.call(
            &Token::dummy(""), 
            &mut state, &[Value::String("test".to_string()), 
            Value::Integer(4),
            Value::Boolean(false)
        ]).unwrap());
    }
    
    #[test]
    fn test_substr() {
        let mut state = ParserState::new();

        assert_eq!(Value::String("t".to_string()), 
            SUBSTR.call(&Token::dummy(""), &mut state, &[Value::String("test".to_string()), Value::Integer(3)]).unwrap()
        );
        assert_eq!(Value::String("tes".to_string()), 
            SUBSTR.call(&Token::dummy(""), &mut state, &[Value::String("test".to_string()), Value::Integer(0), Value::Integer(3)]).unwrap()
        );
    }
    
    #[test]
    fn test_contains() {
        let mut state = ParserState::new();

        assert_eq!(Value::Boolean(true), CONTAINS.call(&Token::dummy(""), &mut state, &[
            Value::String("test".to_string()),
            Value::String("e".to_string())
        ]).unwrap());

        assert_eq!(Value::Boolean(false), CONTAINS.call(&Token::dummy(""), &mut state, &[
            Value::String("test".to_string()),
            Value::String("fff".to_string())
        ]).unwrap());

        assert_eq!(Value::Boolean(true), CONTAINS.call(&Token::dummy(""), &mut state, &[
            Value::Array(vec![
                Value::Integer(5),
                Value::Integer(3),
            ]),
            Value::Integer(5)
        ]).unwrap());

        assert_eq!(Value::Boolean(false), CONTAINS.call(&Token::dummy(""), &mut state, &[
            Value::Array(vec![
                Value::Integer(5),
                Value::Integer(3),
            ]),
            Value::Integer(4)
        ]).unwrap());
    }
}