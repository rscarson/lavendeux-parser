use crate::{ParserState, Value, Token};
use crate::errors::*;
use std::collections::HashMap;

use super::FunctionDefinition;
use super::builtins;

/// Holds a set of callable functions
#[derive(Clone)]
pub struct FunctionTable(HashMap<String, FunctionDefinition>);
impl FunctionTable {
    /// Initialize a new function table, complete with default builtin functions
    pub fn new() -> FunctionTable {
        let mut table : FunctionTable = FunctionTable(HashMap::new());
        table.register_builtins();
        table
    }

    /// Register builtin functions
    fn register_builtins(&mut self) {
        builtins::api::register_functions(self);
        builtins::array::register_functions(self);
        builtins::crypto::register_functions(self);
        builtins::dev::register_functions(self);
        builtins::math::register_functions(self);
        builtins::network::register_functions(self);
        builtins::system::register_functions(self);
        builtins::str::register_functions(self);
        builtins::trig::register_functions(self);
    }

    /// Register a function in the table
    /// 
    /// # Arguments
    /// * `name` - Function name
    /// * `handler` - Function handler
    pub fn register(&mut self, function: FunctionDefinition) {
        self.0.insert(function.name.to_string(), function);
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

    /// Return a given function
    /// 
    /// # Arguments
    /// * `name` - Function name
    pub fn get(&self, name: &str) -> Option<&FunctionDefinition> {
        self.0.get(name)
    }

    /// Get a collection of all included functions
    pub fn all(&self) -> Vec<&FunctionDefinition> {
        let mut a: Vec<&FunctionDefinition>  = self.0.values().collect();
        a.sort_by(|f1, f2|f1.name().cmp(f2.name()));
        a
    }

    /// Return all included function categories, sorted in alphabetical order
    pub fn all_categories(&self) -> Vec<&str> {
        let mut v: Vec<&str> = self.all().iter().map(|f| f.category()).collect();
        v.sort_unstable();
        v.dedup();
        v
    }

    /// Return all included functions sorted by category
    pub fn all_by_category(&self) -> HashMap<&str, Vec<&FunctionDefinition>> {
        let f: Vec<(&str, Vec<&FunctionDefinition>)> = self.all_categories().iter().map(
            |c| (*c, 
                self.all()
                .iter()
                .filter(|f| f.category() == *c)
                .copied()
                .collect::<Vec<&FunctionDefinition>>()
            )
        ).collect();
        let m: HashMap<_, _> = f.into_iter().collect();
        m
    }

    /// Call a function
    /// 
    /// # Arguments
    /// * `name` - Function name
    /// * `args` - Function arguments
    pub fn call(&self, name: &str, token: &Token, state: &mut ParserState, args: &[Value]) -> Result<Value, ParserError> {
        match self.0.get(name) {
            Some(f) => f.call(token, state, args),
            None => Err(FunctionNameError::new(token, name).into())
        }
    }

    /// Return a function's signature
    /// 
    /// # Arguments
    /// * `name` - Function name
    pub fn signature(&self, name: &str) -> Option<String> {
        self.0.get(name).map(|f| f.signature())
    }

    /// Return a function's description
    /// 
    /// # Arguments
    /// * `name` - Function name
    pub fn description(&self, name: &str) -> Option<String> {
        self.0.get(name).map(|f| f.description().to_string())
    }
}

impl Default for FunctionTable {
    fn default() -> Self {
        Self::new()
    }
}