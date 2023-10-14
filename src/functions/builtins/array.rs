//! Builtin functions for array manipulation

use super::*;
use crate::{
    value::{ArrayType, IntegerType, Value},
    ExpectedTypes,
};

const LEN: FunctionDefinition = FunctionDefinition {
    name: "len",
    category: Some("arrays"),
    description: "Returns the length of the given array or object",
    arguments: || {
        vec![FunctionArgument::new_required(
            "input",
            ExpectedTypes::Array,
        )]
    },
    handler: |_function, _token, _state, args| {
        Ok(Value::Integer(match args.get("input").required() {
            Value::Object(v) => v.keys().len() as IntegerType,
            _ => args.get("input").required().as_array().len() as IntegerType,
        }))
    },
};

const IS_EMPTY: FunctionDefinition = FunctionDefinition {
    name: "is_empty",
    category: Some("arrays"),
    description: "Returns true if the given array or object is empty",
    arguments: || {
        vec![FunctionArgument::new_required(
            "input",
            ExpectedTypes::Array,
        )]
    },
    handler: |_function, _token, _state, args| {
        Ok(Value::Boolean(match args.get("input").required() {
            Value::Object(v) => v.is_empty(),
            _ => args.get("input").required().as_array().is_empty(),
        }))
    },
};

const POP: FunctionDefinition = FunctionDefinition {
    name: "pop",
    category: Some("arrays"),
    description: "Remove the last element from an array",
    arguments: || {
        vec![FunctionArgument::new_required(
            "array",
            ExpectedTypes::Array,
        )]
    },
    handler: |_function, token, _state, args| {
        let mut e = args.get("array").required().as_array();
        if e.is_empty() {
            Err(Error::ArrayEmpty(token.clone()))
        } else {
            e.pop();
            Ok(Value::Array(e))
        }
    },
};

const PUSH: FunctionDefinition = FunctionDefinition {
    name: "push",
    category: Some("arrays"),
    description: "Add an element to the end of an array",
    arguments: || {
        vec![
            FunctionArgument::new_required("array", ExpectedTypes::Array),
            FunctionArgument::new_required("element", ExpectedTypes::Any),
        ]
    },
    handler: |_function, _token, _state, args| {
        let mut e = args.get("array").required().as_array();
        e.push(args.get("element").required());
        Ok(Value::Array(e))
    },
};

const DEQUEUE: FunctionDefinition = FunctionDefinition {
    name: "dequeue",
    category: Some("arrays"),
    description: "Remove the first element from an array",
    arguments: || {
        vec![FunctionArgument::new_required(
            "array",
            ExpectedTypes::Array,
        )]
    },
    handler: |_function, token, _state, args| {
        let mut e = args.get("array").required().as_array();
        if e.is_empty() {
            Err(Error::ArrayEmpty(token.clone()))
        } else {
            e.remove(0);
            Ok(Value::Array(e))
        }
    },
};

const ENQUEUE: FunctionDefinition = FunctionDefinition {
    name: "enqueue",
    category: Some("arrays"),
    description: "Add an element to the end of an array",
    arguments: || {
        vec![
            FunctionArgument::new_required("array", ExpectedTypes::Array),
            FunctionArgument::new_required("element", ExpectedTypes::Any),
        ]
    },
    handler: |_function, _token, _state, args| {
        let mut e = args.get("array").required().as_array();
        e.push(args.get("element").required());
        Ok(Value::Array(e))
    },
};

const REMOVE: FunctionDefinition = FunctionDefinition {
    name: "remove",
    category: Some("arrays"),
    description: "Removes an element from an array",
    arguments: || {
        vec![
            FunctionArgument::new_required("input", ExpectedTypes::Array),
            FunctionArgument::new_required("index", ExpectedTypes::Int),
        ]
    },
    handler: |_function, token, _state, args| {
        let input = args.get("input").required();
        let index = args.get("index").required();

        match input {
            Value::Object(mut v) => {
                v.remove(&index);
                Ok(Value::from(v))
            }
            _ => {
                let mut a = input.as_array();
                let idx = index.as_int().unwrap();
                if idx < 0 || idx >= a.len() as IntegerType {
                    Err(Error::Index {
                        key: index,
                        token: token.clone(),
                    })
                } else {
                    a.remove(idx as usize);
                    Ok(Value::Array(a))
                }
            }
        }
    },
};

