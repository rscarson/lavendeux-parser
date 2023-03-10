use super::value::Value;
use std::collections::HashMap;

use super::functions;
use super::decorators;

use super::network::ApiInstance;

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

    /// Available configured APIs
    pub apis: HashMap<String, ApiInstance>,

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
        ParserState {
            depth: 0,
            variables: HashMap::new(),

            constants: HashMap::from([
                ("pi".to_string(), Value::Float(std::f64::consts::PI)),
                ("e".to_string(), Value::Float(std::f64::consts::E)),
                ("tau".to_string(), Value::Float(std::f64::consts::TAU)),
            ]),

            functions: functions::FunctionTable::new(),
            user_functions: HashMap::new(),
            decorators: decorators::DecoratorTable::new(),

            apis: HashMap::from([
                ("animechan".to_string(), ApiInstance::new_with_description(
                    "https://animechan.vercel.app/api/random".to_string(), 
                    "Get a random quote from an anime or a character".to_string(),
                    "api('animechan'), api('animechan', 'character?name=naruto'), api('animechan', 'anime?title=[...]')".to_string(), 
                )),

                ("bible".to_string(), ApiInstance::new_with_description(
                    "https://bible-api.com".to_string(), 
                    "Get a bible quote".to_string(), 
                    "api('bible', 'Mark 14:52')".to_string()
                )),

                ("profanity".to_string(), ApiInstance::new_with_description(
                    "https://www.purgomalum.com/service/plain?text=".to_string(), 
                    "Profanity filter. Add text to censor".to_string(), 
                    "api('profanity', 'Fuckity Bye')".to_string()
                )),

                ("dictionary".to_string(), ApiInstance::new_with_description(
                    "https://api.dictionaryapi.dev/api/v2/entries".to_string(), 
                    "Dictionary API - return a definition for a word. Use language/word, such as en/fart ".to_string(), 
                    "api('dictionary', 'en/fart')".to_string()
                )),

                ("ipify".to_string(), ApiInstance::new_with_description(
                    "https://api.ipify.org/?format=plain".to_string(), 
                    "Returns your own IP address. No endpoint needed".to_string(), 
                    "api('ipify')".to_string()
                )),

                ("uselessfacts".to_string(), ApiInstance::new_with_description(
                    "https://uselessfacts.jsph.pl/api/v2/facts/random".to_string(), 
                    "Get a random factoid. No endpoint needed".to_string(), 
                    "api('uselessfacts')".to_string()
                )),
            ]),

            #[cfg(feature = "extensions")]
            extensions: extensions::ExtensionTable::new(),
        }
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