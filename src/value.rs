use serde::{Deserialize, Serialize};
pub type IntegerType = i64;
pub type FloatType = f64;

#[derive(Debug, Serialize, Deserialize)]
pub enum AtomicValue {
    None, Boolean(bool), Integer(IntegerType), Float(FloatType), String(String)
}

impl std::fmt::Display for AtomicValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl AtomicValue {
    /// Return the value as an integer, if possible
    pub fn as_string(&self) -> String {
        match self {
            AtomicValue::Boolean(v) => return format!("{}", if *v {"true"} else {"false"}),
            AtomicValue::Integer(n) => return format!("{}", *n),
            AtomicValue::Float(n) => return format!("{}", *n),
            AtomicValue::String(s) => return s.to_string(),
            AtomicValue::None => return "".to_string(),
        }
    }
    
    pub fn as_bool(&self) -> bool {
        match self {
            AtomicValue::None => return false,
            AtomicValue::Boolean(v) => return *v,
            AtomicValue::Integer(n) => return *n != 0,
            AtomicValue::Float(n) => return *n != 0.0,
            AtomicValue::String(s) => return s.len() > 0,
        }
    }
    
    pub fn as_int(&self) -> Option<IntegerType> {
        match self {
            AtomicValue::None => return None,
            AtomicValue::Boolean(v) => return Some(*v as IntegerType),
            AtomicValue::Integer(n) => return Some(*n),
            AtomicValue::Float(n) => return Some(*n as IntegerType),
            AtomicValue::String(_) => return None,
        }
    }
    
    /// Return the value as a float, if possible
    pub fn as_float(&self) -> Option<FloatType> {
        match self {
            AtomicValue::None => return None,
            AtomicValue::Boolean(v) => return Some((*v as IntegerType) as FloatType),
            AtomicValue::Integer(n) => return Some(*n as FloatType),
            AtomicValue::Float(n) => return Some(*n),
            AtomicValue::String(_) => return None,
        }
    }

    /// Determine if the value is a float
    pub fn is_float(&self) -> bool {
        return matches!(self, AtomicValue::Float(_));
    }

    /// Determine if the value is a string
    pub fn is_string(&self) -> bool {
        return matches!(self, AtomicValue::String(_));
    }
}

impl Clone for AtomicValue {
    fn clone(&self) -> AtomicValue {
        match self {
            AtomicValue::None => AtomicValue::None,
            AtomicValue::Boolean(v) => AtomicValue::Boolean(*v),
            AtomicValue::Integer(n) => AtomicValue::Integer(*n),
            AtomicValue::Float(n) => AtomicValue::Float(*n),
            AtomicValue::String(s) => AtomicValue::String(s.to_string()),
        }
    }
}

impl PartialEq for AtomicValue {
    fn eq(&self, _other: &Self) -> bool {
        match self {
            AtomicValue::None => matches!(self, _other),
            AtomicValue::Boolean(v) => matches!(self, _other) && *v == _other.as_bool(),
            AtomicValue::Integer(n) => matches!(self, _other) && *n == _other.as_int().unwrap(),
            AtomicValue::Float(n) => matches!(self, _other) && *n == _other.as_float().unwrap(),
            AtomicValue::String(s) => matches!(self, _other) && *s == _other.as_string(),
        }
    }
}
impl Eq for AtomicValue {}

#[cfg(test)]
mod test_atomic_value {
    use super::*;

    #[test]
    fn test_as_string() {
        assert_eq!("5", AtomicValue::Integer(5).as_string());
        assert_eq!("5.1", AtomicValue::Float(5.1).as_string());
        assert_eq!("test", AtomicValue::String("test".to_string()).as_string());
        assert_eq!("", AtomicValue::None.as_string());
    }
    
    #[test]
    fn test_as_bool() {
        assert_eq!(true, AtomicValue::Float(5.0).as_bool());
        assert_eq!(true, AtomicValue::Integer(5).as_bool());
        assert_eq!(true, AtomicValue::String("5.0".to_string()).as_bool());
    }
    
    #[test]
    fn test_as_int() {
        assert_eq!(true, AtomicValue::Float(5.0).as_int().is_some());
        assert_eq!(5, AtomicValue::Float(5.0).as_int().unwrap());

        assert_eq!(true, AtomicValue::Integer(5).as_int().is_some());
        assert_eq!(5, AtomicValue::Integer(5).as_int().unwrap());

        assert_eq!(false, AtomicValue::String("".to_string()).as_int().is_some());
    }
    
    #[test]
    fn test_as_float() {
        assert_eq!(true, AtomicValue::Float(5.0).as_float().is_some());
        assert_eq!(5.0, AtomicValue::Float(5.0).as_float().unwrap());

        assert_eq!(true, AtomicValue::Integer(5).as_float().is_some());
        assert_eq!(5.0, AtomicValue::Integer(5).as_float().unwrap());

        assert_eq!(false, AtomicValue::String("".to_string()).as_float().is_some());
    }
    
    #[test]
    fn test_is_float() {
        assert_eq!(true, AtomicValue::Float(5.0).is_float());
        assert_eq!(false, AtomicValue::Integer(5).is_float());
    }
    
    #[test]
    fn test_is_string() {
        assert_eq!(true, AtomicValue::String("5.0".to_string()).is_string());
        assert_eq!(false, AtomicValue::Integer(5).is_string());
    }
    
    #[test]
    fn test_eq() {
        assert_eq!(false, AtomicValue::Float(5.0) == AtomicValue::Float(5.1));
        assert_eq!(true, AtomicValue::Float(5.0) == AtomicValue::Float(5.0));
        assert_eq!(true, AtomicValue::Integer(5) == AtomicValue::Integer(5));
        assert_eq!(false, AtomicValue::Integer(6) == AtomicValue::Integer(5));
        assert_eq!(true, AtomicValue::None == AtomicValue::None);
        assert_eq!(true, AtomicValue::String("test".to_string()) == AtomicValue::String("test".to_string()));
        assert_eq!(false, AtomicValue::String("test".to_string()) == AtomicValue::String("test2".to_string()));
    }
}