use super::value::Value;
use std::collections::HashMap;

use super::functions;
use super::decorators;

#[cfg(feature = "extensions")]
use super::extensions;

const MAX_STACK_DEPTH: usize = 50;

/// Holds the properties of a function assigned inside an expression
#[derive(Clone)]
pub struct UserFunction {
    name: String,
    arguments: Vec<String>,
    definition: String
}
impl UserFunction {
    /// Return a new user function
    /// 
    /// # Arguments
    /// * `name` - Function name
    /// * `arguments` - Arguments expected by the function
    /// * `definition` - Function definition string
    pub fn new(name: String, arguments: Vec<String>, definition: String) -> Self {
        Self {
            name, arguments, definition
        }
    }

    /// Return the function's name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Return the function's expected arguments
    pub fn arguments(&self) -> &Vec<String> {
        &self.arguments
    }
    
    /// Return the function's definition string
    pub fn definition(&self) -> &str {
        &self.definition
    }

    /// Return the function's signature
    pub fn signature(&self) -> String {
        format!("{}({}) = {}", self.name(), self.arguments().join(", "), self.definition())
    }
}

/// Represents the current state of the parser
/// Holds the functions, decorators, variables and extensions
/// available for expressions to use
#[derive(Clone)]
pub struct ParserState {
    depth : usize,

    /// The assigned variables usable in expressions
    pub variables: HashMap<String, Value>,

    /// Constant values usable in expressions
    pub constants: HashMap<String, Value>,

    /// Functions that can be called by expressions
    pub functions: functions::FunctionTable,

    /// Functions assigned from within, and callable by, expressions
    pub user_functions: HashMap<String, UserFunction>,

    /// Decorators that can be called by expressions
    pub decorators: decorators::DecoratorTable,

    /// Currently loaded extensions
    #[cfg(feature = "extensions")]
    pub extensions: extensions::ExtensionTable,
}

impl Default for ParserState {
    fn default() -> Self {
        Self::new()
    }
} 

impl ParserState {
    /// Create a new parser state
    pub fn new() -> ParserState {
        let mut state = ParserState {
            depth: 0,
            variables: HashMap::new(),
            constants: HashMap::new(),

            functions: functions::FunctionTable::new(),
            user_functions: HashMap::new(),
            decorators: decorators::DecoratorTable::new(),

            #[cfg(feature = "extensions")]
            extensions: extensions::ExtensionTable::new(),
        };

        // Set up constants
        state.constants.insert("pi".to_string(), Value::Float(std::f64::consts::PI));
        state.constants.insert("e".to_string(), Value::Float(std::f64::consts::E));
        state.constants.insert("tau".to_string(), Value::Float(std::f64::consts::TAU));

        state
    }

    /// Returns a new parser with the same properties, and the depth incremented
    /// Fails if the maximum depth is overshot
    pub fn spawn_inner(&self) -> Option<ParserState> {
        let mut s = self.clone();
        s.depth = self.depth + 1;
        if s.depth < MAX_STACK_DEPTH {
            Some(s)
        } else {
            None
        }
    }

    /// Returns the parser's current depth
    pub fn depth(&self) -> usize {
        self.depth
    }
}