use crate::value::{AtomicValue, FloatType};
use crate::errors::*;

pub fn builtin_to_radians(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("to_radians(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType) * (std::f64::consts::PI / 180.0))),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n * (std::f64::consts::PI / 180.0))),
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("to_radians(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_trig(sig: &str, method: fn(FloatType) -> FloatType, args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new(sig, 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float(method(n as FloatType))),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(method(n))),
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new(sig, 1, ExpectedTypes::IntOrFloat)))
    }
}

pub fn builtin_tan(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    builtin_trig("tan(n)", FloatType::tan, args)
}

pub fn builtin_cos(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    builtin_trig("cos(n)", FloatType::cos, args)
}

pub fn builtin_sin(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    builtin_trig("sin(n)", FloatType::sin, args)
}

pub fn builtin_atan(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    builtin_trig("atan(n)", FloatType::atan, args)
}

pub fn builtin_acos(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    builtin_trig("acos(n)", FloatType::acos, args)
}

pub fn builtin_asin(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    builtin_trig("asin(n)", FloatType::asin, args)
}

pub fn builtin_tanh(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    builtin_trig("tanh(n)", FloatType::tanh, args)
}

pub fn builtin_cosh(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    builtin_trig("cosh(n)", FloatType::cosh, args)
}

pub fn builtin_sinh(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    builtin_trig("sinh(n)", FloatType::sinh, args)
}

#[cfg(test)]
mod test_builtin_functions {
    use super::*;
        
    #[test]
    fn test_to_radians() {
        assert_eq!(AtomicValue::Float(std::f64::consts::PI), builtin_to_radians(&[AtomicValue::Integer(180)]).unwrap());
        assert_eq!(AtomicValue::Float(std::f64::consts::PI * 4.0), builtin_to_radians(&[AtomicValue::Integer(720)]).unwrap());
    }
    
    #[test]
    fn test_tan() {
        assert_eq!(AtomicValue::Float(0.0), builtin_tan(&[AtomicValue::Float(0.0)]).unwrap());
        assert_eq!(true, 0.00001 > 1.0 - builtin_tan(&[AtomicValue::Float(std::f64::consts::PI / 4.0)]).unwrap().as_float().unwrap());
    }
    
    #[test]
    fn test_cos() {
        assert_eq!(AtomicValue::Float(1.0), builtin_cos(&[AtomicValue::Float(0.0)]).unwrap());
        assert_eq!(true, 0.00001 > builtin_cos(&[AtomicValue::Float(std::f64::consts::PI / 2.0)]).unwrap().as_float().unwrap());
    }
    
    #[test]
    fn test_sin() {
        assert_eq!(AtomicValue::Float(0.0), builtin_sin(&[AtomicValue::Float(0.0)]).unwrap());
        assert_eq!(AtomicValue::Float(1.0), builtin_sin(&[AtomicValue::Float(std::f64::consts::PI / 2.0)]).unwrap());
    }
    
    #[test]
    fn test_atan() {
        assert_eq!(AtomicValue::Float(0.0), builtin_atan(&[AtomicValue::Float(0.0)]).unwrap());
    }
    
    #[test]
    fn test_acos() {
        assert_eq!(AtomicValue::Float(0.0), builtin_acos(&[AtomicValue::Float(1.0)]).unwrap());
        assert_eq!(AtomicValue::Float(std::f64::consts::PI), builtin_acos(&[AtomicValue::Float(-1.0)]).unwrap());
    }
    
    #[test]
    fn test_asin() {
        assert_eq!(AtomicValue::Float(0.0), builtin_asin(&[AtomicValue::Float(0.0)]).unwrap());
    }
    
    #[test]
    fn test_tanh() {
        assert_eq!(AtomicValue::Float(0.0), builtin_tanh(&[AtomicValue::Float(0.0)]).unwrap());
    }
    
    #[test]
    fn test_cosh() {
        assert_eq!(AtomicValue::Float(1.0), builtin_cosh(&[AtomicValue::Float(0.0)]).unwrap());
    }
    
    #[test]
    fn test_sinh() {
        assert_eq!(AtomicValue::Float(0.0), builtin_sinh(&[AtomicValue::Float(0.0)]).unwrap());
    }
}