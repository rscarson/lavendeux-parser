use super::value::Value;
use super::errors::*;
use js_sandbox::{Script, AnyError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

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
    pub fn load(&mut self, filename: &str) -> Result<Extension, ParserError> {
        let e = Extension::new(filename)?;
        self.0.insert(filename.to_string(), e.clone());
        Ok(e)
    }

    /// Attempt to load all extensions in a directory
    pub fn load_all(&mut self, path: &str) -> Result<Vec<Extension>, ParserError> {
        let e = Extension::load_all(path)?;
        for extension in &e {
            self.0.insert(extension.filename().to_string(), extension.clone());
        }
        Ok(e)
    }

    /// Delete an extension
    pub fn remove(&mut self, filename: &str) {
        self.0.remove(filename);
    }

    /// Returns the full list of extensions available
    pub fn all(&self) -> Vec<Extension> {
        Vec::from_iter(self.0.values().cloned())
    }

    /// Determine if a function exists in the extension
    /// 
    /// # Arguments
    /// * `name` - Function name
    pub fn has_function(&self, name: &str) -> bool {
        for extension in self.all() {
            if extension.has_function(name) {
                return true;
            }
        }
        false
    }

    /// Try to call a function in the loaded extensions
    pub fn call_function(&self, name: &str, args: &[Value]) -> Result<Value, ParserError> {
        for mut extension in self.all() {
            if extension.has_function(name) {
                return extension.call_function(name, args);
            }
        }
        Err(ParserError::FunctionName(FunctionNameError::new(name)))
    }

    /// Determine if a decorator exists in the extension
    /// 
    /// # Arguments
    /// * `name` - Decorator name
    pub fn has_decorator(&self, name: &str) -> bool {
        for extension in self.all() {
            if extension.has_decorator(name) {
                return true;
            }
        }
        false
    }

    /// Try to call a decorator in the loaded extensions
    pub fn call_decorator(&self, name: &str, arg: &Value) -> Result<String, ParserError> {
        for mut extension in self.all() {
            if extension.has_decorator(name) {
                return extension.call_decorator(name, arg);
            }
        }
        Err(ParserError::FunctionName(FunctionNameError::new(&format!("@{}", name))))
    }
}
impl Default for ExtensionTable {
    fn default() -> Self {
        Self::new()
    }
}

fn default_name() -> String { "Unnamed Extension".to_string() }
fn default_author() -> String { "Anonymous".to_string() }
fn default_version() -> String { "0.0.0".to_string() }

/// Represents a single loaded extension. It describes the functions and decorators it adds,
/// as well as metadata about the extension and it's author.
/// 
/// Add this to a ParserState to use it in expressions, or call the extension directly with
/// call_function / call_decorator
#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct Extension {
    #[serde(default)]
    filename: String,

    #[serde(default = "default_name")]
    name: String,

    #[serde(default = "default_author")]
    author: String,

    #[serde(default = "default_version")]
    version: String,
    
    #[serde(default)]
    contents: String,
    
    #[serde(default)]
    functions: HashMap<String, String>,
    
    #[serde(default)]
    decorators: HashMap<String, String>
}

impl std::fmt::Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} v{}, by {}", self.name, self.version, self.author)
    }
}

