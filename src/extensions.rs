use super::value::AtomicValue;
use super::errors::*;
use js_sandbox::{Script, AnyError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Extension {
    #[serde(default)]
    pub contents: String,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub author: String,

    #[serde(default)]
    pub version: String,
    
    #[serde(default)]
    pub functions: HashMap<String, String>,
    
    #[serde(default)]
    pub decorators: HashMap<String, String>
}

unsafe impl Send for Extension {}
impl Extension {
    /// Load an extension from a filename
    /// 
    /// # Arguments
    /// * `name` - Function name
    pub fn new(filename: &str) -> Result<Extension, std::io::Error> {
        match fs::read_to_string(filename) {
            Ok(s) => {
                match script_from_string(&s) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
                }
            },
            Err(e) => Err(e)
        }
    }

    /// Convert the extension to a string
    pub fn to_string(&self) -> String {
        format!("{} v{}, by {}", self.name, self.version, self.author)
    }

    /// Attempt to load all extensions in a directory
    pub fn load_all(directory: &str) -> Result<Vec<Extension>, std::io::Error> {
        let mut extensions : Vec<Extension> = Vec::new();

        match fs::read_dir(directory) {
            Ok(entries) => {
                for file in entries {
                    if let Ok(f) = file {
                        if let Some(filename) = f.path().to_str() {
                            if let Ok(extension) = Extension::new(filename) {
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
    pub fn call_function(&mut self, name: &str, args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
        match self.load_script() {
            Ok(mut script) => {
                let fname = self.functions.get(name).ok_or(ParserError::FunctionName(FunctionNameError::new(name)))?;
                let result : Result<AtomicValue, AnyError> = script.call(fname, &args.to_vec());
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
    pub fn call_decorator(&mut self, name: &str, arg: &AtomicValue) -> Result<String, ParserError> {
        match self.load_script() {
            Ok(mut script) => {
                let fname = self.decorators.get(name).ok_or(ParserError::DecoratorName(DecoratorNameError::new(name)))?;
                let result : Result<String, AnyError> = script.call(fname, &arg);
                match result {
                    Ok(v) => Ok(v.clone()),
                    Err(e) => Err(ParserError::Script(ScriptError::new(&e.to_string())))
                }
            },
            Err(e) => Err(e)
        }
    }
}

/// Load a script from a string
/// 
/// # Arguments
/// * `code` - JS source as string
fn script_from_string(code: &str) -> Result<Extension, AnyError> {
    let mut script = Script::from_string(code)?;
    let mut e : Extension = script.call("extension", &())?;
    e.contents = code.to_string();
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
        assert_eq!(AtomicValue::Integer(0x00FFFF), e.call_function("complement", &[AtomicValue::Integer(0xFFAA00)]).unwrap());
        assert_eq!(AtomicValue::Integer(0xFFF), e.call_function("color", &[AtomicValue::String("white".to_string())]).unwrap());
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
        assert_eq!("#ff0000", e.call_decorator("color", &AtomicValue::Integer(0xFF)).unwrap());
    }
    
    #[test]
    fn test_load_all() {
        let e = Extension::load_all("example_extensions").unwrap();
        assert_eq!(true, e.len() > 0);
    }
    
    #[test]
    fn test_color() {
        let mut e = Extension::new("example_extensions/colour_utils.js").unwrap();
        assert_eq!(AtomicValue::Integer(0x00FFFF), e.call_function("complement", &[AtomicValue::Integer(0xFFAA00)]).unwrap());
    }
}