use super::value::Value;
use super::errors::*;
use std::collections::HashMap;
use chrono::prelude::*;

pub type DecoratorHandler = fn(&Value) -> Result<String, ParserError>;

#[derive(Clone)]
pub struct DecoratorTable(HashMap<String, DecoratorHandler>);
impl DecoratorTable {
    /// Initialize a new decorator table, complete with default builtin decorators
    pub fn new() -> DecoratorTable {
        let mut table : DecoratorTable = DecoratorTable(HashMap::new());

        table.0.insert("default".to_string(), decorator_default);
        table.0.insert("hex".to_string(), decorator_hex);
        table.0.insert("bin".to_string(), decorator_bin);
        table.0.insert("oct".to_string(), decorator_oct);
        
        table.0.insert("sci".to_string(), decorator_sci);
        table.0.insert("float".to_string(), decorator_float);
        table.0.insert("int".to_string(), decorator_int);
        table.0.insert("bool".to_string(), decorator_bool);
        
        table.0.insert("utc".to_string(), decorator_utc);

        table.0.insert("dollar".to_string(), decorator_dollars);
        table.0.insert("dollars".to_string(), decorator_dollars);
        table.0.insert("usd".to_string(), decorator_dollars);
        table.0.insert("aud".to_string(), decorator_dollars);
        table.0.insert("cad".to_string(), decorator_dollars);
        
        table.0.insert("euro".to_string(), decorator_euros);
        table.0.insert("euros".to_string(), decorator_euros);
        
        table.0.insert("pound".to_string(), decorator_pounds);
        table.0.insert("pounds".to_string(), decorator_pounds);
        
        table.0.insert("yen".to_string(), decorator_yen);

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
        self.0.contains_key(name)
    }

    /// Call a decorator
    /// 
    /// # Arguments
    /// * `name` - Decorator name
    /// * `args` - Decorator arguments
    pub fn call(&self, name: &str, arg: &Value) -> Result<String, ParserError> {
        match self.0.get(name) {
            Some(f) => f(arg),
            None => Err(ParserError::DecoratorName(DecoratorNameError::new(name)))
        }
    }
}

impl Default for DecoratorTable {
    fn default() -> Self {
        Self::new()
    }
}

pub fn decorator_default(input: &Value) -> Result<String, ParserError> {
    match input {
        Value::Boolean(_) => decorator_bool(input),
        Value::Integer(_) => decorator_int(input),
        Value::Float(_) => decorator_float(input),
        Value::String(s) => Ok(s.to_string()),
        Value::None => Ok("".to_string())
    }
}

