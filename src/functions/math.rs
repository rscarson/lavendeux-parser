use super::{FunctionDefinition, FunctionArgument, FunctionTable};
use crate::value::{Value, IntegerType};
use crate::errors::*;

const BOOL : FunctionDefinition = FunctionDefinition {
    name: "bool",
    description: "Returns a value as a boolean",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::Any),
    ],
    handler: |_, args: &[Value]| {
        Ok(Value::Boolean(args[0].as_bool()))
    }
};

const ARRAY : FunctionDefinition = FunctionDefinition {
    name: "array",
    description: "Returns a value as an array",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::Any),
    ],
    handler: |_, args: &[Value]| {
        Ok(Value::Array(args[0].as_array()))
    }
};

const INT : FunctionDefinition = FunctionDefinition {
    name: "int",
    description: "Returns a value as an integer",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
    ],
    handler: |_, args: &[Value]| {
        Ok(Value::Integer(args[0].as_int().unwrap()))
    }
};

const FLOAT : FunctionDefinition = FunctionDefinition {
    name: "float",
    description: "Returns a value as a float",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
    ],
    handler: |_, args: &[Value]| {
        Ok(Value::Float(args[0].as_float().unwrap()))
    }
};

const MIN : FunctionDefinition = FunctionDefinition {
    name: "min",
    description: "Returns the smallest numeric value from the supplied arguments",
    arguments: || vec![
        FunctionArgument::new_plural("n", ExpectedTypes::IntOrFloat, false),
    ],
    handler: |_, args: &[Value]| {
        let mut valid_args = args.iter().filter(|a|!a.as_float().unwrap().is_nan()).cloned().collect::<Vec<Value>>();
        valid_args.sort_by(|a,b| a.as_float().unwrap().partial_cmp(&b.as_float().unwrap()).unwrap());
        if valid_args.is_empty() {
            Ok(args[0].clone())
        } else {
            Ok(valid_args[0].clone())
        }
    }
};

const MAX : FunctionDefinition = FunctionDefinition {
    name: "max",
    description: "Returns the largest numeric value from the supplied arguments",
    arguments: || vec![
        FunctionArgument::new_plural("n", ExpectedTypes::IntOrFloat, false),
    ],
    handler: |_, args: &[Value]| {
        let mut valid_args = args.iter().filter(|a|!a.as_float().unwrap().is_nan()).cloned().collect::<Vec<Value>>();
        valid_args.sort_by(|a,b| b.as_float().unwrap().partial_cmp(&a.as_float().unwrap()).unwrap());
        if valid_args.is_empty() {
            Ok(args[0].clone())
        } else {
            Ok(valid_args[0].clone())
        }
    }
};

const CEIL : FunctionDefinition = FunctionDefinition {
    name: "ceil",
    description: "Returns the nearest whole integer larger than n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
    ],
    handler: |_, args: &[Value]| {
        Ok(Value::Integer(args[0].as_float().unwrap().ceil() as IntegerType))
    }
};

const FLOOR : FunctionDefinition = FunctionDefinition {
    name: "floor",
    description: "Returns the nearest whole integer smaller than n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
    ],
    handler: |_, args: &[Value]| {
        Ok(Value::Integer(args[0].as_float().unwrap().floor() as IntegerType))
    }
};

const ROUND : FunctionDefinition = FunctionDefinition {
    name: "round",
    description: "Returns n, rounded to [precision] decimal places",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
        FunctionArgument::new_optional("precision", ExpectedTypes::Int),
    ],
    handler: |_, args: &[Value]| {
        let precision = if args.len()== 1 {0} else {args[1].as_int().unwrap()};    
        if precision > u32::MAX as IntegerType { 
            return Err(ParserError::FunctionArgOverFlow(FunctionArgOverFlowError::new("round(n, precision=0)", 2))); 
        }
    
        let multiplier = f64::powi(10.0, precision as i32);
        let n = args[0].as_float().unwrap();
        Ok(Value::Float((n * multiplier).round() / multiplier))
    }
};

const ABS : FunctionDefinition = FunctionDefinition {
    name: "abs",
    description: "Returns the absolute value of n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat)
    ],
    handler: |_, args: &[Value]| {
        if args[0].is_int() {
            Ok(Value::Integer(args[0].as_int().unwrap().abs()))
        } else {
            Ok(Value::Float(args[0].as_float().unwrap().abs()))
        }
    }
};

const LOG10 : FunctionDefinition = FunctionDefinition {
    name: "log10",
    description: "Returns the base 10 log of n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
    ],
    handler: |_, args: &[Value]| {
        Ok(Value::Float(args[0].as_float().unwrap().log10()))
    }
};

