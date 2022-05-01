use super::{FunctionDefinition, FunctionArgument, FunctionTable};
use crate::value::{Value, FloatType};
use crate::errors::*;

fn builtin_trig(method: fn(FloatType) -> FloatType, args: &[Value]) -> Result<Value, ParserError> {
    let n = args[0].as_float().unwrap();
    Ok(Value::Float(method(n)))
}

const TO_RADIANS : FunctionDefinition = FunctionDefinition {
    name: "to_radians",
    description: "Convert the given degree value into radians",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat)
    ],
    handler: |_, args: &[Value]| {
        let n = args[0].as_float().unwrap();
        Ok(Value::Float(n * (std::f64::consts::PI / 180.0)))
    }
};

const TO_DEGREES : FunctionDefinition = FunctionDefinition {
    name: "to_degrees",
    description: "Convert the given radian value into degrees",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat)
    ],
    handler: |_, args: &[Value]| {
        let n = args[0].as_float().unwrap();
        Ok(Value::Float(n * 180.0 / std::f64::consts::PI))
    }
};

const TAN : FunctionDefinition = FunctionDefinition {
    name: "tan",
    description: "Calculate the tangent of n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat)
    ],
    handler: |_, args: &[Value]| builtin_trig(FloatType::tan, args)
};

const COS : FunctionDefinition = FunctionDefinition {
    name: "cos",
    description: "Calculate the cosine of n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat)
    ],
    handler: |_, args: &[Value]| builtin_trig(FloatType::cos, args)
};

const SIN : FunctionDefinition = FunctionDefinition {
    name: "sin",
    description: "Calculate the sine of n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat)
    ],
    handler: |_, args: &[Value]| builtin_trig(FloatType::sin, args)
};

const ATAN : FunctionDefinition = FunctionDefinition {
    name: "atan",
    description: "Calculate the arctangent of n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat)
    ],
    handler: |_, args: &[Value]| builtin_trig(FloatType::atan, args)
};

const ACOS : FunctionDefinition = FunctionDefinition {
    name: "acos",
    description: "Calculate the arccosine of n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat)
    ],
    handler: |_, args: &[Value]| builtin_trig(FloatType::acos, args)
};

const ASIN : FunctionDefinition = FunctionDefinition {
    name: "asin",
    description: "Calculate the arcsine of n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat)
    ],
    handler: |_, args: &[Value]| builtin_trig(FloatType::asin, args)
};

const TANH : FunctionDefinition = FunctionDefinition {
    name: "tanh",
    description: "Calculate the hyperbolic tangent of n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat)
    ],
    handler: |_, args: &[Value]| builtin_trig(FloatType::tanh, args)
};

const COSH : FunctionDefinition = FunctionDefinition {
    name: "cosh",
    description: "Calculate the hyperbolic cosine of n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat)
    ],
    handler: |_, args: &[Value]| builtin_trig(FloatType::cosh, args)
};

const SINH : FunctionDefinition = FunctionDefinition {
    name: "sinh",
    description: "Calculate the hyperbolic sine of n",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat)
    ],
    handler: |_, args: &[Value]| builtin_trig(FloatType::sinh, args)
};

/// Register trig functions
pub fn register_functions(table: &mut FunctionTable) {
    table.register(TO_RADIANS);
    table.register(TO_DEGREES);

    table.register(TAN);
    table.register(ATAN);
    table.register(TANH);
    
    table.register(COS);
    table.register(ACOS);
    table.register(COSH);
    
    table.register(SIN);
    table.register(ASIN);
    table.register(SINH);
}

#[cfg(test)]
mod test_builtin_functions {
    use super::*;
        
    #[test]
    fn test_to_radians() {
        assert_eq!(Value::Float(std::f64::consts::PI), (TO_RADIANS.handler)(&TO_RADIANS, &[Value::Integer(180)]).unwrap());
        assert_eq!(Value::Float(std::f64::consts::PI * 4.0), (TO_RADIANS.handler)(&TO_RADIANS, &[Value::Integer(720)]).unwrap());
    }
        
    #[test]
    fn test_to_degrees() {
        assert_eq!(Value::Float(180.0), (TO_DEGREES.handler)(&TO_DEGREES, &[Value::Float(std::f64::consts::PI)]).unwrap());
        assert_eq!(Value::Float(90.0), (TO_DEGREES.handler)(&TO_DEGREES, &[Value::Float(std::f64::consts::PI / 2.0)]).unwrap());
    }
    
    #[test]
    fn test_tan() {
        assert_eq!(Value::Float(0.0), (TAN.handler)(&TAN, &[Value::Float(0.0)]).unwrap());
        assert_eq!(true, 0.00001 > 1.0 - (TAN.handler)(&TAN, &[Value::Float(std::f64::consts::PI / 4.0)]).unwrap().as_float().unwrap());
    }
    
    #[test]
    fn test_cos() {
        assert_eq!(Value::Float(1.0), (COS.handler)(&COS, &[Value::Float(0.0)]).unwrap());
        assert_eq!(true, 0.00001 > (COS.handler)(&COS, &[Value::Float(std::f64::consts::PI / 2.0)]).unwrap().as_float().unwrap());
    }
    
    #[test]
    fn test_sin() {
        assert_eq!(Value::Float(0.0), (SIN.handler)(&SIN, &[Value::Float(0.0)]).unwrap());
        assert_eq!(Value::Float(1.0), (SIN.handler)(&SIN, &[Value::Float(std::f64::consts::PI / 2.0)]).unwrap());
    }
    
    #[test]
    fn test_atan() {
        assert_eq!(Value::Float(0.0), (ATAN.handler)(&ATAN, &[Value::Float(0.0)]).unwrap());
    }
    
    #[test]
    fn test_acos() {
        assert_eq!(Value::Float(0.0), (ACOS.handler)(&ACOS, &[Value::Float(1.0)]).unwrap());
        assert_eq!(Value::Float(std::f64::consts::PI), (ACOS.handler)(&ACOS, &[Value::Float(-1.0)]).unwrap());
    }
    
    #[test]
    fn test_asin() {
        assert_eq!(Value::Float(0.0), (ASIN.handler)(&ASIN, &[Value::Float(0.0)]).unwrap());
    }
    
    #[test]
    fn test_tanh() {
        assert_eq!(Value::Float(0.0), (TANH.handler)(&TANH, &[Value::Float(0.0)]).unwrap());
    }
    
    #[test]
    fn test_cosh() {
        assert_eq!(Value::Float(1.0), (COSH.handler)(&COSH, &[Value::Float(0.0)]).unwrap());
    }
    
    #[test]
    fn test_sinh() {
        assert_eq!(Value::Float(0.0), (SINH.handler)(&SINH, &[Value::Float(0.0)]).unwrap());
    }
}