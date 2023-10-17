use lavendeux_parser::{
    define_decorator, define_function, Error, ExpectedTypes, ParserState, Token, Value,
};

// Define a function for the parser
define_function!(
    name = echo,
    description = "Echo back the provided input",
    arguments = [function_arg!("input":String)],
    handler = |function, token, state, args| {
        Ok(Value::String(args.get("input").required().as_string()))
    }
);

// Define a decorator for the parser
define_decorator!(
    name = upper,
    aliases = ["uppercase"],
    description = "Outputs an uppercase version of the input",
    input = ExpectedTypes::Any,
    handler = |decorator, token, input| Ok(input.as_string().to_uppercase())
);

fn main() -> Result<(), Error> {
    // Load the extensions into our parser
    let mut state: ParserState = ParserState::new();

    // Register a new function and decorator
    state.functions.register(echo);
    state.decorators.register(upper);

    // Now we can use the new functions and @decorators
    let token = Token::new("echo('test') @upper", &mut state)?;
    assert_eq!(token.text(), "TEST");

    Ok(())
}