const LN : FunctionDefinition = FunctionDefinition {
    name: "ln",
    description: "Returns the natural log of n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
    ],
    handler: |_, args: &[Value]| {
        Ok(Value::Float(args[0].as_float().unwrap().ln()))
    }
};

const LOG : FunctionDefinition = FunctionDefinition {
    name: "log",
    description: "Returns the logarithm of n in any base",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
        FunctionArgument::new_required("base", ExpectedTypes::IntOrFloat),
    ],
    handler: |_, args: &[Value]| {
        let base = args[1].as_float().unwrap();
        Ok(Value::Float(args[0].as_float().unwrap().log(base)))
    }
};

const SQRT : FunctionDefinition = FunctionDefinition {
    name: "sqrt",
    description: "Returns the square root of n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
    ],
    handler: |_, args: &[Value]| {
        Ok(Value::Float(args[0].as_float().unwrap().sqrt()))
    }
};

const ROOT : FunctionDefinition = FunctionDefinition {
    name: "root",
    description: "Returns a root of n of any base",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
        FunctionArgument::new_required("base", ExpectedTypes::IntOrFloat),
    ],
    handler: |_, args: &[Value]| {
        let base = args[1].as_float().unwrap();
        Ok(Value::Float(args[0].as_float().unwrap().powf(1.0 / base)))
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
        assert_eq!(Value::Integer(3), (MIN.handler)(&MIN, &[
            Value::Float(3.5),
            Value::Integer(3),
            Value::Integer(7),
            Value::Float(FloatType::NAN)
        ]).unwrap());
        assert_eq!(Value::Float(3.1), (MIN.handler)(&MIN, &[
            Value::Float(3.5),
            Value::Float(3.1),
            Value::Integer(7),
            Value::Float(FloatType::NAN)
        ]).unwrap());
    }
    
    #[test]
    fn test_max() {
        assert_eq!(Value::Integer(7), (MAX.handler)(&MAX, &[
            Value::Float(3.5),
            Value::Integer(3),
            Value::Integer(7),
            Value::Float(FloatType::NAN)
        ]).unwrap());
        assert_eq!(Value::Float(7.1), (MAX.handler)(&MAX, &[
            Value::Float(3.5),
            Value::Integer(3),
            Value::Float(7.1),
            Value::Float(FloatType::NAN)
        ]).unwrap());
    }
    
    #[test]
    fn test_ceil() {
        assert_eq!(Value::Integer(4), (CEIL.handler)(&CEIL, &[Value::Float(3.5)]).unwrap());
        assert_eq!(Value::Integer(4), (CEIL.handler)(&CEIL, &[Value::Integer(4)]).unwrap());
    }
    
    #[test]
    fn test_floor() {
        assert_eq!(Value::Integer(3), (FLOOR.handler)(&FLOOR, &[Value::Float(3.5)]).unwrap());
        assert_eq!(Value::Integer(4), (FLOOR.handler)(&FLOOR, &[Value::Integer(4)]).unwrap());
    }
    
    #[test]
    fn test_round() {
        assert_eq!(Value::Float(3.56), (ROUND.handler)(&ROUND, &[Value::Float(3.555), Value::Integer(2)]).unwrap());
        assert_eq!(Value::Float(4.0), (ROUND.handler)(&ROUND, &[Value::Integer(4), Value::Integer(2)]).unwrap());
    }
    
    #[test]
    fn test_abs() {
        assert_eq!(Value::Integer(3), (ABS.handler)(&ABS, &[Value::Integer(3)]).unwrap());
        assert_eq!(Value::Integer(3), (ABS.handler)(&ABS, &[Value::Integer(-3)]).unwrap());
        assert_eq!(Value::Float(4.0), (ABS.handler)(&ABS, &[Value::Float(-4.0)]).unwrap());
    }
    
    #[test]
    fn test_ln() {
        assert_eq!(Value::Float(1.0), (LN.handler)(&LN, &[Value::Float(std::f64::consts::E)]).unwrap());
    }
    
    #[test]
    fn test_log10() {
        assert_eq!(Value::Float(2.0), (LOG10.handler)(&LOG10, &[Value::Float(100.0)]).unwrap());
    }
    
    #[test]
    fn test_log() {
        assert_eq!(Value::Float(2.0), (LOG.handler)(&LOG, &[Value::Float(100.0), Value::Integer(10)]).unwrap());
    }
    
    #[test]
    fn test_sqrt() {
        assert_eq!(Value::Float(3.0), (SQRT.handler)(&SQRT, &[Value::Float(9.0)]).unwrap());
    }
    
    #[test]
    fn test_root() {
        assert_eq!(Value::Float(3.0), (ROOT.handler)(&ROOT, &[Value::Float(27.0), Value::Integer(3)]).unwrap());
    }
}