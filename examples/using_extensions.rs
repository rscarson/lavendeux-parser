use lavendeux_parser::{ParserState, ParserError, Token};

fn main() -> Result<(), ParserError> {
    // Load the extensions into our parser
    let mut state : ParserState = ParserState::new();
    state.extensions.load_all("../example_extensions");
    
    // Now we can use the new functions and @decorators
    let token = Token::new("complement(0xFF0000) @colour", &mut state)?;
    assert_eq!(token.text(), "#ffff00");

    Ok(())
}