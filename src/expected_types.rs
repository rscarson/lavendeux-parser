use std::fmt;

use crate::Value;

/// Represents a type of value that was expected
#[derive(Debug, Copy, Clone)]
pub enum ExpectedTypes {
    /// Integer value
    Int,

    /// Floating point value
    Float,

    /// Any numeric value
    IntOrFloat,

    /// String value
    String,

    /// Boolean value
    Boolean,

    /// Array value
    Array,

    /// Object value
    Object,

    /// Any type of value
    Any,
}

impl ExpectedTypes {
    /// Returns true if the given value matches expectations
    pub fn matches(&self, value: &Value) -> bool {
        if value.is_compound() {
            true
        } else {
            self.strict_matches(value)
        }
    }

    /// Returns true if the given value matches expectations and count
    pub fn strict_matches(&self, value: &Value) -> bool {
        match self {
            ExpectedTypes::Int => value.is_int(),
            ExpectedTypes::Float => value.is_float(),
            ExpectedTypes::IntOrFloat => value.is_numeric(),

            // Can be converted from any type
            _ => true,
        }
    }
}

impl fmt::Display for ExpectedTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpectedTypes::Int => write!(f, "integer"),
            ExpectedTypes::Float => write!(f, "float"),
            ExpectedTypes::IntOrFloat => write!(f, "integer or float"),
            ExpectedTypes::String => write!(f, "string"),
            ExpectedTypes::Boolean => write!(f, "boolean"),
            ExpectedTypes::Array => write!(f, "array"),
            ExpectedTypes::Object => write!(f, "object"),
            ExpectedTypes::Any => write!(f, "any"),
        }
    }
}
