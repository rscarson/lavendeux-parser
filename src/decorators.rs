use super::value::Value;
use super::errors::*;
use std::collections::HashMap;
use chrono::prelude::*;

pub type DecoratorHandler = fn(&DecoratorDefinition, &Value) -> Result<String, ParserError>;

/// Holds a set of callable decorators
#[derive(Clone)]
pub struct DecoratorTable(HashMap<String, DecoratorDefinition>);
impl DecoratorTable {
    /// Initialize a new decorator table, complete with default builtin decorators
    pub fn new() -> DecoratorTable {
        let mut table : DecoratorTable = DecoratorTable(HashMap::new());

        table.register(DEFAULT);
        table.register(HEX);
        table.register(OCT);
        table.register(BIN);
        
        table.register(SCI);
        table.register(FLOAT);
        table.register(INT);
        table.register(BOOL);
        table.register(ARRAY);
        
        table.register(UTC);
        table.register(DOLLAR);
        table.register(EURO);
        table.register(POUND);
        table.register(YEN);

        table
    }

    /// Register a decorator in the table
    /// 
    /// # Arguments
    /// * `name` - Decorator name
    /// * `handler` - Decorator handler
    pub fn register(&mut self, definition: DecoratorDefinition) {
        for name in definition.name() {
            self.0.insert(name.to_string(), definition.clone());
        }
    }

    /// Check if the table contains a decorator by the given name
    /// 
    /// # Arguments
    /// * `name` - Decorator name
    pub fn has(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    /// Return a given decorator
    /// 
    /// # Arguments
    /// * `name` - Function name
    pub fn get(&self, name: &str) -> Option<&DecoratorDefinition> {
        self.0.get(name)
    }

    /// Get a collection of all included decorators
    pub fn all(&self) -> Vec<&String> {
        self.0.keys().collect::<Vec<&String>>()
    }

    /// Call a decorator
    /// 
    /// # Arguments
    /// * `name` - Decorator name
    /// * `args` - Decorator arguments
    pub fn call(&self, name: &str, arg: &Value) -> Result<String, ParserError> {
        match self.0.get(name) {
            Some(f) => f.call(arg),
            None => Err(ParserError::DecoratorName(DecoratorNameError::new(name)))
        }
    }
}

impl Default for DecoratorTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Holds the definition of a builtin callable decorator
#[derive(Clone)]
pub struct DecoratorDefinition {
    /// Decorator call name
    pub name: &'static [&'static str],
    
    /// Decorator short description
    pub description: &'static str,

    /// Type of input the decorator expects
    pub argument: ExpectedTypes,

    /// Handler function
    pub handler: DecoratorHandler
}
impl DecoratorDefinition {
    /// Return the decorator's names
    pub fn name(&self) -> &[&str] {
        self.name
    }
    
    /// Return the decorator's description
    pub fn description(&self) -> &str {
        self.description
    }

    /// Return the decorator's argument type
    pub fn arg(&self) -> ExpectedTypes {
        self.argument.clone()
    }
    
    /// Return the decorator's signature
    pub fn signature(&self) -> String {
        self.name.iter().map(|n|format!("@{n}")).collect::<Vec<String>>().join("/")
    }
    
    /// Return the decorator's signature
    pub fn help(&self) -> String {
        format!("{}: {}", self.signature(), self.description)
    }

    /// Validate decorator arguments, and return an error if one exists
    /// 
    /// # Arguments
    /// * `arg` - Decorator input
    pub fn validate(&self, arg: &Value) -> Option<ParserError> {
        let valid = match self.arg() {
            ExpectedTypes::Float => arg.is_float(),
            ExpectedTypes::Int => arg.is_int(),
            ExpectedTypes::IntOrFloat => arg.is_float() || arg.is_int(),

            // These can be converted from any type
            ExpectedTypes::String => true, 
            ExpectedTypes::Boolean => true, 
            ExpectedTypes::Array => true, 
            ExpectedTypes::Any => true
        };
        
        if !valid {
            Some(ParserError::FunctionArgType(FunctionArgTypeError::new(&self.signature(), 1, self.arg())))
        } else {
            None
        }
    }

