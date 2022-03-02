use super::value::{AtomicValue, IntegerType, FloatType};
use super::errors::*;
use std::collections::HashMap;

const MAX_FLOAT_PRECISION: i32 = 8;

pub type DecoratorHandler = fn(&AtomicValue) -> Result<String, ParserError>;
pub struct DecoratorTable(HashMap<String, DecoratorHandler>);
impl DecoratorTable {
    /// Initialize a new decorator table, complete with default builtin decorators
    pub fn new() -> DecoratorTable {
        let mut table : DecoratorTable = DecoratorTable{0: HashMap::new()};

        table.0.insert("default".to_string(), decorator_default);
        table.0.insert("hex".to_string(), decorator_hex);
        table.0.insert("bin".to_string(), decorator_bin);
        table.0.insert("oct".to_string(), decorator_oct);
        
        table.0.insert("sci".to_string(), decorator_sci);
        table.0.insert("float".to_string(), decorator_float);
        table.0.insert("int".to_string(), decorator_int);

        table
    }

    /// Register a decorator in the table
    /// 
    /// # Arguments
    /// * `name` - Decorator name
    /// * `handler` - Decorator handler
    pub fn register(&mut self, name: &str, handler: DecoratorHandler) {
        self.0.insert(name.to_string(), handler);
    }

    /// Check if the table contains a decorator by the given name
    /// 
    /// # Arguments
    /// * `name` - Decorator name
    pub fn has(&self, name: &str) -> bool {
        return self.0.contains_key(name)
    }

    /// Call a decorator
    /// 
    /// # Arguments
    /// * `name` - Decorator name
    /// * `args` - Decorator arguments
    pub fn call(&self, name: &str, arg: &AtomicValue) -> Result<String, ParserError> {
        match self.0.get(name) {
            Some(f) => match f(&arg) {
                Ok(v) => Ok(v),
                Err(e) => Err(e)
            },
            None => Err(ParserError::DecoratorName(DecoratorNameError::new(name)))
        }
    }
}

pub fn decorator_default(input: &AtomicValue) -> Result<String, ParserError> {
    match input {
        AtomicValue::Integer(_) => decorator_int(input),
        AtomicValue::Float(_) => decorator_float(input),
        AtomicValue::String(s) => Ok(format!("{}", s)),
        AtomicValue::None => Ok("".to_string())
    }
}

fn decorator_hex(input: &AtomicValue) -> Result<String, ParserError> {
    match input {
        AtomicValue::Integer(n) => Ok(format!("{:#x}", *n)),
        AtomicValue::Float(n) => Ok(format!("{:#x}", *n as IntegerType)),
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("@hex", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn decorator_bin(input: &AtomicValue) -> Result<String, ParserError> {
    match input {
        AtomicValue::Integer(n) => Ok(format!("{:#0b}", *n)),
        AtomicValue::Float(n) => Ok(format!("{:#0b}", *n as IntegerType)),
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("@bin", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn decorator_oct(input: &AtomicValue) -> Result<String, ParserError> {
    match input {
        AtomicValue::Integer(n) => Ok(format!("{:#0o}", *n)),
        AtomicValue::Float(n) => Ok(format!("{:#0o}", *n as IntegerType)),
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("@oct", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn decorator_sci(input: &AtomicValue) -> Result<String, ParserError> {
    match input {
        AtomicValue::Integer(n) => Ok(format!("{:e}", *n)),
        AtomicValue::Float(n) => Ok(format!("{:e}", *n)),
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("@sci", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn decorator_float(input: &AtomicValue) -> Result<String, ParserError> {
    let multiplier = f64::powi(10.0, MAX_FLOAT_PRECISION);

    match input {
        AtomicValue::Integer(n) => Ok(format!("{}", *n as FloatType)),
        AtomicValue::Float(n) => {
            let mut v = (*n * multiplier).round() / multiplier;
            if v == -0.0 {
                v = 0.0;
            }
            Ok(format!("{}", v))
        },
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("@float", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn decorator_int(input: &AtomicValue) -> Result<String, ParserError> {
    match input {
        AtomicValue::Integer(n) => Ok(format!("{}", *n)),
        AtomicValue::Float(n) => Ok(format!("{}", *n as IntegerType)),
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("@int", 1, ExpectedTypes::IntOrFloat)))
    }
}

#[cfg(test)]
mod test_builtin_functions {
    use super::*;
    
    #[test]
    fn test_default() {
    }

    #[test]
    fn test_hex() {
        assert_eq!("0xff", decorator_hex(&AtomicValue::Integer(255)).unwrap());
        assert_eq!("0xff", decorator_hex(&AtomicValue::Float(255.1)).unwrap());
    }

    #[test]
    fn test_bin() {
        assert_eq!("0b11111111", decorator_bin(&AtomicValue::Integer(255)).unwrap());
        assert_eq!("0b11111111", decorator_bin(&AtomicValue::Float(255.1)).unwrap());
    }

    #[test]
    fn test_oct() {
        assert_eq!("0o10", decorator_oct(&AtomicValue::Integer(8)).unwrap());
        assert_eq!("0o10", decorator_oct(&AtomicValue::Float(8.1)).unwrap());
    }

    #[test]
    fn test_sci() {
        assert_eq!("8e0", decorator_sci(&AtomicValue::Integer(8)).unwrap());
        assert_eq!("-8.1e1", decorator_sci(&AtomicValue::Float(-81.0)).unwrap());
        assert_eq!("8.1e-2", decorator_sci(&AtomicValue::Float(0.081)).unwrap());
    }

    #[test]
    fn test_float() {
        assert_eq!("8", decorator_float(&AtomicValue::Integer(8)).unwrap());
        assert_eq!("81", decorator_float(&AtomicValue::Float(81.0)).unwrap());
        assert_eq!("0.081", decorator_float(&AtomicValue::Float(0.081)).unwrap());
    }

    #[test]
    fn test_int() {
        assert_eq!("-8", decorator_int(&AtomicValue::Integer(-8)).unwrap());
        assert_eq!("81", decorator_int(&AtomicValue::Float(81.0)).unwrap());
        assert_eq!("0", decorator_int(&AtomicValue::Float(0.081)).unwrap());
    }
}