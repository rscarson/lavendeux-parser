use super::value::{Value, IntegerType, FloatType};
use super::errors::*;
use std::collections::HashMap;

pub type FunctionHandler = fn(&[Value]) -> Result<Value, ParserError>;

mod trig;
use trig::*;

mod dev;
use dev::*;

mod network;
use network::*;

#[derive(Clone)]
pub struct FunctionTable(HashMap<String, FunctionHandler>);
impl FunctionTable {
    /// Initialize a new function table, complete with default builtin functions
    pub fn new() -> FunctionTable {
        let mut table : FunctionTable = FunctionTable(HashMap::new());

        // Rounding functions
        table.register("ceil", builtin_ceil);
        table.register("floor", builtin_floor);
        table.register("round", builtin_round);
        
        // Conversion functions
        table.register("to_radians", builtin_to_radians);
        table.register("abs", builtin_abs);
        
        // Trig functions
        table.register("tan", builtin_tan);
        table.register("cos", builtin_cos);
        table.register("sin", builtin_sin);
        table.register("atan", builtin_atan);
        table.register("acos", builtin_acos);
        table.register("asin", builtin_asin);
        table.register("tanh", builtin_tanh);
        table.register("cosh", builtin_cosh);
        table.register("sinh", builtin_sinh);
        
        // Roots and logs
        table.register("ln", builtin_ln);
        table.register("log10", builtin_log10);
        table.register("log", builtin_log);
        table.register("sqrt", builtin_sqrt);
        table.register("root", builtin_root);
        
        // String functions
        table.register("concat", builtin_concat);
        table.register("strlen", builtin_strlen);
        table.register("substr", builtin_substr);
        table.register("contains", builtin_contains);
        
        // Developper functions
        table.register("choose", builtin_choose);
        table.register("rand", builtin_rand);
        table.register("time", builtin_time);
        table.register("tail", builtin_tail);

        // Networking functions
        table.register("get", builtin_get);
        table.register("post", builtin_post);
        table.register("resolve", builtin_resolve);

        table
    }

    /// Register a function in the table
    /// 
    /// # Arguments
    /// * `name` - Function name
    /// * `handler` - Function handler
    pub fn register(&mut self, name: &str, handler: FunctionHandler) {
        self.0.insert(name.to_string(), handler);
    }

    /// Remove a function from the table
    /// 
    /// # Arguments
    /// * `name` - Function name
    pub fn remove(&mut self, name: &str) {
        self.0.remove(&name.to_string());
    }

