use crate::{Value, errors::*, Token, value::ObjectType};

use std::collections::HashMap;
use chrono::prelude::*;

/// Handler for executing a decorator
pub type DecoratorHandler = fn(&DecoratorDefinition, &Token, &Value) -> Result<String, ParserError>;

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
        table.register(OBJECT);
        
        table.register(UTC);
        table.register(DOLLAR);
        table.register(EURO);
        table.register(POUND);
        table.register(YEN);

        table.register(ROMAN);
        table.register(ORDINAL);
        table.register(PERCENTAGE);

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
    pub fn all(&self) -> Vec<&DecoratorDefinition> {
        let mut a: Vec<&DecoratorDefinition> = self.0.values().collect();
        a.sort_by(|f1, f2|f1.name()[0].cmp(f2.name()[0]));
        a
    }

    /// Call a decorator
    /// 
    /// # Arguments
    /// * `name` - Decorator name
    /// * `args` - Decorator arguments
    pub fn call(&self, name: &str, token: &Token, arg: &Value) -> Result<String, ParserError> {
        match self.0.get(name) {
            Some(f) => f.call(token, arg),
            None => Err(DecoratorNameError::new(token, name).into())
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
    pub fn validate(&self, token: &Token, arg: &Value) -> Option<ParserError> {
        if !self.arg().matches(arg) {
            Some(DecoratorArgTypeError::new(token, &self.signature(), self.arg()).into())
        } else {
            None
        }
    }

    // Call the associated decorator handler
    /// 
    /// # Arguments
    /// * `arg` - Decorator input
    pub fn call(&self, token: &Token, arg: &Value) -> Result<String, ParserError> {
        if let Some(error) = self.validate(token, arg) {
            Err(error)
        } else {
            (self.handler)(self, token, arg)
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

fn pluralized_decorator(decorator: &DecoratorDefinition, token: &Token, input: &Value) -> Result<String, ParserError> {
    match input {
        Value::Array(v) => {
            let mut output : Vec<Value> = Vec::new();
            for value in v {
                match decorator.call(token, value) {
                    Ok(s) => output.push(Value::from(s)),
                    Err(e) => return Err(e)
                }
            }
            Ok(Value::from(output).as_string())
        },

        Value::Object(v) => {
            let mut output : ObjectType = ObjectType::new();
            for (value, key) in v {
                match decorator.call(token, value) {
                    Ok(s) => {output.insert(key.clone(), Value::from(s));},
                    Err(e) => return Err(e)
                }
            }
            Ok(Value::from(output).as_string())
        },

        _ => decorator.call(token, input)
    }
}

const DEFAULT : DecoratorDefinition = DecoratorDefinition {
    name: &["default"],
    description: "Default formatter, type dependent",
    argument: ExpectedTypes::Any,
    handler: |_, token, input| match input {
        Value::Boolean(_) => (BOOL.handler)(&BOOL, token, input),
        Value::Integer(_) => (INT.handler)(&INT, token, input),
        Value::Float(_) => (FLOAT.handler)(&FLOAT, token, input),
        Value::Array(_) => (ARRAY.handler)(&ARRAY, token, input),
        Value::Object(_) => (OBJECT.handler)(&OBJECT, token, input),
        Value::String(s) => Ok(s.to_string()),
        Value::Identifier(_) => Ok("".to_string()),
        Value::None => Ok("".to_string())
    }
};

const HEX : DecoratorDefinition = DecoratorDefinition {
    name: &["hex"],
    description: "Base 16 number formatting, such as 0xFF",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            Ok(format!("{:#0x}", input.as_int().unwrap()))
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
};

const OCT : DecoratorDefinition = DecoratorDefinition {
    name: &["oct"],
    description: "Base 8 number formatting, such as 0b77",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            Ok(format!("{:#0o}", input.as_int().unwrap()))
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
};

const BIN : DecoratorDefinition = DecoratorDefinition {
    name: &["bin"],
    description: "Base 2 number formatting, such as 0b11",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            Ok(format!("{:#0b}", input.as_int().unwrap()))
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
};

const SCI : DecoratorDefinition = DecoratorDefinition {
    name: &["sci"],
    description: "Scientific number formatting, such as 1.2Ee-3",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            Ok(format!("{:e}", input.as_float().unwrap()))
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
};

const UTC : DecoratorDefinition = DecoratorDefinition {
    name: &["utc"],
    description: "Interprets an integer as a timestamp, and formats it in UTC standard",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            let n = input.as_int().unwrap();
            match NaiveDateTime::from_timestamp_millis(n*1000) {
                Some(t) => {
                    let datetime: DateTime<Utc> = DateTime::from_utc(t, Utc);
                    Ok(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
                },
                None => Err(RangeError::new(token, input).into())
            }
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
};

const DOLLAR : DecoratorDefinition = DecoratorDefinition {
    name: &["dollar", "dollars", "usd", "aud", "cad"],
    description: "Format a number as a dollar amount",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            decorator_currency(input, "$")
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
};

const EURO : DecoratorDefinition = DecoratorDefinition {
    name: &["euro", "euros"],
    description: "Format a number as a euro amount",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            decorator_currency(input, "€")
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
};

const POUND : DecoratorDefinition = DecoratorDefinition {
    name: &["pound", "pounds"],
    description: "Format a number as a pound amount",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            decorator_currency(input, "£")
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
};

const YEN : DecoratorDefinition = DecoratorDefinition {
    name: &["yen"],
    description: "Format a number as a yen amount",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            decorator_currency(input, "¥")
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
};

const FLOAT : DecoratorDefinition = DecoratorDefinition {
    name: &["float"],
    description: "Format a number as floating point",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            Ok(Value::Float(input.as_float().unwrap()).as_string())
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
};

const INT : DecoratorDefinition = DecoratorDefinition {
    name: &["int", "integer"],
    description: "Format a number as an integer",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            Ok(Value::Integer(input.as_int().unwrap()).as_string())
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
};

const BOOL : DecoratorDefinition = DecoratorDefinition {
    name: &["bool", "boolean"],
    description: "Format a number as a boolean",
    argument: ExpectedTypes::Any,
    handler: |_, _, input| Ok(Value::Boolean(input.as_bool()).as_string())
};

const ARRAY : DecoratorDefinition = DecoratorDefinition {
    name: &["array"],
    description: "Format a number as an array",
    argument: ExpectedTypes::Any,
    handler: |_, _, input| Ok(Value::Array(input.as_array()).as_string())
};

const OBJECT : DecoratorDefinition = DecoratorDefinition {
    name: &["object"],
    description: "Format a number as an object",
    argument: ExpectedTypes::Any,
    handler: |_, _, input| Ok(Value::Object(input.as_object()).as_string())
};

const PERCENTAGE : DecoratorDefinition = DecoratorDefinition {
    name: &["percentage", "percent"],
    description: "Format a floating point number as a percentage",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            Ok(format!("{}%", input.as_float().unwrap()*100.0))
        } else {
            pluralized_decorator(decorator, token, input)
        }
    } 
};

const ORDINAL : DecoratorDefinition = DecoratorDefinition {
    name: &["percentage", "percent"],
    description: "Format an integer as an ordinal (1st, 38th, etc)",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            let v = Value::Integer(input.as_int().unwrap()).as_string();
            let suffix = 
                if v.ends_with('1') { "st" } 
                else if v.ends_with('2') { "nd" } 
                else if v.ends_with('3') { "rd" } 
                else { "th" };
           Ok(format!("{}{}", v, suffix))
        } else {
            pluralized_decorator(decorator, token, input)
        }
    } 
};

const ROMAN : DecoratorDefinition = DecoratorDefinition {
    name: &["roman"],
    description: "Format an integer as a roman numeral",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            let mut value = input.as_int().unwrap();
            if value > 3999 {
                return Err(OverflowError::new(token).into());
            }

            let roman_numerals = vec![
                (1000, "M"), (900, "CM"),
                (500, "D"), (400, "CD"),
                (100, "C"), (90, "XC"),
                (50, "L"), (40, "XL"),
                (10, "X"), (9, "IX"),
                (5, "V"), (4, "IV"),
                (1, "I"),
            ];
            let mut roman_numeral = String::new();
            for (n, r) in roman_numerals {
                while value >= n {
                    roman_numeral.push_str(r);
                    value -= n;
                }
            }
            Ok(roman_numeral)
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
};

#[cfg(test)]
mod test_builtin_functions {
    use super::*;
    
    #[test]
    fn test_default() {
    }

    #[test]
    fn test_hex() {
        assert_eq!("0xff", HEX.call(&Token::dummy(""), &Value::Integer(255)).unwrap());
        assert_eq!("0xff", HEX.call(&Token::dummy(""), &Value::Float(255.1)).unwrap());
    }

    #[test]
    fn test_bin() {
        assert_eq!("0b11111111", BIN.call(&Token::dummy(""), &Value::Integer(255)).unwrap());
        assert_eq!("0b11111111", BIN.call(&Token::dummy(""), &Value::Float(255.1)).unwrap());
    }

    #[test]
    fn test_oct() {
        assert_eq!("0o10", OCT.call(&Token::dummy(""), &Value::Integer(8)).unwrap());
        assert_eq!("0o10", OCT.call(&Token::dummy(""), &Value::Float(8.1)).unwrap());
    }

    #[test]
    fn test_sci() {
        assert_eq!("8e0", SCI.call(&Token::dummy(""), &Value::Integer(8)).unwrap());
        assert_eq!("-8.1e1", SCI.call(&Token::dummy(""), &Value::Float(-81.0)).unwrap());
        assert_eq!("8.1e-2", SCI.call(&Token::dummy(""), &Value::Float(0.081)).unwrap());
    }

    #[test]
    fn test_float() {
        assert_eq!("8.0", FLOAT.call(&Token::dummy(""), &Value::Integer(8)).unwrap());
        assert_eq!("81.0", FLOAT.call(&Token::dummy(""), &Value::Float(81.0)).unwrap());
        assert_eq!("0.0", FLOAT.call(&Token::dummy(""), &Value::Float(0.0000000001)).unwrap());
        assert_eq!("0.081", FLOAT.call(&Token::dummy(""), &Value::Float(0.081)).unwrap());
    }

    #[test]
    fn test_int() {
        assert_eq!("-8", INT.call(&Token::dummy(""), &Value::Integer(-8)).unwrap());
        assert_eq!("81", INT.call(&Token::dummy(""), &Value::Float(81.0)).unwrap());
        assert_eq!("0", INT.call(&Token::dummy(""), &Value::Float(0.081)).unwrap());
    }

    #[test]
    fn test_bool() {
        assert_eq!("false", BOOL.call(&Token::dummy(""), &Value::Integer(0)).unwrap());
        assert_eq!("true", BOOL.call(&Token::dummy(""), &Value::Integer(81)).unwrap());
        assert_eq!("true", BOOL.call(&Token::dummy(""), &Value::Float(0.081)).unwrap());
    }

    #[test]
    fn test_dollars() {
        assert_eq!("¥100.00", YEN.call(&Token::dummy(""), &Value::Integer(100)).unwrap());
        assert_eq!("$1,000.00", DOLLAR.call(&Token::dummy(""), &Value::Integer(1000)).unwrap());
        assert_eq!("€10,000.00", EURO.call(&Token::dummy(""), &Value::Integer(10000)).unwrap());
        assert_eq!("£100,000.00", POUND.call(&Token::dummy(""), &Value::Integer(100000)).unwrap());
        assert_eq!("£1,000,000.00", POUND.call(&Token::dummy(""), &Value::Integer(1000000)).unwrap());
    }

    #[test]
    fn test_utc() {
        assert_eq!("2022-03-20 14:05:33", UTC.call(&Token::dummy(""), &Value::Integer(1647785133)).unwrap());
    }

    #[test]
    fn test_ordinal() {
        assert_eq!("32nd", ORDINAL.call(&Token::dummy(""), &Value::Integer(32)).unwrap());
    }

    #[test]
    fn test_percentage() {
        assert_eq!("32.5%", PERCENTAGE.call(&Token::dummy(""), &Value::Float(0.325)).unwrap());
    }

    #[test]
    fn test_roman() {
        assert_eq!("XXVI", ROMAN.call(&Token::dummy(""), &Value::Integer(26)).unwrap());
    }
}