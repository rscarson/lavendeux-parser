use crate::value::{Value};
use crate::errors::*;

use std::ops::Index;
use std::collections::HashMap;
use core::slice::Iter;

/// Describes an argument for a callable function
#[derive(Clone)]
pub struct FunctionArgument{ name: String, expected: ExpectedTypes, optional: bool, plural: bool }
impl FunctionArgument {
    /// Build a new function argument
    pub fn new(name: &str, expected: ExpectedTypes, optional: bool) -> Self {
        Self {name: name.to_string(), expected, optional, plural: false}
    }
    
    /// Build a new plural function argument
    pub fn new_plural(name: &str, expected: ExpectedTypes, optional: bool) -> Self {
        Self {name: name.to_string(), expected, optional, plural: true}
    }

    /// Build a new required function argument
    pub fn new_required(name: &str, expected: ExpectedTypes) -> Self {
        Self::new(name, expected, false)
    }

    /// Build a new optional function argument
    pub fn new_optional(name: &str, expected: ExpectedTypes) -> Self {
        Self::new(name, expected, true)
    }

    /// Return the argument's name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the argument's expected type
    pub fn expected(&self) -> &ExpectedTypes {
        &self.expected
    }

    /// Return wether or not the argument is optional
    pub fn optional(&self) -> bool {
        self.optional
    }

    /// Return wether or not the argument is plural
    pub fn plural(&self) -> bool {
        self.plural
    }

    /// Returns a boolean result indicating if the supplied value is valid for this argument
    pub fn validate_value(&self, value: &Value) -> bool {
        match self.expected() {
            ExpectedTypes::Float => value.is_float(),
            ExpectedTypes::Int => value.is_int(),
            ExpectedTypes::IntOrFloat => value.is_float() || value.is_int(),
            
            // These can be converted from any type
            ExpectedTypes::String => true, 
            ExpectedTypes::Boolean => true, 
            ExpectedTypes::Array => true, 
            ExpectedTypes::Any => true
        }
    }
}
impl std::fmt::Display for FunctionArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = if self.plural {
            format!("{}1, {}2", self.name, self.name)
        } else {self.name().to_string()};
        write!(f, "{}{}{}", 
            if self.optional {"["} else {""},
            name,
            if self.optional {"]"} else {""},
        )
    }
}

/// A value returned by an argument
pub struct FunctionArgumentValue(Vec<Value>);
impl FunctionArgumentValue {
    /// Create a new argument value wrapper
    /// 
    /// # Arguments
    /// * `values` - Value array
    pub fn new(values: Vec<Value>) -> Self {
        Self(values)
    }

    /// Return the value as a required argument
    pub fn required(&self) -> Value {
        self.0.first().cloned().unwrap()
    }
    
    /// Return the value as an optional argument
    pub fn optional(&self) -> Option<Value> {
        self.0.first().cloned()
    }
    
    /// Return the value as an argument or a default value
    pub fn optional_or(&self, default: Value) -> Value {
        self.0.first().cloned().unwrap_or(default)
    }

    /// Return the value as a plural argument
    pub fn plural(&self) -> Vec<Value> {
        self.0.clone()
    }
}

/// Represents a collection of function arguments
pub struct FunctionArgumentCollection {
    values: Vec<Value>,
    map: HashMap<String, Vec<Value>>,

    next_index: usize
}

impl FunctionArgumentCollection {
    /// Return a new empty collection
    pub fn new() -> Self {
        Self{
            values: Vec::<Value>::new(),
            map: HashMap::new(),
            next_index: 0
        }
    }

    /// Add a new value to the table
    /// 
    /// # Arguments
    /// * `name` - Function argument key
    /// * `value` - Function value
    pub fn add(&mut self, name: String, value: Value) {
        match self.map.get_mut(&name) {
            Some(v) => {
                v.push(value.clone());
            },
            None => {
                self.map.insert(name.clone(), vec![value.clone()]);
            }
        }
        
        self.values.push(value.clone());
    }

    /// Get a value from the table
    /// 
    /// # Arguments
    /// * `name` - Function argument key
    pub fn get(&self, name: &str) -> FunctionArgumentValue {
        FunctionArgumentValue::new(match self.map.get(name).cloned() {
            Some(v) => v,
            None => Vec::new()
        })
    }

    /// Return the full array of registerd values
    pub fn values(&self) -> &Vec<Value> {
        &self.values
    }

    /// Return an iterator over the values
    pub fn iter(&self) -> Iter<Value> {
        self.values.iter()
    }

    /// Return the number of registered values
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Return true if there were no given arguments
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl Index<usize> for FunctionArgumentCollection {
    type Output = Value;
    fn index(&self, i: usize) -> &Value {
        &self.values[i]
    }
}

impl Iterator for FunctionArgumentCollection {
    type Item = Value;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    fn next(&mut self) -> Option<Self::Item> {
        if self.values().is_empty() || self.next_index == self.values().len() {
            None
        } else {
            self.next_index += 1;
            Some(self[self.next_index - 1].clone())
        }
    }
}