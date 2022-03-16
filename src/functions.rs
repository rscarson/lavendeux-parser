use super::value::{AtomicValue, IntegerType, FloatType};
use super::errors::*;
use std::collections::HashMap;

pub type FunctionHandler = fn(&[AtomicValue]) -> Result<AtomicValue, ParserError>;

#[derive(Clone)]
pub struct FunctionTable(HashMap<String, FunctionHandler>);
impl FunctionTable {
    /// Initialize a new function table, complete with default builtin functions
    pub fn new() -> FunctionTable {
        let mut table : FunctionTable = FunctionTable{0: HashMap::new()};

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
        table.register("strlen", builtin_strlen);
        table.register("substr", builtin_substr);

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

    /// Check if the table contains a function by the given name
    /// 
    /// # Arguments
    /// * `name` - Function name
    pub fn has(&self, name: &str) -> bool {
        return self.0.contains_key(name)
    }

    /// Call a function
    /// 
    /// # Arguments
    /// * `name` - Function name
    /// * `args` - Function arguments
    pub fn call(&self, name: &str, args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
        match self.0.get(name) {
            Some(f) => match f(&args[..]) {
                Ok(v) => Ok(v),
                Err(e) => Err(e)
            },
            None => Err(ParserError::FunctionName(FunctionNameError::new(name)))
        }
    }
}

fn builtin_ceil(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("ceil(n)", 1, 1)))
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Integer(n)),
        AtomicValue::Float(n) => Ok(AtomicValue::Integer(n.ceil() as IntegerType)),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("ceil(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_floor(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("floor(n)", 1, 1)))
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Integer(n)),
        AtomicValue::Float(n) => Ok(AtomicValue::Integer(n.floor() as IntegerType)),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("floor(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_round(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 && args.len() != 2 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("round(n, precision=0)", 1, 2)));
    }

    let precision = if args.len()== 1 {0} else {
        match args[1] {
            AtomicValue::Integer(n) => n,
            _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("round(n, precision=0)", 2, ExpectedTypes::Int)))
        }
    };

    if precision > u32::MAX as IntegerType { 
        return Err(ParserError::FunctionArgOverFlow(FunctionArgOverFlowError::new("round(n, precision=0)", 2))); 
    }

    let multiplier = f64::powi(10.0, precision as i32);

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Integer(n)),
        AtomicValue::Float(n) => Ok(AtomicValue::Float((n * multiplier).round() / multiplier)),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("round(n, precision=0)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_abs(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("abs(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Integer(n.abs())),
        AtomicValue::Float(n) => Ok(AtomicValue::Integer(n.abs() as IntegerType)),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("abs(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_to_radians(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("to_radians(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType) * (std::f64::consts::PI / 180.0))),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n * (std::f64::consts::PI / 180.0))),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("to_radians(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_tan(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("tan(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).tan())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.tan())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("tan(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_cos(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("cos(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).cos())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.cos())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("cos(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_sin(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("sin(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).sin())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.sin())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("sin(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_atan(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("atan(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).atan())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.atan())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("atan(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_acos(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("acos(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).acos())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.acos())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("acos(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_asin(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("asin(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).asin())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.asin())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("asin(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_tanh(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("tanh(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).tanh())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.tanh())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("tanh(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_cosh(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("cosh(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).cosh())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.cosh())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("cosh(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_sinh(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("sinh(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).sinh())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.sinh())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("sinh(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_log10(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("log10(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).log10())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.log10())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("log10(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_ln(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("ln(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).ln())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.ln())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("ln(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_log(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 2 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("log(n, base)", 2, 2)));
    }

    let base = match args[1].as_float() {
        Some(n) => n,
        None => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("log(n, base)", 2, ExpectedTypes::IntOrFloat)))
    };

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).log(base))),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.log(base))),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("log(n, base)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_sqrt(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("sqrt(n)", 1, 1)));
    }

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).sqrt())),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.sqrt())),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("sqrt(n)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_root(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 2 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("root(n, base)", 2, 2)));
    }

    let base = match args[1].as_float() {
        Some(n) => n,
        None => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("root(n, base)", 2, ExpectedTypes::IntOrFloat)))
    };

    match args[0] {
        AtomicValue::Integer(n) => Ok(AtomicValue::Float((n as FloatType).powf(1.0 / base))),
        AtomicValue::Float(n) => Ok(AtomicValue::Float(n.powf(1.0 / base))),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("root(n, base)", 1, ExpectedTypes::IntOrFloat)))
    }
}

fn builtin_strlen(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("strlen(s)", 1, 1)));
    }

    match &args[0] {
        AtomicValue::String(s) => Ok(AtomicValue::Integer(s.len() as IntegerType)),
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("strlen(s)", 1, ExpectedTypes::String)))
    }
}

