//! Builtin functions for advanced mathematics

use super::*;
use crate::value::{Value, IntegerType};

const BOOL : FunctionDefinition = FunctionDefinition {
    name: "bool",
    category: Some("math"),
    description: "Returns a value as a boolean",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::Any),
    ],
    handler: |_function, _state, args| {
        Ok(Value::Boolean(args.get("n").required().as_bool()))
    }
};

const ARRAY : FunctionDefinition = FunctionDefinition {
    name: "array",
    category: Some("math"),
    description: "Returns a value as an array",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::Any),
    ],
    handler: |_function, _state, args| {
        Ok(Value::Array(args.get("n").required().as_array()))
    }
};

const INT : FunctionDefinition = FunctionDefinition {
    name: "int",
    category: Some("math"),
    description: "Returns a value as an integer",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
    ],
    handler: |_function, _state, args| {
        Ok(Value::Integer(args.get("n").required().as_int().unwrap()))
    }
};

const FLOAT : FunctionDefinition = FunctionDefinition {
    name: "float",
    category: Some("math"),
    description: "Returns a value as a float",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
    ],
    handler: |_function, _state, args| {
        Ok(Value::Float(args.get("n").required().as_float().unwrap()))
    }
};

const MIN : FunctionDefinition = FunctionDefinition {
    name: "min",
    category: Some("math"),
    description: "Returns the smallest numeric value from the supplied arguments",
    arguments: || vec![
        FunctionArgument::new_plural("n", ExpectedTypes::IntOrFloat, false),
    ],
    handler: |_function, _state, args| {
        let mut valid_args = args.iter().filter(|a|!a.as_float().unwrap().is_nan()).cloned().collect::<Vec<Value>>();
        valid_args.sort_by(|a,b| a.as_float().unwrap().partial_cmp(&b.as_float().unwrap()).unwrap());
        if valid_args.is_empty() {
            Ok(args.get("n").plural().first().cloned().unwrap())
        } else {
            Ok(valid_args[0].clone())
        }
    }
};

const MAX : FunctionDefinition = FunctionDefinition {
    name: "max",
    category: Some("math"),
    description: "Returns the largest numeric value from the supplied arguments",
    arguments: || vec![
        FunctionArgument::new_plural("n", ExpectedTypes::IntOrFloat, false),
    ],
    handler: |_function, _state, args| {
        let mut valid_args = args.iter().filter(|a|!a.as_float().unwrap().is_nan()).cloned().collect::<Vec<Value>>();
        valid_args.sort_by(|a,b| b.as_float().unwrap().partial_cmp(&a.as_float().unwrap()).unwrap());
        if valid_args.is_empty() {
            Ok(args.get("n").plural().first().cloned().unwrap())
        } else {
            Ok(valid_args[0].clone())
        }
    }
};

const CEIL : FunctionDefinition = FunctionDefinition {
    name: "ceil",
    category: Some("math"),
    description: "Returns the nearest whole integer larger than n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
    ],
    handler: |_function, _state, args| {
        Ok(Value::Integer(args.get("n").required().as_float().unwrap().ceil() as IntegerType))
    }
};

const FLOOR : FunctionDefinition = FunctionDefinition {
    name: "floor",
    category: Some("math"),
    description: "Returns the nearest whole integer smaller than n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
    ],
    handler: |_function, _state, args| {
        Ok(Value::Integer(args.get("n").required().as_float().unwrap().floor() as IntegerType))
    }
};

const ROUND : FunctionDefinition = FunctionDefinition {
    name: "round",
    category: Some("math"),
    description: "Returns n, rounded to [precision] decimal places",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
        FunctionArgument::new_optional("precision", ExpectedTypes::Int),
    ],
    handler: |_function, _state, args| {
        let precision = args.get("precision").optional_or(Value::Integer(0)).as_int().unwrap_or(0);
        if precision > u32::MAX as IntegerType { 
            return Err(ParserError::FunctionArgOverFlow(FunctionArgOverFlowError::new("round(n, precision=0)", 2))); 
        }
    
        let multiplier = f64::powi(10.0, precision as i32);
        let n = args.get("n").required().as_float().unwrap();
        Ok(Value::Float((n * multiplier).round() / multiplier))
    }
};

const ABS : FunctionDefinition = FunctionDefinition {
    name: "abs",
    category: Some("math"),
    description: "Returns the absolute value of n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat)
    ],
    handler: |_function, _state, args| {
        let n = args.get("n").required();
        if n.is_int() {
            Ok(Value::Integer(n.as_int().unwrap().abs()))
        } else {
            Ok(Value::Float(n.as_float().unwrap().abs()))
        }
    }
};

const LOG10 : FunctionDefinition = FunctionDefinition {
    name: "log10",
    category: Some("math"),
    description: "Returns the base 10 log of n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
    ],
    handler: |_function, _state, args| {
        Ok(Value::Float(args.get("n").required().as_float().unwrap().log10()))
    }
};

const LN : FunctionDefinition = FunctionDefinition {
    name: "ln",
    category: Some("math"),
    description: "Returns the natural log of n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
    ],
    handler: |_function, _state, args| {
        Ok(Value::Float(args.get("n").required().as_float().unwrap().ln()))
    }
};

