use crate::{Value, ParserState};
use super::errors::*;

/// Handler for executing a builtin function
pub type FunctionHandler = fn(&FunctionDefinition, state: &mut ParserState, FunctionArgumentCollection) -> Result<Value, ParserError>;

mod function_argument;
pub use function_argument::*;

mod function_definition;
pub use function_definition::*;

mod function_table;
pub use function_table::*;

mod builtins;
pub use builtins::*;

#[cfg(test)]
mod test_builtin_table {
    use super::*;

    const EXAMPLE : FunctionDefinition = FunctionDefinition {
        name: "example",
        category: None,
        description: "Sample function",
        arguments: || vec![
            FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat),
        ],
        handler: |_function, _state, _args| {
            Ok(Value::Integer(4))
        }
    };
    
    #[test]
    fn test_register() {
        let mut table = FunctionTable::new();
        table.register(EXAMPLE);
        assert_eq!(true, table.has("example"));
    }
    
    #[test]
    fn test_has() {
        let mut table = FunctionTable::new();
        table.register(EXAMPLE);
        assert_eq!(true, table.has("example"));
    }
    
    #[test]
    fn test_call() {
        let mut state = ParserState::new();
        let mut table = FunctionTable::new();
        table.register(EXAMPLE);

        table.call("example", &mut state, &[]).unwrap_err();
        table.call("example", &mut state, &[Value::String("".to_string())]).unwrap_err();
        table.call("example", &mut state, &[Value::Integer(4)]).unwrap();
    }
}