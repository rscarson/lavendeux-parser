use lavendeux_parser::{ParserState, ParserError, Token, Value};
use lavendeux_parser::{DecoratorDefinition, FunctionDefinition, FunctionArgument};
use lavendeux_parser::errors::*;

fn main() -> Result<(), ParserError> {
    // Load the extensions into our parser
    let mut state : ParserState = ParserState::new();

    // Register a new function
    state.functions.register(FunctionDefinition {
        name: "echo",
        category: None,
        description: "Echo back the provided input",
        arguments: || vec![
            FunctionArgument::new_required("input", ExpectedTypes::String),
        ],
        handler: |_function, _state, args| {
            Ok(Value::String(args.get("input").required().as_string()))
        }
    });
    
    // Register a new decorator
    state.decorators.register(DecoratorDefinition {
        name: &["upper", "uppercase"],
        description: "Outputs an uppercase version of the input",
        argument: ExpectedTypes::Any,
        handler: |_, input| Ok(input.as_string().to_uppercase())
    });
    
    // Now we can use the new functions and @decorators
    let token = Token::new("echo('test') @upper", &mut state)?;
    assert_eq!(token.text(), "TEST");

    Ok(())
}