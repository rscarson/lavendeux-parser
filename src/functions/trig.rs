use crate::value::{Value, FloatType};
use crate::errors::*;

pub fn builtin_to_radians(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("to_radians(n)", 1, 1)));
    }

    if let Some(n) = args[0].as_float() {
        Ok(Value::Float(n * (std::f64::consts::PI / 180.0)))
    } else {
        Err(ParserError::FunctionArgType(FunctionArgTypeError::new("to_radians(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

pub fn builtin_to_degrees(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("to_degrees(n)", 1, 1)));
    }

    if let Some(n) = args[0].as_float() {
        Ok(Value::Float(n * 180.0 / std::f64::consts::PI))
    } else {
        Err(ParserError::FunctionArgType(FunctionArgTypeError::new("to_degrees(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_trig(sig: &str, method: fn(FloatType) -> FloatType, args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new(sig, 1, 1)));
    }

    if let Some(n) = args[0].as_float() {
        Ok(Value::Float(method(n)))
    } else {
        Err(ParserError::FunctionArgType(FunctionArgTypeError::new(sig, 1, ExpectedTypes::IntOrFloat)))
    }
}

pub fn builtin_tan(args: &[Value]) -> Result<Value, ParserError> {
    builtin_trig("tan(n)", FloatType::tan, args)
}

pub fn builtin_cos(args: &[Value]) -> Result<Value, ParserError> {
    builtin_trig("cos(n)", FloatType::cos, args)
}

pub fn builtin_sin(args: &[Value]) -> Result<Value, ParserError> {
    builtin_trig("sin(n)", FloatType::sin, args)
}

pub fn builtin_atan(args: &[Value]) -> Result<Value, ParserError> {
    builtin_trig("atan(n)", FloatType::atan, args)
}

pub fn builtin_acos(args: &[Value]) -> Result<Value, ParserError> {
    builtin_trig("acos(n)", FloatType::acos, args)
}

pub fn builtin_asin(args: &[Value]) -> Result<Value, ParserError> {
    builtin_trig("asin(n)", FloatType::asin, args)
}

pub fn builtin_tanh(args: &[Value]) -> Result<Value, ParserError> {
    builtin_trig("tanh(n)", FloatType::tanh, args)
}

pub fn builtin_cosh(args: &[Value]) -> Result<Value, ParserError> {
    builtin_trig("cosh(n)", FloatType::cosh, args)
}

pub fn builtin_sinh(args: &[Value]) -> Result<Value, ParserError> {
    builtin_trig("sinh(n)", FloatType::sinh, args)
}

#[cfg(test)]
mod test_builtin_functions {
    use super::*;
        
    #[test]
    fn test_to_radians() {
        assert_eq!(Value::Float(std::f64::consts::PI), builtin_to_radians(&[Value::Integer(180)]).unwrap());
        assert_eq!(Value::Float(std::f64::consts::PI * 4.0), builtin_to_radians(&[Value::Integer(720)]).unwrap());
    }
        
    #[test]
    fn test_to_degrees() {
        assert_eq!(Value::Float(180.0), builtin_to_degrees(&[Value::Float(std::f64::consts::PI)]).unwrap());
        assert_eq!(Value::Float(90.0), builtin_to_degrees(&[Value::Float(std::f64::consts::PI / 2.0)]).unwrap());
    }
    
    #[test]
    fn test_tan() {
        assert_eq!(Value::Float(0.0), builtin_tan(&[Value::Float(0.0)]).unwrap());
        assert_eq!(true, 0.00001 > 1.0 - builtin_tan(&[Value::Float(std::f64::consts::PI / 4.0)]).unwrap().as_float().unwrap());
    }
    
    #[test]
    fn test_cos() {
        assert_eq!(Value::Float(1.0), builtin_cos(&[Value::Float(0.0)]).unwrap());
        assert_eq!(true, 0.00001 > builtin_cos(&[Value::Float(std::f64::consts::PI / 2.0)]).unwrap().as_float().unwrap());
    }
    
    #[test]
    fn test_sin() {
        assert_eq!(Value::Float(0.0), builtin_sin(&[Value::Float(0.0)]).unwrap());
        assert_eq!(Value::Float(1.0), builtin_sin(&[Value::Float(std::f64::consts::PI / 2.0)]).unwrap());
    }
    
    #[test]
    fn test_atan() {
        assert_eq!(Value::Float(0.0), builtin_atan(&[Value::Float(0.0)]).unwrap());
    }
    
    #[test]
    fn test_acos() {
        assert_eq!(Value::Float(0.0), builtin_acos(&[Value::Float(1.0)]).unwrap());
        assert_eq!(Value::Float(std::f64::consts::PI), builtin_acos(&[Value::Float(-1.0)]).unwrap());
    }
    
    #[test]
    fn test_asin() {
        assert_eq!(Value::Float(0.0), builtin_asin(&[Value::Float(0.0)]).unwrap());
    }
    
    #[test]
    fn test_tanh() {
        assert_eq!(Value::Float(0.0), builtin_tanh(&[Value::Float(0.0)]).unwrap());
    }
    
    #[test]
    fn test_cosh() {
        assert_eq!(Value::Float(1.0), builtin_cosh(&[Value::Float(0.0)]).unwrap());
    }
    
    #[test]
    fn test_sinh() {
        assert_eq!(Value::Float(0.0), builtin_sinh(&[Value::Float(0.0)]).unwrap());
    }
}