    /// Check if the table contains a function by the given name
    /// 
    /// # Arguments
    /// * `name` - Function name
    pub fn has(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    /// Call a function
    /// 
    /// # Arguments
    /// * `name` - Function name
    /// * `args` - Function arguments
    pub fn call(&self, name: &str, args: &[Value]) -> Result<Value, ParserError> {
        match self.0.get(name) {
            Some(f) => f(args),
            None => Err(ParserError::FunctionName(FunctionNameError::new(name)))
        }
    }
}

impl Default for FunctionTable {
    fn default() -> Self {
        Self::new()
    }
}

fn builtin_ceil(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("ceil(n)", 1, 1)))
    }

    match args[0] {
        Value::Integer(n) => Ok(Value::Integer(n)),
        Value::Float(n) => Ok(Value::Integer(n.ceil() as IntegerType)),
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("ceil(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_floor(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("floor(n)", 1, 1)))
    }

    match args[0] {
        Value::Integer(n) => Ok(Value::Integer(n)),
        Value::Float(n) => Ok(Value::Integer(n.floor() as IntegerType)),
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("floor(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_round(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 1 && args.len() != 2 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("round(n, precision=0)", 1, 2)));
    }

    let precision = if args.len()== 1 {0} else {
        match args[1] {
            Value::Integer(n) => n,
            _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("round(n, precision=0)", 2, ExpectedTypes::Int)))
        }
    };

    if precision > u32::MAX as IntegerType { 
        return Err(ParserError::FunctionArgOverFlow(FunctionArgOverFlowError::new("round(n, precision=0)", 2))); 
    }

    let multiplier = f64::powi(10.0, precision as i32);

    match args[0] {
        Value::Integer(n) => Ok(Value::Integer(n)),
        Value::Float(n) => Ok(Value::Float((n * multiplier).round() / multiplier)),
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("round(n, precision=0)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_abs(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("abs(n)", 1, 1)));
    }

    match args[0] {
        Value::Integer(n) => Ok(Value::Integer(n.abs())),
        Value::Float(n) => Ok(Value::Integer(n.abs() as IntegerType)),
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("abs(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_log10(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("log10(n)", 1, 1)));
    }

    match args[0] {
        Value::Integer(n) => Ok(Value::Float((n as FloatType).log10())),
        Value::Float(n) => Ok(Value::Float(n.log10())),
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("log10(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_ln(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("ln(n)", 1, 1)));
    }

    match args[0] {
        Value::Integer(n) => Ok(Value::Float((n as FloatType).ln())),
        Value::Float(n) => Ok(Value::Float(n.ln())),
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("ln(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_log(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 2 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("log(n, base)", 2, 2)));
    }

    let base = match args[1].as_float() {
        Some(n) => n,
        None => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("log(n, base)", 2, ExpectedTypes::IntOrFloat)))
    };

    match args[0] {
        Value::Integer(n) => Ok(Value::Float((n as FloatType).log(base))),
        Value::Float(n) => Ok(Value::Float(n.log(base))),
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("log(n, base)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_sqrt(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("sqrt(n)", 1, 1)));
    }

    match args[0] {
        Value::Integer(n) => Ok(Value::Float((n as FloatType).sqrt())),
        Value::Float(n) => Ok(Value::Float(n.sqrt())),
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("sqrt(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_root(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 2 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("root(n, base)", 2, 2)));
    }

    let base = match args[1].as_float() {
        Some(n) => n,
        None => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("root(n, base)", 2, ExpectedTypes::IntOrFloat)))
    };

    match args[0] {
        Value::Integer(n) => Ok(Value::Float((n as FloatType).powf(1.0 / base))),
        Value::Float(n) => Ok(Value::Float(n.powf(1.0 / base))),
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("root(n, base)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_contains(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 2 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("contains(source, s)", 1, 1)));
    }

    Ok(Value::Boolean(args[0].as_string().contains(&args[1].as_string())))
}

fn builtin_strlen(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("strlen(s)", 1, 1)));
    }

    match &args[0] {
        Value::String(s) => Ok(Value::Integer(s.len() as IntegerType)),
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("strlen(s)", 1, ExpectedTypes::String)))
    }
}

fn builtin_concat(args: &[Value]) -> Result<Value, ParserError> {
    if args.is_empty() {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("strlen(s)", 1, 1)));
    }

    Ok(Value::String(args.iter().map(|v|v.as_string()).collect::<String>()))
}

fn builtin_substr(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 2 && args.len() != 3 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("substr(s, start, [length])", 2, 3)));
    }

    let start = match args[1].as_int() {
        Some(n) => n,
        None => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("substr(s, start, [length])", 2, ExpectedTypes::IntOrFloat)))
    };

    match &args[0] {
        Value::String(s) => {
            let length = if args.len() == 3 { match args[2].as_int() {
                Some(n) => n,
                None => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("substr(s, start, [length])", 3, ExpectedTypes::IntOrFloat)))
            } } else { s.len() as IntegerType - start };
            if start >= s.len() as IntegerType || start < 0 {
                return Err(ParserError::FunctionArgOverFlow(FunctionArgOverFlowError::new("substr(s, start, [length])", 2)));
            } else if length < 0 || length > (s.len() - start as usize) as IntegerType {
                return Err(ParserError::FunctionArgOverFlow(FunctionArgOverFlowError::new("substr(s, start, [length])", 3)));
            }

            Ok(Value::String(s.chars().skip(start as usize).take(length as usize).collect()))
        },
        _ => Err(ParserError::FunctionArgType(FunctionArgTypeError::new("substr(s, start, [length])", 1, ExpectedTypes::String)))
    }
}

#[cfg(test)]
mod test_builtin_table {
    use super::*;
    
    #[test]
    fn test_register() {
        assert_eq!(Value::Integer(4), builtin_ceil(&[Value::Float(3.5)]).unwrap());
        assert_eq!(Value::Integer(4), builtin_ceil(&[Value::Integer(4)]).unwrap());
    }
    