    // Call the associated decorator handler
    /// 
    /// # Arguments
    /// * `arg` - Decorator input
    pub fn call(&self, arg: &Value) -> Result<String, ParserError> {
        if let Some(error) = self.validate(arg) {
            Err(error)
        } else {
            (self.handler)(self, arg)
        }
    }
}

fn decorator_currency(input: &Value, symbol: &str) -> Result<String, ParserError> {
    let n = input.as_float().unwrap();
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
}

const DEFAULT : DecoratorDefinition = DecoratorDefinition {
    name: &["default"],
    description: "Default formatter, type dependent",
    argument: ExpectedTypes::Any,
    handler: |_, input| match input {
        Value::Boolean(_) => (BOOL.handler)(&BOOL, input),
        Value::Integer(_) => (INT.handler)(&INT, input),
        Value::Float(_) => (FLOAT.handler)(&FLOAT, input),
        Value::Array(_) => (ARRAY.handler)(&ARRAY, input),
        Value::String(s) => Ok(s.to_string()),
        Value::None => Ok("".to_string())
    }
};

const HEX : DecoratorDefinition = DecoratorDefinition {
    name: &["hex"],
    description: "Base 16 number formatting, such as 0xFF",
    argument: ExpectedTypes::IntOrFloat,
    handler: |_, input| Ok(format!("{:#x}", input.as_int().unwrap()))
};

const OCT : DecoratorDefinition = DecoratorDefinition {
    name: &["oct"],
    description: "Base 8 number formatting, such as 0b77",
    argument: ExpectedTypes::IntOrFloat,
    handler: |_, input| Ok(format!("{:#0o}", input.as_int().unwrap()))
};

const BIN : DecoratorDefinition = DecoratorDefinition {
    name: &["bin"],
    description: "Base 2 number formatting, such as 0b11",
    argument: ExpectedTypes::IntOrFloat,
    handler: |_, input| Ok(format!("{:#0b}", input.as_int().unwrap()))
};

const SCI : DecoratorDefinition = DecoratorDefinition {
    name: &["sci"],
    description: "Scientific number formatting, such as 1.2Ee-3",
    argument: ExpectedTypes::IntOrFloat,
    handler: |_, input| Ok(format!("{:e}", input.as_float().unwrap()))
};

