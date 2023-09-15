use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::cmp::Ordering;

const MAX_FLOAT_PRECISION: i32 = 8;

/// The datatype for integer values
pub type IntegerType = i64;

/// The datatype for floating point values
pub type FloatType = f64;

/// The datatype for array values
pub type ArrayType = Vec<Value>;

/// The datatype for object values
pub type ObjectType = HashMap<Value, Value>;

/// Represents a single value resulting from a calculation
/// Can take the form of an integer, float, boolean or string
/// 
/// Some types are interchangeable:
/// ```rust
/// use lavendeux_parser::Value;
/// assert_eq!(Value::Boolean(true), Value::Integer(2).as_bool());
/// assert_eq!(Value::String("5.0".to_string()), Value::Float(5.0).as_string());
/// ```
#[derive(Debug)]
pub enum Value {
    /// The lack of a value
    None, 

    /// An unresolved identifier
    Identifier(String),
    
    /// A boolean value - all types can be expressed as booleans
    Boolean(bool), 
    
    /// An integer value - floats can also be expressed as integers
    Integer(IntegerType), 
    
    /// A floating point value - integers can also be expressed as floats
    Float(FloatType), 
    
    /// A string value - all types can be expressed as strings
    String(String),

    /// An array value
    Array(ArrayType),

