use super::value::{Value};
use super::errors::*;
use std::collections::HashMap;

pub type FunctionHandler = fn(&FunctionDefinition, &[Value]) -> Result<Value, ParserError>;

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

/// Holds the definition of a builtin callable function
#[derive(Clone)]
pub struct FunctionDefinition {
    /// Function call name
    pub name: &'static str,
    
    /// Function short description
    pub description: &'static str,

    /// Vector of arguments the function supports
    pub arguments: fn() -> Vec<FunctionArgument>,

    /// Handler function
    pub handler: FunctionHandler
}
impl FunctionDefinition {
    /// Return the function's name
    pub fn name(&self) -> &str {
        self.name
    }
    
    /// Return the function's description
    pub fn description(&self) -> &str {
        self.description
    }

    /// Return the function's arguments
    pub fn args(&self) -> Vec<FunctionArgument> {
        (self.arguments)()
    }
    
    /// Return the function's signature
    pub fn signature(&self) -> String {
        format!("{}({})", self.name, self.args().iter().map(|e| e.to_string()).collect::<Vec<String>>().join(", "))
    }
    
    /// Return the function's help string
    pub fn help(&self) -> String {
        format!("{}: {}", self.signature(), self.description())
    }

    /// Validate function arguments, and return an error if one exists
    /// 
    /// # Arguments
    /// * `args` - Function arguments
    pub fn validate(&self, args: &[Value]) -> Option<ParserError> {
        let optional_arguments = self.args().iter().filter(|e| e.optional).count();
        let plural_arguments = self.args().iter().filter(|e| e.plural).count();
        let max_arguments = self.args().len();
        let min_arguments = max_arguments - optional_arguments;

        // Argument count
        if args.len() < min_arguments || (plural_arguments == 0 && args.len() > max_arguments) {
            return Some(ParserError::FunctionNArg(FunctionNArgError::new(&self.signature(), min_arguments, max_arguments)))
        }
        
        // Argument types
        for (i, arg) in args.iter().enumerate() {
            let argument = &self.args()[i];
            let valid = match argument.expected {
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
                return Some(ParserError::FunctionArgType(FunctionArgTypeError::new(&self.signature(), i+1, argument.expected.clone())));
            }
        }

        None
    }

    // Call the associated function handler
    /// 
    /// # Arguments
    /// * `args` - Function arguments
    pub fn call(&self, args: &[Value]) -> Result<Value, ParserError> {
        if let Some(error) = self.validate(args) {
            Err(error)
        } else {
            (self.handler)(self, args)
        }
    }
}

mod math;
mod dev;
mod network;
mod array;
mod str;
mod trig;

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
        math::register_functions(self);
        str::register_functions(self);
        dev::register_functions(self);
        network::register_functions(self);
        trig::register_functions(self);
        array::register_functions(self);
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
        self.0.values().collect()
    }

    /// Call a function
    /// 
    /// # Arguments
    /// * `name` - Function name
    /// * `args` - Function arguments
    pub fn call(&self, name: &str, args: &[Value]) -> Result<Value, ParserError> {
        match self.0.get(name) {
            Some(f) => f.call(args),
            None => Err(ParserError::FunctionName(FunctionNameError::new(name)))
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

#[cfg(test)]
mod test_builtin_table {
    use super::*;

    const EXAMPLE : FunctionDefinition = FunctionDefinition {
        name: "example",
        description: "Sample function",
        arguments: || vec![
            FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
        ],
        handler: |_definition, _args: &[Value]| {
            Ok(Value::Integer(4))
        }
    };
    
    #[test]
    fn test_register() {
        let mut table = FunctionTable::new();
        table.register(EXAMPLE);
        assert_eq!(true, table.has("example"));
    }
    
    #[test]
    fn test_has() {
        let mut table = FunctionTable::new();
        table.register(EXAMPLE);
        assert_eq!(true, table.has("example"));
    }
    
    #[test]
    fn test_call() {
        let mut table = FunctionTable::new();
        table.register(EXAMPLE);

        table.call("example", &[]).unwrap_err();
        table.call("example", &[Value::String("".to_string())]).unwrap_err();
        table.call("example", &[Value::Integer(4)]).unwrap();
    }
}