const LOG : FunctionDefinition = FunctionDefinition {
    name: "log",
    category: Some("math"),
    description: "Returns the logarithm of n in any base",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
        FunctionArgument::new_required("base", ExpectedTypes::IntOrFloat),
    ],
    handler: |_function, _state, args| {
        let base = args.get("base").required().as_float().unwrap();
        Ok(Value::Float(args.get("n").required().as_float().unwrap().log(base)))
    }
};

const SQRT : FunctionDefinition = FunctionDefinition {
    name: "sqrt",
    category: Some("math"),
    description: "Returns the square root of n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
    ],
    handler: |_function, _state, args| {
        Ok(Value::Float(args.get("n").required().as_float().unwrap().sqrt()))
    }
};

const ROOT : FunctionDefinition = FunctionDefinition {
    name: "root",
    category: Some("math"),
    description: "Returns a root of n of any base",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
        FunctionArgument::new_required("base", ExpectedTypes::IntOrFloat),
    ],
    handler: |_function, _state, args| {
        let base = args.get("base").required().as_float().unwrap();
        Ok(Value::Float(args.get("n").required().as_float().unwrap().powf(1.0 / base)))
    }
};

/// Register string functions
pub fn register_functions(table: &mut FunctionTable) {
    // Typecasting
    table.register(BOOL);
    table.register(ARRAY);
    table.register(INT);
    table.register(FLOAT);

    // Rounding functions
    table.register(MIN);
    table.register(MAX);
    table.register(CEIL);
    table.register(FLOOR);
    table.register(ROUND);
    table.register(ABS);
    
    // Roots and logs
    table.register(LOG10);
    table.register(LN);
    table.register(LOG);
    table.register(SQRT);
    table.register(ROOT);
    
}

#[cfg(test)]
mod test_builtin_functions {
    use crate::value::{FloatType};
    use super::*;
    
    #[test]
    fn test_min() {
        let mut state = ParserState::new();

        assert_eq!(Value::Integer(3), MIN.call(&mut state, &[
            Value::Float(3.5),
            Value::Integer(3),
            Value::Integer(7),
            Value::Float(FloatType::NAN)
        ]).unwrap());
        assert_eq!(Value::Float(3.1), MIN.call(&mut state, &[
            Value::Float(3.5),
            Value::Float(3.1),
            Value::Integer(7),
            Value::Float(FloatType::NAN)
        ]).unwrap());
    }
    
    #[test]
    fn test_max() {
        let mut state = ParserState::new();

        assert_eq!(Value::Integer(7), MAX.call(&mut state, &[
            Value::Float(3.5),
            Value::Integer(3),
            Value::Integer(7),
            Value::Float(FloatType::NAN)
        ]).unwrap());
        assert_eq!(Value::Float(7.1), MAX.call(&mut state, &[
            Value::Float(3.5),
            Value::Integer(3),
            Value::Float(7.1),
            Value::Float(FloatType::NAN)
        ]).unwrap());
    }
    
    #[test]
    fn test_ceil() {
        let mut state = ParserState::new();

        assert_eq!(Value::Integer(4), CEIL.call(&mut state, &[Value::Float(3.5)]).unwrap());
        assert_eq!(Value::Integer(4), CEIL.call(&mut state, &[Value::Integer(4)]).unwrap());
    }
    
    #[test]
    fn test_floor() {
        let mut state = ParserState::new();

        assert_eq!(Value::Integer(3), FLOOR.call(&mut state, &[Value::Float(3.5)]).unwrap());
        assert_eq!(Value::Integer(4), FLOOR.call(&mut state, &[Value::Integer(4)]).unwrap());
    }
    
    #[test]
    fn test_round() {
        let mut state = ParserState::new();

        assert_eq!(Value::Float(3.56), ROUND.call(&mut state, &[Value::Float(3.555), Value::Integer(2)]).unwrap());
        assert_eq!(Value::Float(4.0), ROUND.call(&mut state, &[Value::Integer(4), Value::Integer(2)]).unwrap());
    }
    
    #[test]
    fn test_abs() {
        let mut state = ParserState::new();

        assert_eq!(Value::Integer(3), ABS.call(&mut state, &[Value::Integer(3)]).unwrap());
        assert_eq!(Value::Integer(3), ABS.call(&mut state, &[Value::Integer(-3)]).unwrap());
        assert_eq!(Value::Float(4.0), ABS.call(&mut state, &[Value::Float(-4.0)]).unwrap());
    }
    
    #[test]
    fn test_ln() {
        let mut state = ParserState::new();

        assert_eq!(Value::Float(1.0), LN.call(&mut state, &[Value::Float(std::f64::consts::E)]).unwrap());
    }
    
    #[test]
    fn test_log10() {
        let mut state = ParserState::new();

        assert_eq!(Value::Float(2.0), LOG10.call(&mut state, &[Value::Float(100.0)]).unwrap());
    }
    
    #[test]
    fn test_log() {
        let mut state = ParserState::new();

        assert_eq!(Value::Float(2.0), LOG.call(&mut state, &[Value::Float(100.0), Value::Integer(10)]).unwrap());
    }
    
    #[test]
    fn test_sqrt() {
        let mut state = ParserState::new();

        assert_eq!(Value::Float(3.0), SQRT.call(&mut state, &[Value::Float(9.0)]).unwrap());
    }
    
    #[test]
    fn test_root() {
        let mut state = ParserState::new();

        assert_eq!(Value::Float(3.0), ROOT.call(&mut state, &[Value::Float(27.0), Value::Integer(3)]).unwrap());
    }
}