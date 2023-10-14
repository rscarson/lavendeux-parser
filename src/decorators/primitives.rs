use crate::{DecoratorDefinition, ExpectedTypes, Value};

use super::pluralized_decorator;

pub const DEFAULT: DecoratorDefinition = DecoratorDefinition {
    name: &["default"],
    description: "Default formatter, type dependent",
    argument: ExpectedTypes::Any,
    handler: |_, token, input| match input {
        Value::Boolean(_) => (BOOL.handler)(&BOOL, token, input),
        Value::Integer(_) => (INT.handler)(&INT, token, input),
        Value::Float(_) => (FLOAT.handler)(&FLOAT, token, input),
        Value::Array(_) => (ARRAY.handler)(&ARRAY, token, input),
        Value::Object(_) => (OBJECT.handler)(&OBJECT, token, input),
        Value::String(s) => Ok(s.to_string()),
        Value::Identifier(_) => Ok("".to_string()),
        Value::None => Ok("".to_string()),
    },
};

pub const FLOAT: DecoratorDefinition = DecoratorDefinition {
    name: &["float"],
    description: "Format a number as floating point",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            Ok(Value::Float(input.as_float().unwrap()).as_string())
        } else {
            pluralized_decorator(decorator, token, input)
        }
    },
};

pub const INT: DecoratorDefinition = DecoratorDefinition {
    name: &["int", "integer"],
    description: "Format a number as an integer",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            Ok(Value::Integer(input.as_int().unwrap()).as_string())
        } else {
            pluralized_decorator(decorator, token, input)
        }
    },
};

pub const BOOL: DecoratorDefinition = DecoratorDefinition {
    name: &["bool", "boolean"],
    description: "Format a number as a boolean",
    argument: ExpectedTypes::Any,
    handler: |_, _, input| Ok(Value::Boolean(input.as_bool()).as_string()),
};

pub const ARRAY: DecoratorDefinition = DecoratorDefinition {
    name: &["array"],
    description: "Format a number as an array",
    argument: ExpectedTypes::Any,
    handler: |_, _, input| Ok(Value::Array(input.as_array()).as_string()),
};

pub const OBJECT: DecoratorDefinition = DecoratorDefinition {
    name: &["object"],
    description: "Format a number as an object",
    argument: ExpectedTypes::Any,
    handler: |_, _, input| Ok(Value::Object(input.as_object()).as_string()),
};

#[cfg(test)]
mod test_builtin_functions {
    use crate::Token;

    use super::*;

    #[test]
    fn test_float() {
        assert_eq!(
            "8.0",
            FLOAT.call(&Token::dummy(""), &Value::Integer(8)).unwrap()
        );
        assert_eq!(
            "81.0",
            FLOAT.call(&Token::dummy(""), &Value::Float(81.0)).unwrap()
        );
        assert_eq!(
            "0.0",
            FLOAT
                .call(&Token::dummy(""), &Value::Float(0.0000000001))
                .unwrap()
        );
        assert_eq!(
            "0.081",
            FLOAT.call(&Token::dummy(""), &Value::Float(0.081)).unwrap()
        );
    }

    #[test]
    fn test_int() {
        assert_eq!(
            "-8",
            INT.call(&Token::dummy(""), &Value::Integer(-8)).unwrap()
        );
        assert_eq!(
            "81",
            INT.call(&Token::dummy(""), &Value::Float(81.0)).unwrap()
        );
        assert_eq!(
            "0",
            INT.call(&Token::dummy(""), &Value::Float(0.081)).unwrap()
        );
    }

    #[test]
    fn test_bool() {
        assert_eq!(
            "false",
            BOOL.call(&Token::dummy(""), &Value::Integer(0)).unwrap()
        );
        assert_eq!(
            "true",
            BOOL.call(&Token::dummy(""), &Value::Integer(81)).unwrap()
        );
        assert_eq!(
            "true",
            BOOL.call(&Token::dummy(""), &Value::Float(0.081)).unwrap()
        );
    }
}
