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
    let command = "complement(0xFF0000) @colour";
    println!("Running: {}", command);
    let token = Token::new(command, &mut state)?;

    println!("Result: {}", token.text());
    assert_eq!(token.text(), "#ffff00");

    Ok(())
}