fn decorator_hex(input: &Value) -> Result<String, ParserError> {
    if let Some(n) = input.as_int() {
        Ok(format!("{:#x}", n))
    } else {
        Err(ParserError::FunctionArgType(FunctionArgTypeError::new("@hex", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn decorator_bin(input: &Value) -> Result<String, ParserError> {
    if let Some(n) = input.as_int() {
        Ok(format!("{:#0b}", n))
    } else {
        Err(ParserError::FunctionArgType(FunctionArgTypeError::new("@bin", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn decorator_oct(input: &Value) -> Result<String, ParserError> {
    if let Some(n) = input.as_int() {
        Ok(format!("{:#0o}", n))
    } else {
        Err(ParserError::FunctionArgType(FunctionArgTypeError::new("@oct", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn decorator_sci(input: &Value) -> Result<String, ParserError> {
    if let Some(n) = input.as_float() {
        Ok(format!("{:e}", n))
    } else {
        Err(ParserError::FunctionArgType(FunctionArgTypeError::new("@sci", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn decorator_utc(input: &Value) -> Result<String, ParserError> {
    if let Some(n) = input.as_int() {
        let t = NaiveDateTime::from_timestamp(n, 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(t, Utc);
        Ok(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
    } else {
        Err(ParserError::FunctionArgType(FunctionArgTypeError::new("@utc", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn decorator_currency(input: &Value, sig: &str, symbol: &str) -> Result<String, ParserError> {
    if let Some(n) = input.as_float() {
        let mut f = format!("{}{:.2}", symbol, n);
        if !f.contains('.') {
            f += ".0";
        }
        f = f
            .chars().rev().collect::<Vec<char>>()
            .chunks(3).map(|c| c.iter().collect::<String>()).collect::<Vec<String>>().join(",")
            .replacen(',', "", 1)
            .chars().rev().collect::<String>();
        if f.chars().nth(1).unwrap() == ',' {
            f = f.replacen(',', "", 1);
        }
        Ok(f)
    } else {
        Err(ParserError::FunctionArgType(FunctionArgTypeError::new(sig, 1, ExpectedTypes::IntOrFloat)))
    }
}

fn decorator_dollars(input: &Value) -> Result<String, ParserError> {
    decorator_currency(input, "@dollars", "$")
}

fn decorator_euros(input: &Value) -> Result<String, ParserError> {
    decorator_currency(input, "@euros", "€")
}

fn decorator_pounds(input: &Value) -> Result<String, ParserError> {
    decorator_currency(input, "@pounds", "£")
}

fn decorator_yen(input: &Value) -> Result<String, ParserError> {
    decorator_currency(input, "@yen", "¥")
}

fn decorator_float(input: &Value) -> Result<String, ParserError> {
    if let Some(n) = input.as_float() {
        Ok(Value::Float(n).as_string())
    } else {
        Err(ParserError::FunctionArgType(FunctionArgTypeError::new("@float", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn decorator_int(input: &Value) -> Result<String, ParserError> {
    if let Some(n) = input.as_int() {
        Ok(Value::Integer(n).as_string())
    } else {
        Err(ParserError::FunctionArgType(FunctionArgTypeError::new("@int", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn decorator_bool(input: &Value) -> Result<String, ParserError> {
    Ok(Value::Boolean(input.as_bool()).as_string())
}

#[cfg(test)]
mod test_builtin_functions {
    use super::*;
    
    #[test]
    fn test_default() {
    }

    #[test]
    fn test_hex() {
        assert_eq!("0xff", decorator_hex(&Value::Integer(255)).unwrap());
        assert_eq!("0xff", decorator_hex(&Value::Float(255.1)).unwrap());
    }

    #[test]
    fn test_bin() {
        assert_eq!("0b11111111", decorator_bin(&Value::Integer(255)).unwrap());
        assert_eq!("0b11111111", decorator_bin(&Value::Float(255.1)).unwrap());
    }

    #[test]
    fn test_oct() {
        assert_eq!("0o10", decorator_oct(&Value::Integer(8)).unwrap());
        assert_eq!("0o10", decorator_oct(&Value::Float(8.1)).unwrap());
    }

    #[test]
    fn test_sci() {
        assert_eq!("8e0", decorator_sci(&Value::Integer(8)).unwrap());
        assert_eq!("-8.1e1", decorator_sci(&Value::Float(-81.0)).unwrap());
        assert_eq!("8.1e-2", decorator_sci(&Value::Float(0.081)).unwrap());
    }

    #[test]
    fn test_float() {
        assert_eq!("8.0", decorator_float(&Value::Integer(8)).unwrap());
        assert_eq!("81.0", decorator_float(&Value::Float(81.0)).unwrap());
        assert_eq!("0.0", decorator_float(&Value::Float(0.0000000001)).unwrap());
        assert_eq!("0.081", decorator_float(&Value::Float(0.081)).unwrap());
    }

    #[test]
    fn test_int() {
        assert_eq!("-8", decorator_int(&Value::Integer(-8)).unwrap());
        assert_eq!("81", decorator_int(&Value::Float(81.0)).unwrap());
        assert_eq!("0", decorator_int(&Value::Float(0.081)).unwrap());
    }

    #[test]
    fn test_bool() {
        assert_eq!("false", decorator_bool(&Value::Integer(0)).unwrap());
        assert_eq!("true", decorator_bool(&Value::Integer(81)).unwrap());
        assert_eq!("true", decorator_bool(&Value::Float(0.081)).unwrap());
    }

    #[test]
    fn test_dollars() {
        assert_eq!("¥100.00", decorator_yen(&Value::Integer(100)).unwrap());
        assert_eq!("$1,000.00", decorator_dollars(&Value::Integer(1000)).unwrap());
        assert_eq!("€10,000.00", decorator_euros(&Value::Integer(10000)).unwrap());
        assert_eq!("£100,000.00", decorator_pounds(&Value::Integer(100000)).unwrap());
        assert_eq!("£1,000,000.00", decorator_pounds(&Value::Integer(1000000)).unwrap());
    }

    #[test]
    fn utc() {
        assert_eq!("2022-03-20 14:05:33", decorator_utc(&Value::Integer(1647785133)).unwrap());
    }
}