fn builtin_substr(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 2 && args.len() != 3 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("substr(s, start, [length])", 2, 3)));
    }

    let start = match args[1].as_int() {
        Some(n) => n,
        None => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("substr(s, start, [length])", 2, ExpectedTypes::IntOrFloat)))
    };

    match &args[0] {
        AtomicValue::String(s) => {
            let length = if args.len() == 3 { match args[2].as_int() {
                Some(n) => n,
                None => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("substr(s, start, [length])", 3, ExpectedTypes::IntOrFloat)))
            } } else { s.len() as IntegerType - start };
            if start >= s.len() as IntegerType || start < 0 as IntegerType {
                return Err(ParserError::FunctionArgOverFlow(FunctionArgOverFlowError::new("substr(s, start, [length])", 2)));
            } else if length < 0 as IntegerType || length > (s.len() - start as usize) as IntegerType {
                return Err(ParserError::FunctionArgOverFlow(FunctionArgOverFlowError::new("substr(s, start, [length])", 3)));
            }

            Ok(AtomicValue::String(s.chars().skip(start as usize).take(length as usize).collect()))
        },
        _ => return Err(ParserError::FunctionArgType(FunctionArgTypeError::new("substr(s, start, [length])", 1, ExpectedTypes::String)))
    }
}

#[cfg(test)]
mod test_builtin_table {
    use super::*;
    
    #[test]
    fn test_register() {
        assert_eq!(AtomicValue::Integer(4), builtin_ceil(&[AtomicValue::Float(3.5)]).unwrap());
        assert_eq!(AtomicValue::Integer(4), builtin_ceil(&[AtomicValue::Integer(4)]).unwrap());
    }
    
    #[test]
    fn test_has() {
        assert_eq!(AtomicValue::Integer(4), builtin_ceil(&[AtomicValue::Float(3.5)]).unwrap());
        assert_eq!(AtomicValue::Integer(4), builtin_ceil(&[AtomicValue::Integer(4)]).unwrap());
    }
    
    #[test]
    fn test_run() {
        assert_eq!(AtomicValue::Integer(4), builtin_ceil(&[AtomicValue::Float(3.5)]).unwrap());
        assert_eq!(AtomicValue::Integer(4), builtin_ceil(&[AtomicValue::Integer(4)]).unwrap());
    }
}

#[cfg(test)]
mod test_builtin_functions {
    use super::*;
    
    #[test]
    fn test_ceil() {
        assert_eq!(AtomicValue::Integer(4), builtin_ceil(&[AtomicValue::Float(3.5)]).unwrap());
        assert_eq!(AtomicValue::Integer(4), builtin_ceil(&[AtomicValue::Integer(4)]).unwrap());
    }
    
    #[test]
    fn test_floor() {
        assert_eq!(AtomicValue::Integer(3), builtin_floor(&[AtomicValue::Float(3.5)]).unwrap());
        assert_eq!(AtomicValue::Integer(4), builtin_floor(&[AtomicValue::Integer(4)]).unwrap());
    }
    
    #[test]
    fn test_round() {
        assert_eq!(AtomicValue::Float(3.56), builtin_round(&[AtomicValue::Float(3.555), AtomicValue::Integer(2)]).unwrap());
        assert_eq!(AtomicValue::Float(4.0), builtin_round(&[AtomicValue::Integer(4), AtomicValue::Integer(2)]).unwrap());
    }
    
    #[test]
    fn test_abs() {
        assert_eq!(AtomicValue::Integer(3), builtin_abs(&[AtomicValue::Integer(3)]).unwrap());
        assert_eq!(AtomicValue::Integer(3), builtin_abs(&[AtomicValue::Integer(-3)]).unwrap());
        assert_eq!(AtomicValue::Float(4.0), builtin_abs(&[AtomicValue::Float(-4.0)]).unwrap());
    }
        
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
    
    #[test]
    fn test_ln() {
        assert_eq!(AtomicValue::Float(1.0), builtin_ln(&[AtomicValue::Float(std::f64::consts::E)]).unwrap());
    }
    
    #[test]
    fn test_log10() {
        assert_eq!(AtomicValue::Float(2.0), builtin_log10(&[AtomicValue::Float(100.0)]).unwrap());
    }
    
    #[test]
    fn test_log() {
        assert_eq!(AtomicValue::Float(2.0), builtin_log(&[AtomicValue::Float(100.0), AtomicValue::Integer(10)]).unwrap());
    }
    
    #[test]
    fn test_sqrt() {
        assert_eq!(AtomicValue::Float(3.0), builtin_sqrt(&[AtomicValue::Float(9.0)]).unwrap());
    }
    
    #[test]
    fn test_root() {
        assert_eq!(AtomicValue::Float(3.0), builtin_root(&[AtomicValue::Float(27.0), AtomicValue::Integer(3)]).unwrap());
    }

    #[test]
    fn test_strlen() {
        assert_eq!(AtomicValue::Integer(0), builtin_strlen(&[AtomicValue::String("".to_string())]).unwrap());
        assert_eq!(AtomicValue::Integer(3), builtin_strlen(&[AtomicValue::String("   ".to_string())]).unwrap());
    }
    
    #[test]
    fn test_substr() {
        assert_eq!(AtomicValue::String("t".to_string()), builtin_substr(
            &[AtomicValue::String("test".to_string()), AtomicValue::Integer(3)]
            
        ).unwrap());
        assert_eq!(AtomicValue::String("tes".to_string()), builtin_substr(
            &[AtomicValue::String("test".to_string()), AtomicValue::Integer(0), AtomicValue::Integer(3)], 
        ).unwrap());
    }
}