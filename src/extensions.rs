use crate::{Value, errors::*, Token};

use js_sandbox::{Script, JsError};
use serde::{Deserialize, Serialize};
use core::time::Duration;
use std::error::Error;
use std::collections::HashMap;
use std::fs;

const SCRIPT_TIMEOUT : u64 = 1000;

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
    pub fn load(&mut self, filename: &str) -> Result<Extension, std::io::Error> {
        let e = Extension::new(filename)?;
        self.0.insert(filename.to_string(), e.clone());
        Ok(e)
    }

    /// Attempt to load all extensions in a directory
    pub fn load_all(&mut self, path: &str) -> Vec<Result<Extension, Box<dyn Error>>> {
        let e = Extension::load_all(path);
        self.0.clear();
        for extension in e.iter().flatten() {
            self.0.insert(extension.filename().to_string(), extension.clone());
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
        a.sort_by(|f1, f2|f1.name().cmp(f2.name()));
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
    pub fn call_function(&mut self, name: &str, token: &Token, args: &[Value], variables: &mut HashMap<String, Value>) -> Result<Value, ParserError> {
        for extension in self.all() {
            if extension.has_function(name) {
                return extension.call_function(name, token, args, variables);
            }
        }
        Err(FunctionNameError::new(token, name).into())
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
    pub fn call_decorator(&mut self, name: &str, token: &Token, variables: &mut HashMap<String, Value>) -> Result<String, ParserError> {
        for extension in self.all() {
            if extension.has_decorator(name) {
                return extension.call_decorator(name, token, variables);
            }
        }
        Err(FunctionNameError::new(token, &format!("@{}", name)).into())
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
                    Err(e) => Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData, 
                        e.to_string().replace("sandboxed.js", filename)
                    ))
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
    pub fn load_all(directory: &str) -> Vec<Result<Extension, Box<dyn Error>>> {
        let mut extensions : Vec<Result<Extension, Box<dyn Error>>> = Vec::new();

        match fs::read_dir(directory) {
            Ok(entries) => {
                for file in entries.flatten() {
                    if let Some(filename) = file.path().to_str() {
                        if !filename.ends_with("js") { continue; }
                        match Extension::new(filename) {
                            Ok(extension) => extensions.push(Ok(extension)),
                            Err(e) => {extensions.push(Err(Box::new(e)))}
                        }
                    }
                }
            },
            Err(e) => {
                extensions.push(Err(Box::new(e)));
            }
        }

        extensions
    }

    /// Determine if a function exists in the extension
    /// 
    /// # Arguments
    /// * `name` - Function name
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// Load the script from string
    pub fn load_script(&self) -> Result<Script, JsError> {
        match Script::from_string(&self.contents) {
            Ok(s) => Ok(s),
            Err(e) => Err(e)
        }
    }

    /// Call a function from the extension
    /// 
    /// # Arguments
    /// * `name` - Function name
    /// * `args` - Values to pass in
    pub fn call_function(&mut self, name: &str, token: &Token, args: &[Value], variables: &mut HashMap<String, Value>) -> Result<Value, ParserError> {
        match self.load_script() {
            Ok(mut script) => {
                // Inject parser state
                self.call_js_function(&mut script, "setState", token, (&variables,))?;
        
                // Call function
                let e: ParserError = FunctionNameError::new(token, name).into();
                let fname = self.functions.get(name).ok_or(e)?;
                let result: Value = self.call_js_function(&mut script, fname, token, (&args.to_vec(),))?;
        
                // Pull out modified state
                let state : HashMap<String, Value> = self.call_js_function(&mut script, "getState", token, (&variables,))?;
                variables.clear();
                for k in state.keys() {
                    variables.insert(k.to_string(), state.get(k).unwrap().clone());
                }
        
                Ok(result)
            },
            Err(e) => Err(ScriptError::from_jserror(token, self.filename(), e).into())
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
    pub fn call_decorator(&mut self, name: &str, token: &Token, variables: &mut HashMap<String, Value>) -> Result<String, ParserError> {
        match self.load_script() {
            Ok(mut script) => {
                // Inject parser state
                self.call_js_function(&mut script, "setState", token, (&variables,))?;
        
                // Call decorator
                let e: ParserError = DecoratorNameError::new(token, name).into();
                let fname = self.decorators.get(name).ok_or(e)?;
                let result: String = self.call_js_function(&mut script, fname, token, (token.value(),))?;
        
                // Pull out modified state
                let state : HashMap<String, Value> = self.call_js_function(&mut script, "getState", token, (&variables,))?;
                variables.clear();
                for k in state.keys() {
                    variables.insert(k.to_string(), state.get(k).unwrap().clone());
                }
        
                Ok(result)
            },
            Err(e) => Err(ScriptError::from_jserror(token, self.filename(), e).into())
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
        let mut a : Vec<String> = self.functions.keys().cloned().collect();
        a.sort();
        a
    }

    /// Return the list of all decorators in the extension
    pub fn decorators(&self) -> Vec<String> {
        let mut a : Vec<String> = self.decorators.keys().cloned().collect();
        a.sort();
        a
    }

    fn call_js_function<A,T>(&self, script: &mut Script, function: &str, token: &Token, args: A) -> Result<T, ParserError>
    where T: serde::de::DeserializeOwned, A: js_sandbox::CallArgs {
        match script.call::<A, T>(function, args) {
            Ok(r) => Ok(r),
            Err(e) => Err(ScriptError::from_jserror(token, &format!("{}:{}", self.filename(), function), e).into())
        }
    }
} 

/// Load a script from a string
/// 
/// # Arguments
/// * `code` - JS source as string
fn script_from_string(filename: &str, code: &str) -> Result<Extension, JsError> {
    match Script::from_string(code) {
        Ok(script) => {
            let mut e : Extension = script.with_timeout(Duration::from_millis(SCRIPT_TIMEOUT))
                .call("extension", ())?;
            e.contents = code.to_string();
            e.filename = filename.to_string();

            // Append state information
            e.contents = format!("{}\n\n{}",
                "
                    let lavendeux_state = {}; 
                    const setState = (s) => {lavendeux_state = s}; 
                    const getState = () => lavendeux_state;",
                e.contents
            );

            Ok(e)
        },
        Err(e) => Err(e)
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
    fn test_call_function() {
        let mut e = Extension::new("example_extensions/colour_utils.js").unwrap();
        assert_eq!(Value::Integer(0x00FFFF), e.call_function("complement", &Token::dummy(""), &[Value::Integer(0xFFAA00)], &mut HashMap::new()).unwrap());
        assert_eq!(Value::Integer(0xFFF), e.call_function("color", &Token::dummy(""), &[Value::String("white".to_string())], &mut HashMap::new()).unwrap());
    }
    
    #[test]
    fn test_maintains_state() {
        let mut e = Extension::new("example_extensions/stateful_functions.js").unwrap();
        let mut state: HashMap<String, Value> = HashMap::new();
        state.insert("foo".to_string(), Value::String("bar".to_string()));
        assert_eq!(Value::Integer(0xFFAA00), e.call_function("set", &Token::dummy(""), &[Value::String("test".to_string()), Value::Integer(0xFFAA00)], &mut state).unwrap());
        assert_eq!(true, state.contains_key("test") && state.get("test").unwrap().as_int().unwrap() == 0xFFAA00);
    }
    
    #[test]
    fn test_can_fail() {
        let mut e = Extension::new("example_extensions/colour_utils.js").unwrap();
        assert_eq!(true, matches!(e.call_function("complement", &Token::dummy(""), &[], &mut HashMap::new()), Err(_)));
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
        assert_eq!("#ff0000", e.call_decorator("color", &token, &mut state).unwrap());
    }
    
    #[test]
    fn test_load_all() {
        let e = Extension::load_all("example_extensions");
        assert_eq!(true, e.len() > 0);
    }
    
    #[test]
    fn test_color() {
        let mut e = Extension::new("example_extensions/colour_utils.js").unwrap();
        assert_eq!(Value::Integer(0x00FFFF), e.call_function("complement", &Token::dummy(""), &[Value::Integer(0xFFAA00)], &mut HashMap::new()).unwrap());
    }
}