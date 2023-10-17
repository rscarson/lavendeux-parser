use lavendeux_parser::{Error, ParserState, Token};

fn main() -> Result<(), Error> {
    // Load the extensions into our parser
    let mut state: ParserState = ParserState::new();
    let results = state.extensions.load_all("./example_extensions");

    for result in results {
        if let Err(err) = result {
            println!("{}", err);
        }
    }

    // Now we can use the new functions and @decorators
    let command = "add(1, 3) @colour";
    println!("Running: {}", command);
    let token = Token::new(command, &mut state)?;

    println!("Result: {}", token.text());
    assert_eq!(token.text(), "#000004");

    Ok(())
}
