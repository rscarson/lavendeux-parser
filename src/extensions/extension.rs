use crate::{Token, Value};

use js_playground::Module;
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
    /// Defines all the functions provided by this extension
    pub function_definitions: Option<HashMap<String, ExtensionFunction>>,

    #[serde(default)]
    /// Defines all the decorators provided by this extension
    pub decorator_definitions: Option<HashMap<String, ExtensionFunction>>,

    #[serde(default)]
    /// Legacy extension support
    pub functions: Option<HashMap<String, String>>,

    #[serde(default)]
    /// Legacy extension support
    pub decorators: Option<HashMap<String, String>>,
}

impl std::fmt::Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} v{}, by {}", self.name, self.version, self.author)
    }
}

impl Extension {
    /// Create a new extension object by loading it from a JS module
    pub fn new(path: &str) -> Result<Self, js_playground::Error> {
        ExtensionsRuntime::load_extension(path)
    }

    /// Determine if a function exists in the extension
    ///
    /// # Arguments
    /// * `name` - Function name
    pub fn has_function(&self, name: &str) -> bool {
        if let Some(functions) = &self.function_definitions {
            functions.contains_key(name)
        } else if let Some(functions) = &self.functions {
            // Legacy function support
            functions.contains_key(name)
        } else {
            false
        }
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
    ) -> Result<Value, js_playground::Error> {
        if let Some(functions) = &self.function_definitions {
            let function_properties = functions
                .get(name)
                .ok_or(js_playground::Error::ValueNotFound(name.to_string()))?;
            function_properties.call(&self.module, args, variables)
        } else if let Some(functions) = &self.functions {
            // Legacy function support
            let function_name = functions
                .get(name)
                .ok_or(js_playground::Error::ValueNotFound(name.to_string()))?;
            ExtensionFunction::call_legacy(function_name, &self.module, args)
        } else {
            Err(js_playground::Error::JsonDecode(
                "invalid extension definition".to_string(),
            ))
        }
    }

    /// Determine if a decorator exists in the extension
    ///
    /// # Arguments
    /// * `name` - Decorator name
    pub fn has_decorator(&self, name: &str) -> bool {
        if let Some(decorators) = &self.function_definitions {
            decorators.contains_key(name)
        } else if let Some(decorators) = &self.decorators {
            // Legacy function support
            decorators.contains_key(name)
        } else {
            false
        }
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
    ) -> Result<String, js_playground::Error> {
        if let Some(decorator) = &self.decorator_definitions {
            let function_properties = decorator
                .get(name)
                .ok_or(js_playground::Error::ValueNotFound(name.to_string()))?;
            Ok(function_properties
                .call(&self.module, &[token.value()], variables)?
                .to_string())
        } else if let Some(decorator) = &self.decorators {
            // Legacy function support
            let function_name = decorator
                .get(name)
                .ok_or(js_playground::Error::ValueNotFound(name.to_string()))?;
            ExtensionFunction::call_legacy_decorator(function_name, &self.module, token.value())
        } else {
            Err(js_playground::Error::JsonDecode(
                "invalid extension definition".to_string(),
            ))
        }
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
        let mut function_keys = if let Some(functions) = &self.function_definitions {
            functions.keys().cloned().collect()
        } else if let Some(functions) = &self.functions {
            functions.keys().cloned().collect()
        } else {
            vec![]
        };

        function_keys.sort();
        function_keys
    }

    /// Return the list of all functions, with complete signatures
    pub fn function_signatures(&self) -> Vec<String> {
        let mut function_keys = if let Some(functions) = &self.function_definitions {
            functions.values().map(|k| k.signature()).collect()
        } else if let Some(functions) = &self.functions {
            functions.keys().map(|k| format!("{}()", k)).collect()
        } else {
            vec![]
        };

        function_keys.sort();
        function_keys
    }

    /// Return the list of all decorators in the extension
    pub fn decorators(&self) -> Vec<String> {
        let mut decorator_keys = if let Some(decorators) = &self.decorator_definitions {
            decorators.keys().cloned().collect()
        } else if let Some(decorators) = &self.decorators {
            decorators.keys().cloned().collect()
        } else {
            vec![]
        };

        decorator_keys.sort();
        decorator_keys
    }

    /// Return the list of all decorators, with complete signatures
    pub fn decorator_signatures(&self) -> Vec<String> {
        let mut decorator_keys = if let Some(decorators) = &self.decorator_definitions {
            decorators.values().map(|k| k.signature()).collect()
        } else if let Some(decorators) = &self.decorators {
            decorators.keys().map(|k| format!("@{}", k)).collect()
        } else {
            vec![]
        };

        decorator_keys.sort();
        decorator_keys
    }
}

#[cfg(test)]
mod test_extensions {
    use super::*;

    #[test]
    fn test_new() {
        let e = Extension::new("example_extensions/colour_utils.js").unwrap();
        assert_eq!("HTML Colour Utilities", e.name);
    }

    #[test]
    fn test_to_string() {
        let e = Extension::new("example_extensions/colour_utils.js").unwrap();
        assert_eq!("HTML Colour Utilities v0.2.0, by @rscarson", e.to_string());
    }

    #[test]
    fn test_has_function() {
        let e = Extension::new("example_extensions/colour_utils.js").unwrap();
        assert_eq!(true, e.has_function("complement"));
        assert_eq!(false, e.has_function("foobar"));
    }

    #[test]
    fn test_call_simple() {
        let mut e = Extension::new("example_extensions/simple.js").unwrap();
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
        let mut e = Extension::new("example_extensions/colour_utils.js").unwrap();
        assert_eq!(
            Value::Integer(0x00FFFF),
            e.call_function(
                "complement",
                &[Value::Integer(0xFFAA00)],
                &mut HashMap::new()
            )
            .unwrap()
        );
        assert_eq!(
            Value::Integer(0xFFF),
            e.call_function(
                "color",
                &[Value::String("white".to_string())],
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
        let mut e = Extension::new("example_extensions/colour_utils.js").unwrap();
        assert_eq!(
            true,
            matches!(
                e.call_function("complement", &[], &mut HashMap::new()),
                Err(_)
            )
        );
    }

    #[test]
    fn test_has_decorator() {
        let e = Extension::new("example_extensions/colour_utils.js").unwrap();
        assert_eq!(true, e.has_decorator("color"));
        assert_eq!(false, e.has_decorator("foobar"));
    }

    #[test]
    fn test_call_decorator() {
        let mut e = Extension::new("example_extensions/colour_utils.js").unwrap();
        let mut state: HashMap<String, Value> = HashMap::new();
        let mut token = Token::dummy("");
        token.set_value(Value::Integer(0xFF));
        assert_eq!(
            "#ff0000",
            e.call_decorator("color", &token, &mut state).unwrap()
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
        let mut e = Extension::new("example_extensions/colour_utils.js").unwrap();
        assert_eq!(
            Value::Integer(0x00FFFF),
            e.call_function(
                "complement",
                &[Value::Integer(0xFFAA00)],
                &mut HashMap::new()
            )
            .unwrap()
        );
    }
}
