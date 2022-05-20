use super::{FunctionDefinition, FunctionArgument, FunctionTable};
use crate::value::{Value, IntegerType, ArrayType};
use crate::errors::*;

const LENGTH : FunctionDefinition = FunctionDefinition {
    name: "length",
    description: "Returns the length of the given array",
    arguments: || vec![
        FunctionArgument::new_required("array", ExpectedTypes::Array)
    ],
    handler: |_, args: &[Value]| {
        Ok(Value::Integer(args[0].as_array().len() as IntegerType))
    }
};

const IS_EMPTY : FunctionDefinition = FunctionDefinition {
    name: "is_empty",
    description: "Returns true if the given array is empty",
    arguments: || vec![
        FunctionArgument::new_required("array", ExpectedTypes::Array)
    ],
    handler: |_, args: &[Value]| {
        Ok(Value::Boolean(args[0].as_array().is_empty()))
    }
};

const POP : FunctionDefinition = FunctionDefinition {
    name: "pop",
    description: "Return the last element from an array",
    arguments: || vec![
        FunctionArgument::new_required("array", ExpectedTypes::Array)
    ],
    handler: |_, args: &[Value]| {
        let mut e = args[0].as_array();
        if e.is_empty() {
            Err(ParserError::ArrayLength(ArrayLengthError::new()))
        } else {
            Ok(e.pop().unwrap())
        }
    }
};

const PUSH : FunctionDefinition = FunctionDefinition {
    name: "push",
    description: "Add an element to the end of an array",
    arguments: || vec![
        FunctionArgument::new_required("array", ExpectedTypes::Array),
        FunctionArgument::new_required("element", ExpectedTypes::Any)
    ],
    handler: |_, args: &[Value]| {
        let mut e = args[0].as_array();
        e.push(args[1].clone());
        Ok(Value::Array(e))
    }
};

const DEQUEUE : FunctionDefinition = FunctionDefinition {
    name: "dequeue",
    description: "Return the first element from an array",
    arguments: || vec![
        FunctionArgument::new_required("array", ExpectedTypes::Array)
    ],
    handler: |_, args: &[Value]| {
        let e = args[0].as_array();
        if e.is_empty() {
            Err(ParserError::ArrayLength(ArrayLengthError::new()))
        } else {
            Ok(e[0].clone())
        }
    }
};

const ENQUEUE : FunctionDefinition = FunctionDefinition {
    name: "enqueue",
    description: "Add an element to the end of an array",
    arguments: || vec![
        FunctionArgument::new_required("array", ExpectedTypes::Array),
        FunctionArgument::new_required("element", ExpectedTypes::Any)
    ],
    handler: |_, args: &[Value]| {
        let mut e = args[0].as_array();
        e.push(args[1].clone());
        Ok(Value::Array(e))
    }
};

