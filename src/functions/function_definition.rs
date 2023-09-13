use crate::{ParserState, Value, Token};
use crate::errors::*;
use super::{FunctionArgument, FunctionArgumentCollection, FunctionHandler};

const DEFAULT_CATEGORY : &str = "misc";

/// Holds the definition of a builtin callable function
#[derive(Clone)]
pub struct FunctionDefinition {
    /// Function call name
    pub name: &'static str,
    
    /// Function category
    pub category: Option<&'static str>,
    
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
    
    /// Return the function's category
    pub fn category(&self) -> &str {
        self.category.unwrap_or(DEFAULT_CATEGORY)
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

    /// Validate function arguments, and return the collected arguments
    /// 
    /// # Arguments
    /// * `args` - Function arguments
    pub fn collect(&self, token: &Token, args: &[Value]) -> Result<FunctionArgumentCollection, ParserError> {
        let optional_arguments = self.args().iter().filter(|e| e.optional()).count();
        let plural_arguments = self.args().iter().filter(|e| e.plural()).count();
        let max_arguments = self.args().len();
        let min_arguments = max_arguments - optional_arguments;

        // Prevent ambiguities resulting from plural args
        if plural_arguments > 1 {
            return Err(AmbiguousFunctionError::new(token, self.name(), "only one plural argument allowed in a function").into());
        } else if plural_arguments == 1 && !self.args().last().unwrap().plural() {
            return Err(AmbiguousFunctionError::new(token, self.name(), "plural argument must be the last function argument").into());
        }

        // Argument count
        if args.len() < min_arguments || (plural_arguments == 0 && args.len() > max_arguments) {
            return Err(FunctionNArgsError::new(token, &self.signature(), min_arguments, max_arguments).into())
        }

        // Collect argument values
        let mut arg_iter = args.iter();
        let mut argument_collection = FunctionArgumentCollection::new();
        for (args_consumed, arg) in self.args().into_iter().enumerate() {
            let values: Vec<Value> = arg_iter.by_ref().take(if arg.plural() {args.len() - args_consumed} else {1}).cloned()
                .collect();


            // Validate types
            for value in values {
                if arg.validate_value(&value) {
                    argument_collection.add(arg.name().to_string(), value.clone());
                } else {
                    return Err(FunctionArgTypeError::new(
                        token,
                        &self.signature(), 
                        args_consumed+1, 
                        arg.expected().clone()
                    ).into());
                }
            }
        }

        Ok(argument_collection)
    }

    // Call the associated function handler
    /// 
    /// # Arguments
    /// * `args` - Function arguments
    pub fn call(&self, token: &Token, state: &mut ParserState, args: &[Value]) -> Result<Value, ParserError> {
        match self.collect(token, args) {
            Ok(a) => (self.handler)(self, token, state, a),
            Err(e) => Err(e)
        }
    }
}