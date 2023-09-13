use lavendeux_parser::{ParserState, ParserError, Token};

fn main() -> Result<(), ParserError> {
    // Load the extensions into our parser
    let mut state : ParserState = ParserState::new();
    let results = state.extensions.load_all("./example_extensions");

    for result in results {
        if let Err(err) = result {
            println!("Error: {}", err);
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