    /// An object value
    Object(ObjectType),
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>, {
        
        #[derive(Deserialize)]
        enum IntermediateValue {
            /// The lack of a value
            None, 
            Identifier(String),
            Boolean(bool), 
            Integer(IntegerType), 
            Float(FloatType), 
            String(String),
            Array(ArrayType),
            Object(Vec<(Value, Value)>),
        }
        
        let _value = IntermediateValue::deserialize(deserializer)?;
        match _value {
            IntermediateValue::None => Ok(Value::None),
            IntermediateValue::Identifier(id) => Ok(Value::Identifier(id)),
            IntermediateValue::Boolean(b) => Ok(Value::Boolean(b)),
            IntermediateValue::Integer(i) => Ok(Value::Integer(i)),
            IntermediateValue::Float(f) => Ok(Value::Float(f)),
            IntermediateValue::String(s) => Ok(Value::String(s)),
            IntermediateValue::Array(a) => Ok(Value::Array(a)),
            IntermediateValue::Object(o) => {
                let m: ObjectType = o.into_iter().collect();
                Ok(Value::Object(m))
            }
        }
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer, {
        match self {
            Value::None => serializer.serialize_newtype_variant("Value", 0, "None", &()),
            Value::Identifier(id) => serializer.serialize_newtype_variant("Value", 1, "Identifier", id),
            Value::Boolean(b) => serializer.serialize_newtype_variant("Value", 2, "Boolean", b),
            Value::Integer(i) => serializer.serialize_newtype_variant("Value", 3, "Integer", i),
            Value::Float(f) => serializer.serialize_newtype_variant("Value", 4, "Float", f),
            Value::String(s) => serializer.serialize_newtype_variant("Value", 5, "String", s),
            Value::Array(a) => serializer.serialize_newtype_variant("Value", 6, "Array", a),
            Value::Object(o) => {
                let flat: Vec<(&Value, &Value)> = o.iter().map(|(item, idx)| (item, idx)).collect();
                serializer.serialize_newtype_variant("Value", 7, "Object", &flat)
            }
        }
    }
}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Value::None => (),
            Value::Identifier(id) => id.hash(state),
            Value::Boolean(b) => b.hash(state),
            Value::Integer(i) => i.hash(state),
            Value::Float(f) => f.to_bits().hash(state),
            Value::String(s) => s.hash(state),
            Value::Array(a) => a.hash(state),
            Value::Object(o) => {
                let mut v: Vec<(&Value, &Value)> = o.iter().collect();
                v.sort_by_key(|(k, _)| (*k).clone());
                v.hash(state);
            }
        }
    }
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
                let mut f = format!("{:}", v);
                if !f.contains('.') {
                    f += ".0";
                }
                
                f
            },
            Value::String(s) => s.to_string(),
            Value::Array(v) => format!("[{}]", v.iter().map(|e| e.as_string()).collect::<Vec<String>>().join(", ")),
            Value::Object(v) => format!("{{{}}}", v.keys()
                .map(|k| format!("{}:{}", 
                    if k.is_string() {format!("\"{}\"", k.as_string()
                        .replace('\'', "\\'")
                        .replace('\"', "\\\"")
                        .replace('\n', "\\n")
                        .replace('\r', "\\r")
                        .replace('\t', "\\t")
                    )} else {k.to_string()}, 
                    if v.get(k).unwrap().is_string() {format!("\"{}\"", v.get(k).unwrap().as_string()
                        .replace('\'', "\\'")
                        .replace('\"', "\\\"")
                        .replace('\n', "\\n")
                        .replace('\r', "\\r")
                        .replace('\t', "\\t")
                    )} else {v.get(k).unwrap().to_string()}))
                .collect::<Vec<String>>()
                .join(", ")
            ),
            Value::Identifier(s) => s.to_string(),
            Value::None => "".to_string(),
        }
    }
    
    /// Return the value as a boolean
    pub fn as_bool(&self) -> bool {
        match self {
            Value::None => false,
            Value::Identifier(_) => false,
            Value::Boolean(v) => *v,
            Value::Integer(n) => *n != 0,
            Value::Float(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(v) => v.iter().any(|e|e.as_bool()),
            Value::Object(v) => v.values().any(|e|e.as_bool())
        }
    }
    
    /// Return the value as an integer, if possible
    pub fn as_int(&self) -> Option<IntegerType> {
        match self {
            Value::None => None,
            Value::Identifier(_) => None,
            Value::Boolean(_) => None,
            Value::Integer(n) => Some(*n),
            Value::Float(n) => Some(*n as IntegerType),
            Value::String(_) => None,
            Value::Array(_) => None,
            Value::Object(_) => None,
        }
    }
    
    /// Return the value as a float, if possible
    pub fn as_float(&self) -> Option<FloatType> {
        match self {
            Value::None => None,
            Value::Identifier(_) => None,
            Value::Boolean(_) => None,
            Value::Integer(n) => Some(*n as FloatType),
            Value::Float(n) => Some(*n),
            Value::String(_) => None,
            Value::Array(_) => None,
            Value::Object(_) => None,
        }
    }
    
    /// Return the value as an array
    pub fn as_array(&self) -> ArrayType {
        match self {
            Value::None => vec![],
            Value::Identifier(_) => vec![],
            Value::Boolean(_) => vec![self.clone()],
            Value::Integer(_) => vec![self.clone()],
            Value::Float(_) => vec![self.clone()],
            Value::String(_) => vec![self.clone()],
            Value::Array(v) => v.clone(),
            Value::Object(v) => v.values().cloned().collect(),
        }
    }
    
    /// Return the value as an object
    pub fn as_object(&self) -> ObjectType {
        match self {
            Value::Object(v) => v.clone(),
            _ => self.as_array().iter().enumerate().map(|(i, v)| (Value::Integer(i as IntegerType), v.clone())).collect()
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

    /// Determine if the value is an array
    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    /// Determine if the value is an object
    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    /// Determine if the value is an array or object
    pub fn is_compound(&self) -> bool {
        self.is_object() || self.is_array()
    }

    /// Determine if the value is an identifier
    pub fn is_identifier(&self) -> bool {
        matches!(self, Value::Identifier(_))
    }

    /// Determine if the value is empty
    pub fn is_none(&self) -> bool {
        matches!(self, Value::None)
    }
}

impl Clone for Value {
    fn clone(&self) -> Value {
        match self {
            Value::None => Value::None,
            Value::Identifier(s) => Value::Identifier(s.to_string()),
            Value::Boolean(v) => Value::Boolean(*v),
            Value::Integer(n) => Value::Integer(*n),
            Value::Float(n) => Value::Float(*n),
            Value::String(s) => Value::String(s.to_string()),
            Value::Array(v) => Value::Array(v.clone()),
            Value::Object(v) => Value::Object(v.clone()),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            // Boolean comparisons - false < * < true
            (Value::Boolean(b1), Value::Boolean(b2)) => b1.partial_cmp(b2),
            (Value::Boolean(b1), _) => b1.partial_cmp(&other.as_bool()),
            (_, Value::Boolean(b2)) => self.as_bool().partial_cmp(b2),

            // For objects, compare sorted values
            (Value::Object(obj1), _) => {
                let mut v1: Vec<_> = obj1.values().collect(); v1.sort();
                let obj2 = other.as_object();
                let mut v2: Vec<_> = obj2.values().collect(); v2.sort();
                v1.partial_cmp(&v2)
            },
            (_, Value::Object(obj2)) => {
                let obj1 = self.as_object();
                let mut v1: Vec<_> = obj1.values().collect(); v1.sort();
                let mut v2: Vec<_> = obj2.values().collect(); v2.sort();
                v1.partial_cmp(&v2)
            },

            // Array comparisons
            (Value::Array(a1), _) => a1.partial_cmp(&other.as_array()),
            (_, Value::Array(a2)) => self.as_array().partial_cmp(a2),

            // Number to number
            (Value::Integer(i1), Value::Integer(i2)) => i1.partial_cmp(i2),
            (Value::Integer(i1), Value::Float(f2)) => (*i1 as f64).partial_cmp(f2),
            (Value::Float(f1), Value::Integer(i2)) => f1.partial_cmp(&(*i2 as f64)),
            (Value::Float(f1), Value::Float(f2)) => f1.partial_cmp(f2),

            // String comparisons, If one is a string, both are strings
            (Value::String(s1), _) => s1.partial_cmp(&other.as_string()),
            (_, Value::String(s2)) => self.as_string().partial_cmp(s2),
            (Value::Identifier(_), Value::Identifier(_)) => self.as_string().partial_cmp(&other.as_string()),

            // Treat identifiers and none as false
            (Value::Identifier(_), _) => Some(Ordering::Less),
            (_, Value::Identifier(_)) => Some(Ordering::Greater),
            (Value::None, Value::None) => Some(Ordering::Equal),
            (Value::None, _) => Some(Ordering::Less),
            (_, Value::None) => Some(Ordering::Greater),
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

impl PartialEq<ArrayType> for Value {
    fn eq(&self, other: &ArrayType) -> bool {
        self.as_array().len() == other.len() &&
        self.as_array().iter().zip(other.iter()).all(|(a,b)| a == b) 
    }
}

impl Eq for Value {}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<ArrayType> for Value {
    fn from(value: ArrayType) -> Self {
        Self::Array(value)
    }
}

impl From<ObjectType> for Value {
    fn from(value: ObjectType) -> Self {
        Self::Object(value)
    }
}

impl From<FloatType> for Value {
    fn from(value: FloatType) -> Self {
        Self::Float(value)
    }
}

impl From<IntegerType> for Value {
    fn from(value: IntegerType) -> Self {
        Self::Integer(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

#[cfg(test)]
mod test_atomic_value {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

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
    fn test_as_array() {
        assert_eq!(1, Value::Float(5.0).as_array().len());
        assert_eq!(2, Value::Array(vec![Value::Integer(5), Value::Integer(5)]).as_array().len());
    }
    
    #[test]
    fn test_hash() {
        let mut hasher = DefaultHasher::new();
        Value::String("1".to_string()).hash(&mut hasher);
        let hstring = hasher.finish();

        hasher = DefaultHasher::new();
        Value::Integer(1).hash(&mut hasher);
        let hint = hasher.finish();

        hasher = DefaultHasher::new();
        Value::Integer(2).hash(&mut hasher);
        let hint2 = hasher.finish();

        hasher = DefaultHasher::new();
        Value::Integer(2).hash(&mut hasher);
        let hint2b = hasher.finish();

        assert_eq!(false, hstring == hint);
        assert_eq!(false, hint2 == hint);
        assert_eq!(true, hint2 == hint2b);
    }
    
    #[test]
    fn test_object() {
        let object = Value::Object(HashMap::from([
            (Value::String("1".to_string()), Value::Integer(1)),
            (Value::Integer(1), Value::Integer(2)),
            (Value::Integer(2), Value::Integer(3)),
        ]));

        assert_eq!(Value::Integer(2), *object.as_object().get(&Value::Integer(1)).unwrap());
        assert_eq!(Value::Integer(1), *object.as_object().get(&Value::String("1".to_string())).unwrap());
        assert_eq!(Value::Integer(3), *object.as_object().get(&Value::Integer(2)).unwrap());
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
    fn test_is_array() {
        assert_eq!(true, Value::Array(vec![Value::Integer(5)]).is_array());
        assert_eq!(false, Value::Integer(5).is_array());
    }
    
    #[test]
    fn test_is_identifier() {
        assert_eq!(false, Value::Array(vec![Value::Integer(5)]).is_identifier());
        assert_eq!(false, Value::Integer(5).is_array());
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

    #[test]
    fn test_ord_bool() {
        // Boolean - Boolean
        assert!(Value::from(false) == Value::from(false));
        assert!(Value::from(false) != Value::from(true));
        assert!(Value::from(false) < Value::from(true));
        assert!(Value::from(true) > Value::from(false));

        // Boolean - Integer
        assert!(Value::from(false) == Value::from(0));
        assert!(Value::from(0) == Value::from(false));
        //
        assert!(Value::from(1) != Value::from(false));
        assert!(Value::from(false) != Value::from(1));
        //
        assert!(Value::from(false) < Value::from(1));
        assert!(Value::from(1) > Value::from(false));
        //
        assert!(Value::from(true) > Value::from(0));
        assert!(Value::from(0) < Value::from(true));

        // Boolean - Float
        assert!(Value::from(false) == Value::from(0.0));
        assert!(Value::from(0.0) == Value::from(false));
        //
        assert!(Value::from(false) != Value::from(1.0));
        assert!(Value::from(1.0) != Value::from(false));
        //
        assert!(Value::from(false) < Value::from(1.0));
        assert!(Value::from(1.0) > Value::from(false));
        //
        assert!(Value::from(true) > Value::from(0.0));
        assert!(Value::from(0.0) < Value::from(true));

        // Boolean - String
        assert!(Value::from(false) == Value::from(""));
        assert!(Value::from("") == Value::from(false));
        //
        assert!(Value::from(false) != Value::from("test"));
        assert!(Value::from("test") != Value::from(false));
        //
        assert!(Value::from(false) < Value::from("test"));
        assert!(Value::from("test") > Value::from(false));
        //
        assert!(Value::from(true) > Value::from(""));
        assert!(Value::from("") < Value::from(true));

        // Boolean - Array
        assert!(Value::from(false) == Value::from(vec![]));
        assert!(Value::from(vec![]) == Value::from(false));
        //
        assert!(Value::from(false) != Value::from(vec![ Value::from(1) ]));
        assert!(Value::from(vec![ Value::from(1) ]) != Value::from(false));
        //
        assert!(Value::from(false) < Value::from(vec![ Value::from(1) ]));
        assert!(Value::from(vec![ Value::from(1) ]) > Value::from(false));
        //
        assert!(Value::from(true) > Value::from(vec![]));
        assert!(Value::from(vec![]) < Value::from(true));

        // Boolean - Object
        assert!(Value::from(false) == Value::from(Value::from(vec![]).as_object()));
        assert!(Value::from(vec![]) == Value::from(false));
        //
        assert!(Value::from(false) != Value::from(Value::from(vec![ Value::from(1) ]).as_object()));
        assert!(Value::from(Value::from(vec![ Value::from(1) ]).as_object()) != Value::from(false));
        //
        assert!(Value::from(false) < Value::from(Value::from(vec![ Value::from(1) ]).as_object()));
        assert!(Value::from(Value::from(vec![ Value::from(1) ]).as_object()) > Value::from(false));
        //
        assert!(Value::from(true) > Value::from(Value::from(vec![]).as_object()));
        assert!(Value::from(vec![]) < Value::from(true));
    }

    #[test]
    fn test_ord_int() {
        // Integer - Integer
        assert!(Value::from(1) == Value::from(1));
        assert!(Value::from(0) == Value::from(0));
        //
        assert!(Value::from(1) != Value::from(0));
        assert!(Value::from(1) != Value::from(0));
        //
        assert!(Value::from(1) > Value::from(0));
        assert!(Value::from(0) < Value::from(1));

        // Integer - Float
        assert!(Value::from(1.0) == Value::from(1));
        assert!(Value::from(0) == Value::from(0.0));
        //
        assert!(Value::from(1) != Value::from(0.0));
        assert!(Value::from(1.0) != Value::from(0));
        //
        assert!(Value::from(1) > Value::from(0.0));
        assert!(Value::from(0.0) < Value::from(1));

        // Integer - String
        assert!(Value::from(1) == Value::from("1"));
        assert!(Value::from("0") == Value::from(0));
        //
        assert!(Value::from("1") != Value::from(0));
        assert!(Value::from(1) != Value::from("0.1"));
        //
        assert!(Value::from(1) > Value::from("0"));
        assert!(Value::from(0) < Value::from("1"));

        // Integer - Array
        assert!(Value::from(1) == Value::from(vec![ Value::from(1) ]));
        //
        assert!(Value::from(1) != Value::from(vec![]));
        assert!(Value::from(vec![]) != Value::from(1));
        //
        assert!(Value::from(1) > Value::from(vec![]));
        assert!(Value::from(vec![]) < Value::from(1));

        // Integer - Object
        assert!(Value::from(1) == Value::from(Value::from(vec![ Value::from(1) ]).as_object()));
        //
        assert!(Value::from(1) != Value::from(Value::from(vec![ ]).as_object()));
        assert!(Value::from(Value::from(vec![ ]).as_object()) != Value::from(1));
        //
        assert!(Value::from(1) > Value::from(Value::from(vec![]).as_object()));
        assert!(Value::from(Value::from(vec![]).as_object()) < Value::from(1));
    }

    #[test]
    fn test_ord_float() {
        // Float - Float
        assert!(Value::from(1.0) == Value::from(1.0));
        assert!(Value::from(0.0) == Value::from(0.0));
        //
        assert!(Value::from(1.0) != Value::from(0.0));
        assert!(Value::from(1.0) != Value::from(0.1));
        //
        assert!(Value::from(1.0) > Value::from(0.0));
        assert!(Value::from(0.0) < Value::from(1.0));

        // Float - String
        assert!(Value::from(1.0) == Value::from("1.0"));
        assert!(Value::from("0.0") == Value::from(0.0));
        //
        assert!(Value::from("1.0") != Value::from(0.0));
        assert!(Value::from(1.0) != Value::from("0.1"));
        //
        assert!(Value::from(1.0) > Value::from("0.0"));
        assert!(Value::from("0.0") < Value::from(1.0));

        // Float - Array
        assert!(Value::from(1.0) == Value::from(vec![ Value::from(1.0) ]));
        assert!(Value::from(vec![ Value::from(1.0) ]) == Value::from(1.0));
        //
        assert!(Value::from(1.0) != Value::from(vec![]));
        assert!(Value::from(vec![]) != Value::from(1.0));
        //
        assert!(Value::from(1.0) > Value::from(vec![]));
        assert!(Value::from(vec![]) < Value::from(1.0));

        // Float - Object
        assert!(Value::from(1.0) == Value::from(Value::from(vec![ Value::from(1.0) ]).as_object()));
        assert!(Value::from(Value::from(vec![ Value::from(1.0) ]).as_object()) == Value::from(1.0));
        //
        assert!(Value::from(1.0) != Value::from(Value::from(vec![ ]).as_object()));
        assert!(Value::from(Value::from(vec![ ]).as_object()) != Value::from(1.0));
        //
        assert!(Value::from(1.0) > Value::from(Value::from(vec![]).as_object()));
        assert!(Value::from(Value::from(vec![]).as_object()) < Value::from(1.0));
    }

    #[test]
    fn test_ord_string() {
        // String - String
        assert!(Value::from("test") == Value::from("test"));
        //
        assert!(Value::from("test") != Value::from(""));
        assert!(Value::from("") != Value::from("test"));
        //
        assert!(Value::from("test") > Value::from(""));
        assert!(Value::from("") < Value::from("test"));

        // String - Array
        assert!(Value::from("1") == Value::from(vec![ Value::from(1) ]));
        assert!(Value::from(vec![ Value::from(1) ]) == Value::from("1"));
        //
        assert!(Value::from("test") != Value::from(vec![]));
        assert!(Value::from(vec![]) != Value::from("test"));
        //
        assert!(Value::from("test") > Value::from(vec![]));
        assert!(Value::from(vec![]) < Value::from("test"));

        // String - Object
        assert!(Value::from("1") == Value::from(Value::from(vec![ Value::from(1) ]).as_object()));
        assert!(Value::from(Value::from(vec![ Value::from(1) ]).as_object()) == Value::from("1"));
        //
        assert!(Value::from("test") != Value::from(Value::from(vec![ ]).as_object()));
        assert!(Value::from(Value::from(vec![ ]).as_object()) != Value::from("test"));
        //
        assert!(Value::from("test") > Value::from(Value::from(vec![]).as_object()));
        assert!(Value::from(Value::from(vec![]).as_object()) < Value::from("test"));
    }

    #[test]
    fn test_ord_array() {
        // Array - Array
        assert!(Value::from(vec![ Value::from(1) ]) == Value::from(vec![ Value::from(1) ]));
        //
        assert!(Value::from(vec![ Value::from(1) ])  != Value::from(vec![]));
        assert!(Value::from(vec![]) != Value::from(vec![ Value::from(1) ]) );
        //
        assert!(Value::from(vec![ Value::from(1) ])  > Value::from(vec![]));
        assert!(Value::from(vec![]) < Value::from(vec![ Value::from(1) ]) );

        // Array - Object
        assert!(Value::from(vec![ Value::from(1) ]) == Value::from(Value::from(vec![ Value::from(1) ]).as_object()));
        assert!(Value::from(Value::from(vec![]).as_object()) == Value::from(vec![]));
        //
        assert!(Value::from(vec![ Value::from(1) ]) != Value::from(Value::from(vec![ ]).as_object()));
        assert!(Value::from(Value::from(vec![ ]).as_object()) != Value::from(vec![ Value::from(1) ]));
        //
        assert!(Value::from(vec![ Value::from(1) ]) > Value::from(Value::from(vec![]).as_object()));
        assert!(Value::from(Value::from(vec![]).as_object()) < Value::from(vec![ Value::from(1) ]));
    }
    
    #[test]
    fn test_ord_obj() {
        // Object - Object
        assert!(Value::from(Value::from(vec![ Value::from(1) ]).as_object()) == Value::from(Value::from(vec![ Value::from(1) ]).as_object()));
        //
        assert!(Value::from(Value::from(vec![ Value::from(1) ]).as_object()) != Value::from(Value::from(vec![]).as_object()));
        assert!(Value::from(Value::from(vec![]).as_object()) != Value::from(Value::from(vec![ Value::from(1) ]).as_object()));
        //
        assert!(Value::from(Value::from(vec![ Value::from(1) ]).as_object()) > Value::from(Value::from(vec![]).as_object()));
        assert!(Value::from(Value::from(vec![]).as_object()) < Value::from(Value::from(vec![ Value::from(1) ]).as_object()));
    }
}