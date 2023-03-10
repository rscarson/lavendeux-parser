//! Builtin functions for array manipulation

use super::*;
use crate::value::{Value, IntegerType, ArrayType};

const LEN : FunctionDefinition = FunctionDefinition {
    name: "len",
    category: Some("arrays"),
    description: "Returns the length of the given array",
    arguments: || vec![
        FunctionArgument::new_required("array", ExpectedTypes::Array)
    ],
    handler: |_function, _state, args| {
        Ok(Value::Integer(
            args.get("array").required().as_array().len() as IntegerType
        ))
    }
};

const IS_EMPTY : FunctionDefinition = FunctionDefinition {
    name: "is_empty",
    category: Some("arrays"),
    description: "Returns true if the given array is empty",
    arguments: || vec![
        FunctionArgument::new_required("array", ExpectedTypes::Array)
    ],
    handler: |_function, _state, args| {
        Ok(Value::Boolean(
            args.get("array").required().as_array().is_empty()
        ))
    }
};

const POP : FunctionDefinition = FunctionDefinition {
    name: "pop",
    category: Some("arrays"),
    description: "Remove the last element from an array",
    arguments: || vec![
        FunctionArgument::new_required("array", ExpectedTypes::Array)
    ],
    handler: |_function, _state, args| {
        let mut e = args.get("array").required().as_array();
        if e.is_empty() {
            Err(ParserError::ArrayLength(ArrayLengthError::new()))
        } else {
            e.pop();
            Ok(Value::Array(e))
        }
    }
};

const PUSH : FunctionDefinition = FunctionDefinition {
    name: "push",
    category: Some("arrays"),
    description: "Add an element to the end of an array",
    arguments: || vec![
        FunctionArgument::new_required("array", ExpectedTypes::Array),
        FunctionArgument::new_required("element", ExpectedTypes::Any)
    ],
    handler: |_function, _state, args| {
        let mut e = args.get("array").required().as_array();
        e.push(args.get("element").required().clone());
        Ok(Value::Array(e))
    }
};

const DEQUEUE : FunctionDefinition = FunctionDefinition {
    name: "dequeue",
    category: Some("arrays"),
    description: "Remove the first element from an array",
    arguments: || vec![
        FunctionArgument::new_required("array", ExpectedTypes::Array)
    ],
    handler: |_function, _state, args| {
        let mut e = args.get("array").required().as_array();
        if e.is_empty() {
            Err(ParserError::ArrayLength(ArrayLengthError::new()))
        } else {
            e.remove(0);
            Ok(Value::Array(e))
        }
    }
};

const ENQUEUE : FunctionDefinition = FunctionDefinition {
    name: "enqueue",
    category: Some("arrays"),
    description: "Add an element to the end of an array",
    arguments: || vec![
        FunctionArgument::new_required("array", ExpectedTypes::Array),
        FunctionArgument::new_required("element", ExpectedTypes::Any)
    ],
    handler: |_function, _state, args| {
        let mut e = args.get("array").required().as_array();
        e.push(args.get("element").required().clone());
        Ok(Value::Array(e))
    }
};

const REMOVE : FunctionDefinition = FunctionDefinition {
    name: "remove",
    category: Some("arrays"),
    description: "Removes an element from an array",
    arguments: || vec![
        FunctionArgument::new_required("array", ExpectedTypes::Array),
        FunctionArgument::new_required("index", ExpectedTypes::Int)
    ],
    handler: |_function, _state, args| {
        let mut a = args.get("array").required().as_array();
        let idx = args.get("index").required().as_int().unwrap();
        if idx < 0 || idx >= a.len() as IntegerType {
            Err(ParserError::ArrayIndex(ArrayIndexError::new(idx as usize)))
        } else {
            a.remove(idx as usize);
            Ok(Value::Array(a))
        }
    }
};

const ELEMENT : FunctionDefinition = FunctionDefinition {
    name: "element",
    category: Some("arrays"),
    description: "Return an element from a location in an array",
    arguments: || vec![
        FunctionArgument::new_required("array", ExpectedTypes::Array),
        FunctionArgument::new_required("index", ExpectedTypes::Int)
    ],
    handler: |_function, _state, args| {
        let a = args.get("array").required().as_array();
        let idx = args.get("index").required().as_int().unwrap();
        if idx < 0 || idx >= a.len() as IntegerType {
            Err(ParserError::ArrayIndex(ArrayIndexError::new(idx as usize)))
        } else {
            Ok(a[idx as usize].clone())
        }
    }
};