unsafe impl Send for Extension {}
impl Extension {
    /// Load an extension from a filename
    /// 
    /// # Arguments
    /// * `filename` - Source filename
    pub fn new(filename: &str) -> Result<Extension, std::io::Error> {
        match fs::read_to_string(filename) {
            Ok(s) => {
                match script_from_string(filename, &s) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
                }
            },
            Err(e) => Err(e)
        }
    }

    /// Create a new dummy extension that cannot be called or used
    /// 
    /// # Arguments
    /// * `name` - Extension name
    /// * `author` - Extension author
    /// * `author` - Extension author
    /// * `version` - Extension version
    /// * `functions` - Extension functions
    /// * `decorators` - Extension decorators
    pub fn new_stub(name: Option<&str>, author: Option<&str>, version: Option<&str>, functions: Vec<String>, decorators: Vec<String>) -> Self {
        let mut stub = Self {
            name: name.unwrap_or(&default_name()).to_string(),
            author: author.unwrap_or(&default_author()).to_string(),
            version: version.unwrap_or(&default_version()).to_string(),
            contents: "".to_string(),
            filename: "".to_string(),
            functions: HashMap::new(),
            decorators: HashMap::new()
        };

        for f in functions { stub.functions.insert(f.clone(), f); }
        for d in decorators { stub.decorators.insert(d.clone(), d); }

        stub
    }

    /// Attempt to load all extensions in a directory
    pub fn load_all(directory: &str) -> Result<Vec<Extension>, std::io::Error> {
        let mut extensions : Vec<Extension> = Vec::new();

        match fs::read_dir(directory) {
            Ok(entries) => {
                for file in entries.flatten() {
                    if let Some(filename) = file.path().to_str() {
                        if let Ok(extension) = Extension::new(filename) {
                            if filename.ends_with("js") {
                                extensions.push(extension);
                            }
                        }
                    }
                }
            },
            Err(e) => {
                return Err(e);
            }
        }

        Ok(extensions)
    }

    /// Determine if a function exists in the extension
    /// 
    /// # Arguments
    /// * `name` - Function name
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// Load the script from string
    pub fn load_script(&mut self) -> Result<Script, ParserError> {
        match Script::from_string(&self.contents) {
            Ok(s) => Ok(s),
            Err(e) => Err(ParserError::Script(ScriptError::new(&e.to_string())))
        }
    }

    /// Call a function from the extension
    /// 
    /// # Arguments
    /// * `name` - Function name
    /// * `args` - Values to pass in
    pub fn call_function(&mut self, name: &str, args: &[Value]) -> Result<Value, ParserError> {
        match self.load_script() {
            Ok(mut script) => {
                let fname = self.functions.get(name).ok_or_else(|| ParserError::FunctionName(FunctionNameError::new(name)))?;
                let result : Result<Value, AnyError> = script.call(fname, &args.to_vec());
                match result {
                    Ok(v) => Ok(v),
                    Err(e) => Err(ParserError::Script(ScriptError::new(&e.to_string())))
                }
            },
            Err(e) => Err(e)
        }
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
    pub fn call_decorator(&mut self, name: &str, arg: &Value) -> Result<String, ParserError> {
        match self.load_script() {
            Ok(mut script) => {
                let fname = self.decorators.get(name).ok_or_else(|| ParserError::DecoratorName(DecoratorNameError::new(name)))?;
                let result : Result<String, AnyError> = script.call(fname, &arg);
                match result {
                    Ok(v) => Ok(v),
                    Err(e) => Err(ParserError::Script(ScriptError::new(&e.to_string())))
                }
            },
            Err(e) => Err(e)
        }
    }

    /// Returns the file from which an extension was loaded
    pub fn filename(&self) -> &str {
        &self.filename
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
        self.functions.keys().cloned().collect()
    }

    /// Return the list of all decorators in the extension
    pub fn decorators(&self) -> Vec<String> {
        self.decorators.keys().cloned().collect()
    }
} 

/// Load a script from a string
/// 
/// # Arguments
/// * `code` - JS source as string
fn script_from_string(filename: &str, code: &str) -> Result<Extension, AnyError> {
    let mut script = Script::from_string(code)?;
    let mut e : Extension = script.call("extension", &())?;
    e.contents = code.to_string();
    e.filename = filename.to_string();
    Ok(e)
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
    fn test_call_function() {
        let mut e = Extension::new("example_extensions/colour_utils.js").unwrap();
        assert_eq!(Value::Integer(0x00FFFF), e.call_function("complement", &[Value::Integer(0xFFAA00)]).unwrap());
        assert_eq!(Value::Integer(0xFFF), e.call_function("color", &[Value::String("white".to_string())]).unwrap());
    }
    
    #[test]
    fn test_can_fail() {
        let mut e = Extension::new("example_extensions/colour_utils.js").unwrap();
        assert_eq!(true, matches!(e.call_function("complement", &[]), Err(_)));
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
        assert_eq!("#ff0000", e.call_decorator("color", &Value::Integer(0xFF)).unwrap());
    }
    
    #[test]
    fn test_load_all() {
        let e = Extension::load_all("example_extensions").unwrap();
        assert_eq!(true, e.len() > 0);
    }
    
    #[test]
    fn test_color() {
        let mut e = Extension::new("example_extensions/colour_utils.js").unwrap();
        assert_eq!(Value::Integer(0x00FFFF), e.call_function("complement", &[Value::Integer(0xFFAA00)]).unwrap());
    }
}