use crate::value::{AtomicValue, FloatType};
use crate::errors::*;

pub fn builtin_to_radians(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("to_radians(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType) * (std::f64::consts::PI / 180.0))),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n * (std::f64::consts::PI / 180.0))),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("to_radians(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

pub fn builtin_tan(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("tan(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).tan())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.tan())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("tan(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

pub fn builtin_cos(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("cos(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).cos())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.cos())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("cos(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

pub fn builtin_sin(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("sin(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).sin())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.sin())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("sin(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

pub fn builtin_atan(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("atan(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).atan())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.atan())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("atan(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

pub fn builtin_acos(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("acos(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).acos())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.acos())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("acos(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

pub fn builtin_asin(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("asin(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).asin())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.asin())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("asin(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

pub fn builtin_tanh(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("tanh(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).tanh())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.tanh())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("tanh(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

pub fn builtin_cosh(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("cosh(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).cosh())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.cosh())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("cosh(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

pub fn builtin_sinh(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("sinh(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).sinh())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.sinh())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("sinh(n)", 1, ExpectedTypes::IntOrFloat)))
    }
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