const ELEMENT: FunctionDefinition = FunctionDefinition {
    name: "element",
    category: Some("arrays"),
    description: "Return an element from a location in an array or object",
    arguments: || {
        vec![
            FunctionArgument::new_required("input", ExpectedTypes::Array),
            FunctionArgument::new_required("index", ExpectedTypes::Int),
        ]
    },
    handler: |_function, token, _state, args| {
        let input = args.get("input").required();
        let index = args.get("index").required();

        match input {
            Value::Object(v) => match v.get(&index) {
                None => Err(Error::Index {
                    key: index,
                    token: token.clone(),
                }),
                Some(v) => Ok(v.clone()),
            },
            _ => {
                let a = input.as_array();
                let idx = index.as_int().unwrap();
                if idx < 0 || idx > a.len() as IntegerType {
                    Err(Error::Index {
                        key: index,
                        token: token.clone(),
                    })
                } else {
                    Ok(a[idx as usize].clone())
                }
            }
        }
    },
};

const MERGE: FunctionDefinition = FunctionDefinition {
    name: "merge",
    category: Some("arrays"),
    description: "Merge all given arrays or objects",
    arguments: || {
        vec![
            FunctionArgument::new("target", ExpectedTypes::Any, false),
            FunctionArgument::new_plural("inputs", ExpectedTypes::Any, false),
        ]
    },
    handler: |_function, _token, _state, args| match args.get("target").required() {
        Value::Object(mut v) => {
            for arg in args.get("inputs").plural() {
                v.extend(arg.as_object());
            }
            Ok(Value::Object(v))
        }

        _ => {
            let mut result: ArrayType = args.get("target").required().as_array();
            for arg in args.get("inputs").plural() {
                result.append(&mut arg.as_array());
            }
            Ok(Value::Array(result))
        }
    },
};

const KEYS: FunctionDefinition = FunctionDefinition {
    name: "keys",
    category: Some("arrays"),
    description: "Get a list of keys in the object or array",
    arguments: || vec![FunctionArgument::new("input", ExpectedTypes::Any, false)],
    handler: |_function, _token, _state, args| {
        let mut a = args
            .get("input")
            .required()
            .as_object()
            .keys()
            .cloned()
            .collect::<ArrayType>();
        a.sort();
        Ok(Value::Array(a))
    },
};

const VALUES: FunctionDefinition = FunctionDefinition {
    name: "values",
    category: Some("arrays"),
    description: "Get a list of values in the object or array",
    arguments: || vec![FunctionArgument::new("input", ExpectedTypes::Any, false)],
    handler: |_function, _token, _state, args| {
        let mut a = args
            .get("input")
            .required()
            .as_object()
            .values()
            .cloned()
            .collect::<ArrayType>();
        a.sort();
        Ok(Value::Array(a))
    },
};

/// Register array functions
pub fn register_functions(table: &mut FunctionTable) {
    table.register(LEN);
    table.register(IS_EMPTY);
    table.register(POP);
    table.register(PUSH);
    table.register(DEQUEUE);
    table.register(ENQUEUE);
    table.register(REMOVE);
    table.register(ELEMENT);
    table.register(MERGE);
    table.register(KEYS);
    table.register(VALUES);
}

#[cfg(test)]
mod test_builtin_functions {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_len() {
        let mut state = ParserState::new();

        assert_eq!(
            Value::Integer(1),
            LEN.call(
                &Token::dummy(""),
                &mut state,
                &[Value::Array(vec![Value::Integer(5),])]
            )
            .unwrap()
        );
        assert_eq!(
            Value::Integer(3),
            LEN.call(
                &Token::dummy(""),
                &mut state,
                &[Value::Array(vec![
                    Value::Integer(5),
                    Value::Float(2.0),
                    Value::String("test".to_string())
                ])]
            )
            .unwrap()
        );
    }

    #[test]
    fn test_is_empty() {
        let mut state = ParserState::new();

        assert_eq!(
            Value::Boolean(false),
            IS_EMPTY
                .call(
                    &Token::dummy(""),
                    &mut state,
                    &[Value::Array(vec![Value::Integer(5),])]
                )
                .unwrap()
        );
        assert_eq!(
            Value::Boolean(true),
            IS_EMPTY
                .call(&Token::dummy(""), &mut state, &[Value::Array(vec![])])
                .unwrap()
        );
    }