    #[test]
    fn test_has() {
        assert_eq!(Value::Integer(4), builtin_ceil(&[Value::Float(3.5)]).unwrap());
        assert_eq!(Value::Integer(4), builtin_ceil(&[Value::Integer(4)]).unwrap());
    }
    
    #[test]
    fn test_run() {
        assert_eq!(Value::Integer(4), builtin_ceil(&[Value::Float(3.5)]).unwrap());
        assert_eq!(Value::Integer(4), builtin_ceil(&[Value::Integer(4)]).unwrap());
    }
}

#[cfg(test)]
mod test_builtin_functions {
    use super::*;
    
    #[test]
    fn test_ceil() {
        assert_eq!(Value::Integer(4), builtin_ceil(&[Value::Float(3.5)]).unwrap());
        assert_eq!(Value::Integer(4), builtin_ceil(&[Value::Integer(4)]).unwrap());
    }
    
    #[test]
    fn test_floor() {
        assert_eq!(Value::Integer(3), builtin_floor(&[Value::Float(3.5)]).unwrap());
        assert_eq!(Value::Integer(4), builtin_floor(&[Value::Integer(4)]).unwrap());
    }
    
    #[test]
    fn test_round() {
        assert_eq!(Value::Float(3.56), builtin_round(&[Value::Float(3.555), Value::Integer(2)]).unwrap());
        assert_eq!(Value::Float(4.0), builtin_round(&[Value::Integer(4), Value::Integer(2)]).unwrap());
    }
    
    #[test]
    fn test_abs() {
        assert_eq!(Value::Integer(3), builtin_abs(&[Value::Integer(3)]).unwrap());
        assert_eq!(Value::Integer(3), builtin_abs(&[Value::Integer(-3)]).unwrap());
        assert_eq!(Value::Float(4.0), builtin_abs(&[Value::Float(-4.0)]).unwrap());
    }
    
    #[test]
    fn test_ln() {
        assert_eq!(Value::Float(1.0), builtin_ln(&[Value::Float(std::f64::consts::E)]).unwrap());
    }
    
    #[test]
    fn test_log10() {
        assert_eq!(Value::Float(2.0), builtin_log10(&[Value::Float(100.0)]).unwrap());
    }
    
    #[test]
    fn test_log() {
        assert_eq!(Value::Float(2.0), builtin_log(&[Value::Float(100.0), Value::Integer(10)]).unwrap());
    }
    
    #[test]
    fn test_sqrt() {
        assert_eq!(Value::Float(3.0), builtin_sqrt(&[Value::Float(9.0)]).unwrap());
    }
    
    #[test]
    fn test_root() {
        assert_eq!(Value::Float(3.0), builtin_root(&[Value::Float(27.0), Value::Integer(3)]).unwrap());
    }

    #[test]
    fn test_strlen() {
        assert_eq!(Value::Integer(0), builtin_strlen(&[Value::String("".to_string())]).unwrap());
        assert_eq!(Value::Integer(3), builtin_strlen(&[Value::String("   ".to_string())]).unwrap());
    }

    #[test]
    fn test_concat() {
        assert_eq!(Value::String(" ".to_string()), builtin_concat(
            &[Value::String("".to_string()), 
            Value::String(" ".to_string())
        ]).unwrap());
        assert_eq!(Value::String("test4false".to_string()), builtin_concat(
            &[Value::String("test".to_string()), 
            Value::Integer(4),
            Value::Boolean(false)
        ]).unwrap());
    }
    
    #[test]
    fn test_substr() {
        assert_eq!(Value::String("t".to_string()), builtin_substr(
            &[Value::String("test".to_string()), Value::Integer(3)]
            
        ).unwrap());
        assert_eq!(Value::String("tes".to_string()), builtin_substr(
            &[Value::String("test".to_string()), Value::Integer(0), Value::Integer(3)], 
        ).unwrap());
    }
    
    #[test]
    fn test_contains() {
        assert_eq!(Value::Boolean(true), builtin_contains(
            &[Value::String("test".to_string()), Value::String("e".to_string())]
            
        ).unwrap());
        assert_eq!(Value::Boolean(false), builtin_contains(
            &[Value::String("test".to_string()), Value::String("fff".to_string())]
            
        ).unwrap());
    }
}