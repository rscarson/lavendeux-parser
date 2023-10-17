use super::{FunctionArgument, FunctionArgumentCollection, FunctionHandler};
use crate::Error;
use crate::{ParserState, Token, Value};

#[macro_use]
pub mod function_macros {
    /// Internal macro for function definitions
    /// See define_function
    #[macro_export]
    macro_rules! _define_function_category {
        () => {
            None
        };

        ($cat:literal) => {
            Some($cat)
        };
    }

    /// Describes the requirements of an argument to a builtin function
    ///
    /// Examples:
    /// ```ignore
    /// function_arg!("name_of_variable")
    /// function_arg!("plural", "name_of_variable_array")
    /// function_arg!("optional", "name_of__optional_variable")
    /// function_arg!("plural+optional", "name_of__optional_variable_array")
    /// ```
    #[macro_export]
    macro_rules! function_arg {
        ($name:literal:$type:ident) => {
            $crate::FunctionArgument::new($name, crate::ExpectedTypes::$type, false)
        };

        ("plural", $name:literal:$type:ident) => {
            $crate::FunctionArgument::new_plural($name, crate::ExpectedTypes::$type, false)
        };

        ("optional", $name:literal:$type:ident) => {
            $crate::FunctionArgument::new($name, crate::ExpectedTypes::$type, true)
        };

        ("plural+optional", $name:literal:$type:ident) => {
            $crate::FunctionArgument::new_plural($name, crate::ExpectedTypes::$type, true)
        };
    }

    /// Defines a function for registration as a builtin
    ///
    /// name = identifier for the new function, and the callable name,
    /// category = Optional string category for the help menu
    /// description = String describing the function
    /// arguments = Set of arguments defined with function_arg!
    /// handler = closure taking in |function, token, state, args|
    ///
    /// Example:
    /// ```ignore
    /// define_function!(
    ///     name = echo,
    ///     description = "Echo back the provided input",
    ///     arguments = [function_arg!("input":String)],
    ///     handler = |function, token, state, args| {
    ///         Ok(Value::String(args.get("input").required().as_string()))
    ///     }
    /// );
    /// ```
    #[macro_export]
    macro_rules! define_function {
        (
            name = $function_name:ident,
            $(category = $function_cat:expr,)?
            description = $function_desc:literal,
            arguments = [$($function_arg:expr),*],
            handler = $function_impl:expr
        ) => {
            /// Builtin-function definition for use with Lavendeux
            /// It should be registered with 'function_table.register(
            #[doc = stringify!($function_name)]
            /// );
            #[allow(non_upper_case_globals, unused_variables)]
            const $function_name: $crate::FunctionDefinition = $crate::FunctionDefinition {
                name: stringify!($function_name),
                category: $crate::_define_function_category!($($function_cat)?),
                description: "Returns the SHA256 hash of a given string",
                arguments: || {
                    vec![$crate::FunctionArgument::new_plural(
                        "input",
                        $crate::ExpectedTypes::Any,
                        false,
                    )]
                },
                handler: $function_impl,
            };
        };
    }
}

const DEFAULT_CATEGORY: &str = "misc";

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
    pub handler: FunctionHandler,
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
        format!(
            "{}({})",
            self.name,
            self.args()
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    /// Return the function's help string
    pub fn help(&self) -> String {
        format!("{}: {}", self.signature(), self.description())
    }

    /// Validate function arguments, and return the collected arguments
    ///
    /// # Arguments
    /// * `args` - Function arguments
    pub fn collect(
        &self,
        token: &Token,
        args: &[Value],
    ) -> Result<FunctionArgumentCollection, Error> {
        let optional_arguments = self.args().iter().filter(|e| e.optional()).count();
        let plural_arguments = self.args().iter().filter(|e| e.plural()).count();
        let max_arguments = self.args().len();
        let min_arguments = max_arguments - optional_arguments;

        // Prevent ambiguities resulting from plural args
        if plural_arguments > 1 || (plural_arguments == 1 && !self.args().last().unwrap().plural())
        {
            return Err(Error::AmbiguousFunctionDefinition {
                signature: self.signature(),
                token: token.clone(),
            });
        }

        // Argument count
        if args.len() < min_arguments || (plural_arguments == 0 && args.len() > max_arguments) {
            return Err(Error::FunctionArguments {
                min: min_arguments,
                max: max_arguments,
                signature: self.signature(),
                token: token.clone(),
            });
        }

        // Collect argument values
        let mut arg_iter = args.iter();
        let mut argument_collection = FunctionArgumentCollection::new();
        for (args_consumed, arg) in self.args().into_iter().enumerate() {
            let values: Vec<Value> = arg_iter
                .by_ref()
                .take(if arg.plural() {
                    args.len() - args_consumed
                } else {
                    1
                })
                .cloned()
                .collect();

            // Validate types
            for value in values {
                if arg.validate_value(&value) {
                    argument_collection.add(arg.name().to_string(), value.clone());
                } else {
                    return Err(Error::FunctionArgumentType {
                        arg: args_consumed + 1,
                        expected_type: *arg.expected(),
                        signature: self.signature(),
                        token: token.clone(),
                    });
                }
            }
        }

        Ok(argument_collection)
    }

    // Call the associated function handler
    ///
    /// # Arguments
    /// * `args` - Function arguments
    pub fn call(
        &self,
        token: &Token,
        state: &mut ParserState,
        args: &[Value],
    ) -> Result<Value, Error> {
        match self.collect(token, args) {
            Ok(a) => (self.handler)(self, token, state, a),
            Err(e) => Err(e),
        }
    }
}
