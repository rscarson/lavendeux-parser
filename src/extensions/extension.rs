use crate::{Token, Value};

use rustyscript::Module;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{function::ExtensionFunction, runtime::ExtensionsRuntime};

fn default_name() -> String {
    "Unnamed Extension".to_string()
}
fn default_author() -> String {
    "Anonymous".to_string()
}
fn default_version() -> String {
    "0.0.0".to_string()
}

/// Represents a single loaded extension. It describes the functions and decorators it adds,
/// as well as metadata about the extension and it's author.
///
/// Add this to a ParserState to use it in expressions, or call the extension directly with
/// call_function / call_decorator
#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct Extension {
    #[serde(default)]
    /// Associated code / filename for the extension
    pub module: Module,

    #[serde(default = "default_name")]
    /// Name of this extension
    pub name: String,

    #[serde(default = "default_author")]
    /// Author of this extension
    pub author: String,

    #[serde(default = "default_version")]
    /// Version of the extension
    pub version: String,

    #[serde(default)]
    /// Functions supported by this extension
    pub functions: HashMap<String, ExtensionFunction>,

    #[serde(default)]
    /// Decorators supported by this extension
    pub decorators: HashMap<String, ExtensionFunction>,
}

impl std::fmt::Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} v{}, by {}", self.name, self.version, self.author)
    }
}

impl Extension {
    /// Create a new extension object by loading it from a JS module
    pub fn new(path: &str) -> Result<Self, rustyscript::Error> {
        ExtensionsRuntime::load_extension(path)
    }

    /// Determine if a function exists in the extension
    ///
    /// # Arguments
    /// * `name` - Function name
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// Call a function from the extension
    ///
    /// # Arguments
    /// * `name` - Function name
    /// * `args` - Values to pass in
    pub fn call_function(
        &mut self,
        name: &str,
        args: &[Value],
        variables: &mut HashMap<String, Value>,
    ) -> Result<Value, rustyscript::Error> {
        let function_properties = self
            .functions
            .get(name)
            .ok_or(rustyscript::Error::ValueNotFound(name.to_string()))?;
        function_properties.call(&self.module, args, variables)
    }

    /// Determine if a decorator exists in the extension
    ///
    /// # Arguments
    /// * `name` - Decorator name
    pub fn has_decorator(&self, name: &str) -> bool {
        self.decorators.contains_key(name)
    }

    /// Call a decorator from the extension
    ///
    /// # Arguments
    /// * `name` - Decorator name
    /// * `arg` - Value to pass in
    pub fn call_decorator(
        &mut self,
        name: &str,
        token: &Token,
        variables: &mut HashMap<String, Value>,
    ) -> Result<String, rustyscript::Error> {
        let function_properties = self
            .decorators
            .get(name)
            .ok_or(rustyscript::Error::ValueNotFound(name.to_string()))?;
        function_properties
            .call(&self.module, &[token.value()], variables)
            .and_then(|v| Ok(v.to_string()))
    }

    /// Returns the file from which an extension was loaded
    pub fn filename(&self) -> &str {
        self.module.filename()
    }

    /// Returns the name of the extension
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the name of the extension's author
    pub fn author(&self) -> &str {
        &self.author
    }

    /// Returns the version of the extension
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Return the list of all functions in the extension
    pub fn functions(&self) -> Vec<String> {
        let mut function_keys: Vec<String> = self.functions.keys().cloned().collect();
        function_keys.sort();
        function_keys
    }

    /// Return the list of all functions, with complete signatures
    pub fn function_signatures(&self) -> Vec<String> {
        let mut function_keys: Vec<String> = self
            .functions
            .values()
            .map(|k| k.function_signature())
            .collect();
        function_keys.sort();
        function_keys
    }

    /// Return the list of all decorators in the extension
    pub fn decorators(&self) -> Vec<String> {
        let mut decorator_keys: Vec<String> = self.decorators.keys().cloned().collect();
        decorator_keys.sort();
        decorator_keys
    }

    /// Return the list of all decorators, with complete signatures
    pub fn decorator_signatures(&self) -> Vec<String> {
        let mut decorator_keys: Vec<String> = self
            .decorators
            .values()
            .map(|k| k.decorator_signature())
            .collect();
        decorator_keys.sort();
        decorator_keys
    }
}

#[cfg(test)]
mod test_extensions {
    use super::*;

    #[test]
    fn test_new() {
        let e = Extension::new("example_extensions/simple_extension.js").unwrap();
        assert_eq!("simple_extension", e.name);
    }

    #[test]
    fn test_to_string() {
        let e = Extension::new("example_extensions/simple_extension.js").unwrap();
        assert_eq!("simple_extension v1.0.0, by @rscarson", e.to_string());
    }

    #[test]
    fn test_has_function() {
        let e = Extension::new("example_extensions/simple_extension.js").unwrap();
        assert_eq!(true, e.has_function("add"));
        assert_eq!(false, e.has_function("foobar"));
    }

    #[test]
    fn test_call_simple() {
        let mut e = Extension::new("example_extensions/simple_extension.js").unwrap();
        assert_eq!(
            Value::Float(3.0),
            e.call_function(
                "add",
                &[Value::Integer(1), Value::Integer(2)],
                &mut HashMap::new()
            )
            .unwrap()
        );
    }

    #[test]
    fn test_call_function() {
        let mut e = Extension::new("example_extensions/simple_extension.js").unwrap();
        assert_eq!(
            Value::Integer(3),
            e.call_function(
                "add",
                &[Value::Integer(1), Value::Integer(2)],
                &mut HashMap::new()
            )
            .unwrap()
        );
    }

    #[test]
    fn test_maintains_state() {
        let mut e = Extension::new("example_extensions/stateful_functions.js").unwrap();
        let mut state: HashMap<String, Value> = HashMap::new();
        state.insert("foo".to_string(), Value::String("bar".to_string()));
        assert_eq!(
            Value::Integer(0xFFAA00),
            e.call_function(
                "put",
                &[Value::String("test".to_string()), Value::Integer(0xFFAA00)],
                &mut state
            )
            .unwrap()
        );
        assert_eq!(Some(&Value::Integer(0xFFAA00)), state.get("test"));
    }

    #[test]
    fn test_can_fail() {
        let mut e = Extension::new("example_extensions/simple_extension.js").unwrap();
        assert_eq!(
            true,
            matches!(e.call_function("add", &[], &mut HashMap::new()), Err(_))
        );
    }

    #[test]
    fn test_has_decorator() {
        let e = Extension::new("example_extensions/simple_extension.js").unwrap();
        assert_eq!(true, e.has_decorator("colour"));
        assert_eq!(false, e.has_decorator("foobar"));
    }

    #[test]
    fn test_call_decorator() {
        let mut e = Extension::new("example_extensions/simple_extension.js").unwrap();
        let mut state: HashMap<String, Value> = HashMap::new();
        let mut token = Token::dummy("");
        token.set_value(Value::Integer(0xFF));
        assert_eq!(
            "#ff0000",
            e.call_decorator("colour", &token, &mut state).unwrap()
        );
    }
    /*
        #[test]
        fn test_load_all() {
            let mut table = ExtensionTable::new();
            let e = table.load_all("example_extensions");
            assert_eq!(true, e.len() > 0);
        }
    */
    #[test]
    fn test_color() {
        let mut e = Extension::new("example_extensions/simple_extension.js").unwrap();
        assert_eq!(
            Value::Integer(3),
            e.call_function(
                "add",
                &[Value::Integer(1), Value::Integer(2)],
                &mut HashMap::new()
            )
            .unwrap()
        );
    }
}