const UTC : DecoratorDefinition = DecoratorDefinition {
    name: &["utc"],
    description: "Interprets an integer as a timestamp, and formats it in UTC standard",
    argument: ExpectedTypes::IntOrFloat,
    handler: |_, input| {
        let n = input.as_int().unwrap();
        let t = NaiveDateTime::from_timestamp(n, 0);
        let datetime: DateTime<Utc> = DateTime::from_utc(t, Utc);
        Ok(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
    }
};

const DOLLAR : DecoratorDefinition = DecoratorDefinition {
    name: &["dollar", "dollars", "usd", "aud", "cad"],
    description: "Format a number as a dollar amount",
    argument: ExpectedTypes::IntOrFloat,
    handler: |_, input| decorator_currency(input, "$")
};

const EURO : DecoratorDefinition = DecoratorDefinition {
    name: &["euro", "euros"],
    description: "Format a number as a euro amount",
    argument: ExpectedTypes::IntOrFloat,
    handler: |_, input| decorator_currency(input, "€")
};

const POUND : DecoratorDefinition = DecoratorDefinition {
    name: &["pound", "pounds"],
    description: "Format a number as a pound amount",
    argument: ExpectedTypes::IntOrFloat,
    handler: |_, input| decorator_currency(input, "£")
};

const YEN : DecoratorDefinition = DecoratorDefinition {
    name: &["yen"],
    description: "Format a number as a yen amount",
    argument: ExpectedTypes::IntOrFloat,
    handler: |_, input| decorator_currency(input, "¥")
};

const FLOAT : DecoratorDefinition = DecoratorDefinition {
    name: &["float"],
    description: "Format a number as floating point",
    argument: ExpectedTypes::IntOrFloat,
    handler: |_, input| Ok(Value::Float(input.as_float().unwrap()).as_string())
};

const INT : DecoratorDefinition = DecoratorDefinition {
    name: &["int", "integer"],
    description: "Format a number as an integer",
    argument: ExpectedTypes::IntOrFloat,
    handler: |_, input| Ok(Value::Integer(input.as_int().unwrap()).as_string())
};

const BOOL : DecoratorDefinition = DecoratorDefinition {
    name: &["bool", "boolean"],
    description: "Format a number as a boolean",
    argument: ExpectedTypes::Any,
    handler: |_, input| Ok(Value::Boolean(input.as_bool()).as_string())
};

const ARRAY : DecoratorDefinition = DecoratorDefinition {
    name: &["array"],
    description: "Format a number as an array",
    argument: ExpectedTypes::Any,
    handler: |_, input| Ok(Value::Array(input.as_array()).as_string())
};

#[cfg(test)]
mod test_builtin_functions {
    use super::*;
    
    #[test]
    fn test_default() {
    }

    #[test]
    fn test_hex() {
        assert_eq!("0xff", HEX.call(&Value::Integer(255)).unwrap());
        assert_eq!("0xff", HEX.call(&Value::Float(255.1)).unwrap());
    }

    #[test]
    fn test_bin() {
        assert_eq!("0b11111111", BIN.call(&Value::Integer(255)).unwrap());
        assert_eq!("0b11111111", BIN.call(&Value::Float(255.1)).unwrap());
    }

    #[test]
    fn test_oct() {
        assert_eq!("0o10", OCT.call(&Value::Integer(8)).unwrap());
        assert_eq!("0o10", OCT.call(&Value::Float(8.1)).unwrap());
    }

    #[test]
    fn test_sci() {
        assert_eq!("8e0", SCI.call(&Value::Integer(8)).unwrap());
        assert_eq!("-8.1e1", SCI.call(&Value::Float(-81.0)).unwrap());
        assert_eq!("8.1e-2", SCI.call(&Value::Float(0.081)).unwrap());
    }

    #[test]
    fn test_float() {
        assert_eq!("8.0", FLOAT.call(&Value::Integer(8)).unwrap());
        assert_eq!("81.0", FLOAT.call(&Value::Float(81.0)).unwrap());
        assert_eq!("0.0", FLOAT.call(&Value::Float(0.0000000001)).unwrap());
        assert_eq!("0.081", FLOAT.call(&Value::Float(0.081)).unwrap());
    }

    #[test]
    fn test_int() {
        assert_eq!("-8", INT.call(&Value::Integer(-8)).unwrap());
        assert_eq!("81", INT.call(&Value::Float(81.0)).unwrap());
        assert_eq!("0", INT.call(&Value::Float(0.081)).unwrap());
    }

    #[test]
    fn test_bool() {
        assert_eq!("false", BOOL.call(&Value::Integer(0)).unwrap());
        assert_eq!("true", BOOL.call(&Value::Integer(81)).unwrap());
        assert_eq!("true", BOOL.call(&Value::Float(0.081)).unwrap());
    }

    #[test]
    fn test_dollars() {
        assert_eq!("¥100.00", YEN.call(&Value::Integer(100)).unwrap());
        assert_eq!("$1,000.00", DOLLAR.call(&Value::Integer(1000)).unwrap());
        assert_eq!("€10,000.00", EURO.call(&Value::Integer(10000)).unwrap());
        assert_eq!("£100,000.00", POUND.call(&Value::Integer(100000)).unwrap());
        assert_eq!("£1,000,000.00", POUND.call(&Value::Integer(1000000)).unwrap());
    }

    #[test]
    fn utc() {
        assert_eq!("2022-03-20 14:05:33", UTC.call(&Value::Integer(1647785133)).unwrap());
    }
}