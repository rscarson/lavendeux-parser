use lavendeux_parser::{ParserState, FunctionNArgError, ParserError, Token, Value};

fn echo_function(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 1 {
        Err(ParserError::FunctionNArg(FunctionNArgError::new("echo(input)", 1, 1)))
    } else {
        Ok(Value::String(args[0].as_string()))
    }
}

fn main() -> Result<(), ParserError> {
    // Load the extensions into our parser
    let mut state : ParserState = ParserState::new();
    state.functions.register("echo", echo_function);
    
    // Now we can use the new functions and @decorators
    let token = Token::new("echo(5)", &mut state)?;
    assert_eq!(token.text(), "5");

    Ok(())
}