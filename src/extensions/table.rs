use crate::{Error, Token, Value};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::extensions::extension::Extension;
use crate::extensions::runtime::ExtensionsRuntime;

/// Holds a set of registered extensions
#[derive(Deserialize, Serialize, Clone)]
pub struct ExtensionTable(HashMap<String, Extension>);
impl ExtensionTable {
    /// Create a new empty table
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Add an extension
    ///
    /// # Arguments
    /// * `filename` - File name
    /// * `extension` - Extension to add
    pub fn add(&mut self, filename: &str, extension: Extension) {
        self.0.insert(filename.to_string(), extension);
    }

    /// Load an extension from a filename
    ///
    /// # Arguments
    /// * `filename` - File name
    pub fn load(&mut self, filename: &str) -> Result<Extension, rustyscript::Error> {
        let e = ExtensionsRuntime::load_extension(filename)?;
        self.0.insert(filename.to_string(), e.clone());
        Ok(e)
    }

    /// Attempt to load all extensions in a directory
    pub fn load_all(&mut self, path: &str) -> Vec<Result<Extension, rustyscript::Error>> {
        let e = ExtensionsRuntime::load_extensions(path);
        self.0.clear();
        for extension in e.iter().flatten() {
            self.0
                .insert(extension.filename().to_string(), extension.clone());
        }
        e
    }

    /// Delete an extension
    pub fn remove(&mut self, filename: &str) {
        self.0.remove(filename);
    }

    /// Returns the full list of extensions available
    pub fn all(&mut self) -> Vec<&mut Extension> {
        let mut a = Vec::from_iter(self.0.values_mut());
        a.sort_by(|f1, f2| f1.name().cmp(f2.name()));
        a
    }

    /// Determine if a function exists in the extension
    ///
    /// # Arguments
    /// * `name` - Function name
    pub fn has_function(&mut self, name: &str) -> bool {
        for extension in self.all() {
            if extension.has_function(name) {
                return true;
            }
        }
        false
    }

    /// Try to call a function in the loaded extensions
    pub fn call_function(
        &mut self,
        name: &str,
        token: &Token,
        args: &[Value],
        variables: &mut HashMap<String, Value>,
    ) -> Result<Value, Error> {
        for extension in self.all() {
            if extension.has_function(name) {
                return match extension.call_function(name, args, variables) {
                    Ok(value) => Ok(value),
                    Err(e) => Err(Error::Javascript(e, token.clone())),
                };
            }
        }
        Err(Error::FunctionName {
            name: name.to_string(),
            token: token.clone(),
        })
    }

    /// Determine if a decorator exists in the extension
    ///
    /// # Arguments
    /// * `name` - Decorator name
    pub fn has_decorator(&mut self, name: &str) -> bool {
        for extension in self.all() {
            if extension.has_decorator(name) {
                return true;
            }
        }
        false
    }

    /// Try to call a decorator in the loaded extensions
    pub fn call_decorator(
        &mut self,
        name: &str,
        token: &Token,
        variables: &mut HashMap<String, Value>,
    ) -> Result<String, Error> {
        for extension in self.all() {
            if extension.has_decorator(name) {
                return match extension.call_decorator(name, token, variables) {
                    Ok(value) => Ok(value),
                    Err(e) => Err(Error::Javascript(e, token.clone())),
                };
            }
        }
        Err(Error::DecoratorName {
            name: format!("@{}", name),
            token: token.clone(),
        })
    }
}
impl Default for ExtensionTable {
    fn default() -> Self {
        Self::new()
    }
}
