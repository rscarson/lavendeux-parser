use lavendeux_parser::{ParserState, ParserError, Token, Value};
use lavendeux_parser::{FunctionDefinition, FunctionArgument};
use lavendeux_parser::errors::*;

const ECHO : FunctionDefinition = FunctionDefinition {
    name: "echo",
    description: "Echo back the provided input",
    arguments: || vec![
        FunctionArgument::new_required("input", ExpectedTypes::String),
    ],
    handler: |_, args: &[Value]| {
        Ok(Value::String(args[0].as_string()))
    }
};

fn main() -> Result<(), ParserError> {
    // Load the extensions into our parser
    let mut state : ParserState = ParserState::new();
    state.functions.register(ECHO);
    
    // Now we can use the new functions and @decorators
    let token = Token::new("echo(5)", &mut state)?;
    assert_eq!(token.text(), "5");

    Ok(())
}