const REMOVE : FunctionDefinition = FunctionDefinition {
    name: "remove",
    description: "Removes an element from an array",
    arguments: || vec![
        FunctionArgument::new_required("array", ExpectedTypes::Array),
        FunctionArgument::new_required("index", ExpectedTypes::Int)
    ],
    handler: |_, args: &[Value]| {
        let mut a = args[0].as_array();
        let idx = args[1].as_int().unwrap();
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
    description: "Return an element from a location in an array",
    arguments: || vec![
        FunctionArgument::new_required("array", ExpectedTypes::Array),
        FunctionArgument::new_required("index", ExpectedTypes::Int)
    ],
    handler: |_, args: &[Value]| {
        let a = args[0].as_array();
        let idx = args[1].as_int().unwrap();
        if idx < 0 || idx >= a.len() as IntegerType {
            Err(ParserError::ArrayIndex(ArrayIndexError::new(idx as usize)))
        } else {
            Ok(a[idx as usize].clone())
        }
    }
};

const MERGE :FunctionDefinition = FunctionDefinition {
    name: "merge",
    description: "Merge all given arrays",
    arguments: || vec![
        FunctionArgument::new_plural("arrays", ExpectedTypes::Any, false)
    ],
    handler: |_, args: &[Value]| {
        let mut result : ArrayType = vec![];
        for arg in args {
            result.append(&mut arg.as_array());
        }
        Ok(Value::Array(result))
    }
};

/// Register array functions
pub fn register_functions(table: &mut FunctionTable) {
    table.register(LENGTH);
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
    fn test_length() {
        assert_eq!(Value::Integer(1), (LENGTH.handler)(&LENGTH, &[
            Value::Array(vec![
                Value::Integer(5), 
            ])
        ]).unwrap());
        assert_eq!(Value::Integer(3), (LENGTH.handler)(&LENGTH, &[
            Value::Array(vec![
                Value::Integer(5), 
                Value::Float(2.0), 
                Value::String("test".to_string())
            ])
        ]).unwrap());
    }

    #[test]
    fn test_is_empty() {
        assert_eq!(Value::Boolean(false), (IS_EMPTY.handler)(&IS_EMPTY, &[
            Value::Array(vec![
                Value::Integer(5), 
            ])
        ]).unwrap());
        assert_eq!(Value::Boolean(true), (IS_EMPTY.handler)(&IS_EMPTY, &[
            Value::Array(vec![])
        ]).unwrap());
    }

    #[test]
    fn test_pop() {
        assert_eq!(Value::Integer(3), (POP.handler)(&POP, &[
            Value::Array(vec![
                Value::Integer(5), Value::Integer(3), 
            ])
        ]).unwrap());
    }

    #[test]
    fn test_push() {
        assert_eq!(Value::Array(vec![
            Value::Integer(5), Value::Integer(3), 
        ]), (PUSH.handler)(&PUSH, &[
            Value::Array(vec![
                Value::Integer(5), 
            ]), Value::Integer(3)
        ]).unwrap());
    }

    #[test]
    fn test_dequeue() {
        assert_eq!(Value::Integer(5), (DEQUEUE.handler)(&DEQUEUE, &[
            Value::Array(vec![
                Value::Integer(5), Value::Integer(3), 
            ])
        ]).unwrap());
    }

    #[test]
    fn test_enqueue() {
        assert_eq!(Value::Array(vec![
            Value::Integer(5), Value::Integer(3), 
        ]), (ENQUEUE.handler)(&ENQUEUE, &[
            Value::Array(vec![
                Value::Integer(5), 
            ]), Value::Integer(3)
        ]).unwrap());
    }

    #[test]
    fn test_remove() {
        assert_eq!(Value::Array(vec![Value::Integer(3)]), (REMOVE.handler)(&REMOVE, &[
            Value::Array(vec![
                Value::Integer(5), Value::Integer(3), 
            ]),
            Value::Integer(0)
        ]).unwrap());
        assert_eq!(Value::Array(vec![Value::Integer(5)]), (REMOVE.handler)(&REMOVE, &[
            Value::Array(vec![
                Value::Integer(5), Value::Integer(3), 
            ]),
            Value::Integer(1)
        ]).unwrap());
        assert_eq!(true, (REMOVE.handler)(&REMOVE, &[
            Value::Array(vec![
                Value::Integer(5), Value::Integer(3), 
            ]),
            Value::Integer(2)
        ]).is_err());
    }

    #[test]
    fn test_element() {
        assert_eq!(Value::Boolean(false), (IS_EMPTY.handler)(&IS_EMPTY, &[
            Value::Array(vec![
                Value::Integer(5), 
            ])
        ]).unwrap());
    }

    #[test]
    fn test_merge() {
        assert_eq!(Value::Boolean(false), (IS_EMPTY.handler)(&IS_EMPTY, &[
            Value::Array(vec![
                Value::Integer(5), 
            ])
        ]).unwrap());
    }
}