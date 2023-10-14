use crate::{value::ObjectType, Error, ExpectedTypes, Token, Value};
use std::collections::HashMap;

#[macro_use]
pub mod decorator_macros {
    /// Defines a function for registration as a builtin
    #[macro_export]
    macro_rules! define_decorator {
        (
            name = $function_name:ident,
            $(aliases = [$($function_alias:literal),+],)?
            description = $function_desc:literal,
            input = $function_arg:expr,
            handler = $function_impl:expr
        ) => {
            /// Decorator definition for use with Lavendeux
            /// It should be registered with 'decorator_table.register()
            #[allow(non_upper_case_globals, unused_variables)]
            pub const $function_name: $crate::DecoratorDefinition = $crate::DecoratorDefinition {
                name: &[stringify!($function_name)$(, $($function_alias),+)?],
                description: $function_desc,
                argument: $function_arg,
                handler: $function_impl,
            };
        };
    }
}

mod currency;
mod numeric;
mod primitives;
mod string;

/// Handler for executing a decorator
pub type DecoratorHandler = fn(&DecoratorDefinition, &Token, &Value) -> Result<String, Error>;

/// Holds a set of callable decorators
#[derive(Clone)]
pub struct DecoratorTable(HashMap<String, DecoratorDefinition>);
impl DecoratorTable {
    /// Initialize a new decorator table, complete with default builtin decorators
    pub fn new() -> DecoratorTable {
        let mut table: DecoratorTable = DecoratorTable(HashMap::new());

        table.register(numeric::hex);
        table.register(numeric::oct);
        table.register(numeric::bin);
        table.register(numeric::sci);
        table.register(numeric::utc);

        table.register(currency::dollar);
        table.register(currency::euro);
        table.register(currency::pound);
        table.register(currency::yen);

        table.register(primitives::DEFAULT);
        table.register(primitives::FLOAT);
        table.register(primitives::INT);
        table.register(primitives::BOOL);
        table.register(primitives::ARRAY);
        table.register(primitives::OBJECT);

        table.register(string::ROMAN);
        table.register(string::ORDINAL);
        table.register(string::PERCENTAGE);

        table
    }

    /// Register a decorator in the table
    ///
    /// # Arguments
    /// * `name` - Decorator name
    /// * `handler` - Decorator handler
    pub fn register(&mut self, definition: DecoratorDefinition) {
        for name in definition.name() {
            self.0.insert(name.to_string(), definition.clone());
        }
    }

    /// Check if the table contains a decorator by the given name
    ///
    /// # Arguments
    /// * `name` - Decorator name
    pub fn has(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    /// Return a given decorator
    ///
    /// # Arguments
    /// * `name` - Function name
    pub fn get(&self, name: &str) -> Option<&DecoratorDefinition> {
        self.0.get(name)
    }

    /// Get a collection of all included decorators
    pub fn all(&self) -> Vec<&DecoratorDefinition> {
        let mut a: Vec<&DecoratorDefinition> = self.0.values().collect();
        a.sort_by(|f1, f2| f1.name()[0].cmp(f2.name()[0]));
        a
    }

    /// Call a decorator
    ///
    /// # Arguments
    /// * `name` - Decorator name
    /// * `args` - Decorator arguments
    pub fn call(&self, name: &str, token: &Token, arg: &Value) -> Result<String, Error> {
        match self.0.get(name) {
            Some(f) => f.call(token, arg),
            None => Err(Error::DecoratorName {
                name: name.to_string(),
                token: token.clone(),
            }),
        }
    }
}

impl Default for DecoratorTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Holds the definition of a builtin callable decorator
#[derive(Clone)]
pub struct DecoratorDefinition {
    /// Decorator call name
    pub name: &'static [&'static str],

    /// Decorator short description
    pub description: &'static str,

    /// Type of input the decorator expects
    pub argument: ExpectedTypes,

    /// Handler function
    pub handler: DecoratorHandler,
}
impl DecoratorDefinition {
    /// Return the decorator's names
    pub fn name(&self) -> &[&str] {
        self.name
    }

    /// Return the decorator's description
    pub fn description(&self) -> &str {
        self.description
    }

    /// Return the decorator's argument type
    pub fn arg(&self) -> ExpectedTypes {
        self.argument.clone()
    }

    /// Return the decorator's signature
    pub fn signature(&self) -> String {
        self.name
            .iter()
            .map(|n| format!("@{n}"))
            .collect::<Vec<String>>()
            .join("/")
    }

    /// Return the decorator's signature
    pub fn help(&self) -> String {
        format!("{}: {}", self.signature(), self.description)
    }

    /// Validate decorator arguments, and return an error if one exists
    ///
    /// # Arguments
    /// * `arg` - Decorator input
    pub fn validate(&self, token: &Token, arg: &Value) -> Option<Error> {
        if !self.arg().matches(arg) {
            Some(Error::DecoratorArgumentType {
                name: self.signature(),
                expected_type: self.arg(),
                token: token.clone(),
            })
        } else {
            None
        }
    }

    // Call the associated decorator handler
    ///
    /// # Arguments
    /// * `arg` - Decorator input
    pub fn call(&self, token: &Token, arg: &Value) -> Result<String, Error> {
        if let Some(error) = self.validate(token, arg) {
            Err(error)
        } else {
            (self.handler)(self, token, arg)
        }
    }
}

/// Runs a decorator on plural types
pub fn pluralized_decorator(
    decorator: &DecoratorDefinition,
    token: &Token,
    input: &Value,
) -> Result<String, Error> {
    match input {
        Value::Array(v) => {
            let mut output: Vec<Value> = Vec::new();
            for value in v {
                match decorator.call(token, value) {
                    Ok(s) => output.push(Value::from(s)),
                    Err(e) => return Err(e),
                }
            }
            Ok(Value::from(output).as_string())
        }

        Value::Object(v) => {
            let mut output: ObjectType = ObjectType::new();
            for (value, key) in v {
                match decorator.call(token, value) {
                    Ok(s) => {
                        output.insert(key.clone(), Value::from(s));
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(Value::from(output).as_string())
        }

        _ => decorator.call(token, input),
    }
}