const MERGE :FunctionDefinition = FunctionDefinition {
    name: "merge",
    category: Some("arrays"),
    description: "Merge all given arrays",
    arguments: || vec![
        FunctionArgument::new_plural("arrays", ExpectedTypes::Any, false)
    ],
    handler: |_function, _state, args| {
        let mut result : ArrayType = vec![];
        for arg in args.get("arrays").plural() {
            result.append(&mut arg.as_array());
        }
        Ok(Value::Array(result))
    }
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
}

#[cfg(test)]
mod test_builtin_functions {
    use super::*;

    #[test]
    fn test_len() {
        let mut state = ParserState::new();

        assert_eq!(Value::Integer(1), LEN.call(&mut state, &[
            Value::Array(vec![
                Value::Integer(5), 
            ])
        ]).unwrap());
        assert_eq!(Value::Integer(3), LEN.call(&mut state, &[
            Value::Array(vec![
                Value::Integer(5), 
                Value::Float(2.0), 
                Value::String("test".to_string())
            ])
        ]).unwrap());
    }

    #[test]
    fn test_is_empty() {
        let mut state = ParserState::new();

        assert_eq!(Value::Boolean(false), IS_EMPTY.call(&mut state, &[
            Value::Array(vec![
                Value::Integer(5), 
            ])
        ]).unwrap());
        assert_eq!(Value::Boolean(true), IS_EMPTY.call(&mut state, &[
            Value::Array(vec![
            ])
        ]).unwrap());
    }

    #[test]
    fn test_pop() {
        let mut state = ParserState::new();

        assert_eq!(Value::Array(vec![
            Value::Integer(5)
        ]), POP.call(&mut state, &[
            Value::Array(vec![
                Value::Integer(5), Value::Integer(3), 
            ])
        ]).unwrap());
    }

    #[test]
    fn test_push() {
        let mut state = ParserState::new();

        assert_eq!(Value::Array(vec![
            Value::Integer(5), Value::Integer(3), 
        ]), PUSH.call(&mut state, &[
            Value::Array(vec![
                Value::Integer(5), 
            ]), Value::Integer(3)
        ]).unwrap());
    }

    #[test]
    fn test_dequeue() {
        let mut state = ParserState::new();

        assert_eq!(Value::Array(vec![
            Value::Integer(3), 
        ]), DEQUEUE.call(&mut state, &[
            Value::Array(vec![
                Value::Integer(5), Value::Integer(3), 
            ])
        ]).unwrap());
    }

    #[test]
    fn test_enqueue() {
        let mut state = ParserState::new();

        assert_eq!(Value::Array(vec![
            Value::Integer(5), Value::Integer(3), 
        ]), ENQUEUE.call(&mut state, &[
            Value::Array(vec![
                Value::Integer(5), 
            ]), Value::Integer(3)
        ]).unwrap());
    }

    #[test]
    fn test_remove() {
        let mut state = ParserState::new();

        assert_eq!(Value::Array(vec![Value::Integer(3)]), REMOVE.call(&mut state, &[
            Value::Array(vec![
                Value::Integer(5), Value::Integer(3), 
            ]),
            Value::Integer(0)
        ]).unwrap());
        assert_eq!(Value::Array(vec![Value::Integer(5)]), REMOVE.call(&mut state, &[
            Value::Array(vec![
                Value::Integer(5), Value::Integer(3), 
            ]),
            Value::Integer(1)
        ]).unwrap());
        assert_eq!(true, REMOVE.call(&mut state, &[
            Value::Array(vec![
                Value::Integer(5), Value::Integer(3), 
            ]),
            Value::Integer(2)
        ]).is_err());
    }

    #[test]
    fn test_element() {
        let mut state = ParserState::new();

        assert_eq!(Value::Integer(3), ELEMENT.call(&mut state, &[
            Value::Array(vec![
                Value::Integer(5), 
                Value::Integer(3), 
            ]),
            Value::Integer(1)
        ]).unwrap());
    }

    #[test]
    fn test_merge() {
        let mut state = ParserState::new();

        assert_eq!(Value::Array(
            vec![Value::Integer(1), Value::Integer(2), Value::Integer(3), Value::Integer(4)]), 
            MERGE.call(&mut state, &[
                Value::Array(vec![Value::Integer(1)]), 
                Value::Array(vec![Value::Integer(2), Value::Integer(3)]), 
                Value::Integer(4)
            ]).unwrap());
    }
}