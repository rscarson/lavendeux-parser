use serde::{Deserialize, Serialize};

const MAX_FLOAT_PRECISION: i32 = 8;

/// The datatype for integer values
pub type IntegerType = i64;

/// The datatype for floating point values
pub type FloatType = f64;

/// Represents a single value resulting from a calculation
/// Can take the form of an integer, float, boolean or string
/// 
/// Some types are interchangeable:
/// ```rust
/// use lavendeux_parser::Value;
/// assert_eq!(Value::Boolean(true), Value::Integer(2).as_bool());
/// assert_eq!(Value::String("5.0".to_string()), Value::Float(5.0).as_string());
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub enum Value {
    /// The lack of a value
    None, 
    
    /// A boolean value - all types can be expressed as booleans
    Boolean(bool), 
    
    /// An integer value - floats can also be expressed as integers
    Integer(IntegerType), 
    
    /// A floating point value - integers can also be expressed as floats
    Float(FloatType), 
    
    /// A string value - all types can be expressed as booleans
    String(String)
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl Value {
    /// Return the value as a string
    pub fn as_string(&self) -> String {
        match self {
            Value::Boolean(v) => (if *v {"true"} else {"false"}).to_string(),
            Value::Integer(n) => {format!("{}", *n)},
            Value::Float(n) => {
                let multiplier = f64::powi(10.0, MAX_FLOAT_PRECISION);
                let mut v = (*n * multiplier).round() / multiplier;

                if v == -0.0 { v = 0.0; }
                let mut f = format!("{:.}", v);
                if !f.contains('.') {
                    f += ".0";
                }
                
                f
            },
            Value::String(s) => s.to_string(),
            Value::None => "".to_string(),
        }
    }
    
    /// Return the value as a boolean
    pub fn as_bool(&self) -> bool {
        match self {
            Value::None => false,
            Value::Boolean(v) => *v,
            Value::Integer(n) => *n != 0,
            Value::Float(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
        }
    }
    
    /// Return the value as an integer, if possible
    pub fn as_int(&self) -> Option<IntegerType> {
        match self {
            Value::None => None,
            Value::Boolean(v) => Some(*v as IntegerType),
            Value::Integer(n) => Some(*n),
            Value::Float(n) => Some(*n as IntegerType),
            Value::String(_) => None,
        }
    }
    
    /// Return the value as a float, if possible
    pub fn as_float(&self) -> Option<FloatType> {
        match self {
            Value::None => None,
            Value::Boolean(v) => Some((*v as IntegerType) as FloatType),
            Value::Integer(n) => Some(*n as FloatType),
            Value::Float(n) => Some(*n),
            Value::String(_) => None,
        }
    }

    /// Determine if the value is a boolean
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    /// Determine if the value is an int
    pub fn is_int(&self) -> bool {
        matches!(self, Value::Integer(_))
    }

    /// Determine if the value is a float
    pub fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
    }

    /// Determine if the value is a float or int
    pub fn is_numeric(&self) -> bool {
        self.is_float() || self.is_int()
    }

    /// Determine if the value is a string
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }
}

impl Clone for Value {
    fn clone(&self) -> Value {
        match self {
            Value::None => Value::None,
            Value::Boolean(v) => Value::Boolean(*v),
            Value::Integer(n) => Value::Integer(*n),
            Value::Float(n) => Value::Float(*n),
            Value::String(s) => Value::String(s.to_string()),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, _other: &Self) -> bool {
        match self {
            Value::None => matches!(self, _other),
            Value::Boolean(v) => matches!(self, _other) && *v == _other.as_bool(),
            Value::Integer(n) => matches!(self, _other) && *n == _other.as_int().unwrap(),
            Value::Float(n) => matches!(self, _other) && *n == _other.as_float().unwrap(),
            Value::String(s) => matches!(self, _other) && *s == _other.as_string(),
        }
    }
}

impl PartialEq<bool> for Value {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == *other
    }
}

impl PartialEq<IntegerType> for Value {
    fn eq(&self, other: &IntegerType) -> bool {
        if let Some(n) = self.as_int() {
            n == *other
        } else {
            false
        }
    }
}

impl PartialEq<FloatType> for Value {
    fn eq(&self, other: &FloatType) -> bool {
        if let Some(n) = self.as_float() {
            n == *other
        } else {
            false
        }
    }
}

impl PartialEq<String> for Value {
    fn eq(&self, other: &String) -> bool {
        self.as_string() == *other
    }
}

impl PartialEq<&str> for Value {
    fn eq(&self, other: &&str) -> bool {
        self.as_string() == *other.to_string()
    }
}

impl Eq for Value {}

#[cfg(test)]
mod test_atomic_value {
    use super::*;

    #[test]
    fn test_as_string() {
        assert_eq!("5", Value::Integer(5).as_string());
        assert_eq!("5.0", Value::Float(5.0).as_string());
        assert_eq!("5.1", Value::Float(5.1).as_string());
        assert_eq!("test", Value::String("test".to_string()).as_string());
        assert_eq!("", Value::None.as_string());
    }
    
    #[test]
    fn test_as_bool() {
        assert_eq!(true, Value::Float(5.0).as_bool());
        assert_eq!(true, Value::Integer(5).as_bool());
        assert_eq!(true, Value::String("5.0".to_string()).as_bool());
    }
    
    #[test]
    fn test_as_int() {
        assert_eq!(true, Value::Float(5.0).as_int().is_some());
        assert_eq!(5, Value::Float(5.0).as_int().unwrap());

        assert_eq!(true, Value::Integer(5).as_int().is_some());
        assert_eq!(5, Value::Integer(5).as_int().unwrap());

        assert_eq!(false, Value::String("".to_string()).as_int().is_some());
    }
    
    #[test]
    fn test_as_float() {
        assert_eq!(true, Value::Float(5.0).as_float().is_some());
        assert_eq!(5.0, Value::Float(5.0).as_float().unwrap());

        assert_eq!(true, Value::Integer(5).as_float().is_some());
        assert_eq!(5.0, Value::Integer(5).as_float().unwrap());

        assert_eq!(false, Value::String("".to_string()).as_float().is_some());
    }
    
    #[test]
    fn test_is_float() {
        assert_eq!(true, Value::Float(5.0).is_float());
        assert_eq!(false, Value::Integer(5).is_float());
    }
    
    #[test]
    fn test_is_string() {
        assert_eq!(true, Value::String("5.0".to_string()).is_string());
        assert_eq!(false, Value::Integer(5).is_string());
    }
    
    #[test]
    fn test_eq() {
        assert_eq!(false, Value::Float(5.0) == Value::Float(5.1));
        assert_eq!(true, Value::Float(5.0) == Value::Float(5.0));
        assert_eq!(true, Value::Integer(5) == Value::Integer(5));
        assert_eq!(false, Value::Integer(6) == Value::Integer(5));
        assert_eq!(true, Value::None == Value::None);
        assert_eq!(true, Value::String("test".to_string()) == Value::String("test".to_string()));
        assert_eq!(false, Value::String("test".to_string()) == Value::String("test2".to_string()));
    }
}