    #[test]
    fn test_pop() {
        let mut state = ParserState::new();

        assert_eq!(
            Value::Array(vec![Value::Integer(5)]),
            POP.call(
                &Token::dummy(""),
                &mut state,
                &[Value::Array(vec![Value::Integer(5), Value::Integer(3),])]
            )
            .unwrap()
        );
    }

    #[test]
    fn test_push() {
        let mut state = ParserState::new();

        assert_eq!(
            Value::Array(vec![Value::Integer(5), Value::Integer(3),]),
            PUSH.call(
                &Token::dummy(""),
                &mut state,
                &[Value::Array(vec![Value::Integer(5),]), Value::Integer(3)]
            )
            .unwrap()
        );
    }

    #[test]
    fn test_dequeue() {
        let mut state = ParserState::new();

        assert_eq!(
            Value::Array(vec![Value::Integer(3),]),
            DEQUEUE
                .call(
                    &Token::dummy(""),
                    &mut state,
                    &[Value::Array(vec![Value::Integer(5), Value::Integer(3),])]
                )
                .unwrap()
        );
    }

    #[test]
    fn test_enqueue() {
        let mut state = ParserState::new();

        assert_eq!(
            Value::Array(vec![Value::Integer(5), Value::Integer(3),]),
            ENQUEUE
                .call(
                    &Token::dummy(""),
                    &mut state,
                    &[Value::Array(vec![Value::Integer(5),]), Value::Integer(3)]
                )
                .unwrap()
        );
    }

    #[test]
    fn test_remove() {
        let mut state = ParserState::new();

        assert_eq!(
            Value::Array(vec![Value::Integer(3)]),
            REMOVE
                .call(
                    &Token::dummy(""),
                    &mut state,
                    &[
                        Value::Array(vec![Value::Integer(5), Value::Integer(3),]),
                        Value::Integer(0)
                    ]
                )
                .unwrap()
        );
        assert_eq!(
            Value::Array(vec![Value::Integer(5)]),
            REMOVE
                .call(
                    &Token::dummy(""),
                    &mut state,
                    &[
                        Value::Array(vec![Value::Integer(5), Value::Integer(3),]),
                        Value::Integer(1)
                    ]
                )
                .unwrap()
        );
        assert_eq!(
            true,
            REMOVE
                .call(
                    &Token::dummy(""),
                    &mut state,
                    &[
                        Value::Array(vec![Value::Integer(5), Value::Integer(3),]),
                        Value::Integer(2)
                    ]
                )
                .is_err()
        );
    }

    #[test]
    fn test_element() {
        let mut state = ParserState::new();

        assert_eq!(
            Value::Integer(3),
            ELEMENT
                .call(
                    &Token::dummy(""),
                    &mut state,
                    &[
                        Value::Array(vec![Value::Integer(5), Value::Integer(3),]),
                        Value::Integer(1)
                    ]
                )
                .unwrap()
        );
    }

    #[test]
    fn test_merge() {
        let mut state = ParserState::new();

        assert_eq!(
            Value::Array(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3),
                Value::Integer(4)
            ]),
            MERGE
                .call(
                    &Token::dummy(""),
                    &mut state,
                    &[
                        Value::Array(vec![Value::Integer(1)]),
                        Value::Array(vec![Value::Integer(2), Value::Integer(3)]),
                        Value::Integer(4)
                    ]
                )
                .unwrap()
        );
    }

    #[test]
    fn test_keys() {
        let mut state = ParserState::new();

        assert_eq!(
            Value::Array(vec![Value::Integer(1), Value::String("2".to_string())]),
            KEYS.call(
                &Token::dummy(""),
                &mut state,
                &[Value::Object(HashMap::from([
                    (Value::Integer(1), Value::Integer(3)),
                    (
                        Value::String("2".to_string()),
                        Value::String("4".to_string())
                    ),
                ]))]
            )
            .unwrap()
        );
    }

    #[test]
    fn test_values() {
        let mut state = ParserState::new();

        assert_eq!(
            Value::Array(vec![Value::Integer(3), Value::String("4".to_string())]),
            VALUES
                .call(
                    &Token::dummy(""),
                    &mut state,
                    &[Value::Object(HashMap::from([
                        (Value::Integer(1), Value::Integer(3)),
                        (
                            Value::String("2".to_string()),
                            Value::String("4".to_string())
                        ),
                    ]))]
                )
                .unwrap()
        );
    }
}
