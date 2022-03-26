use lavendeux_parser::{ParserState, ParserError, Token};

fn main() -> Result<(), ParserError> {
    let expression = "
factorial(x) = x==0 ? 1 : (x * factorial(x - 1) )
x = factorial(5)
x == 5!
    ";

    let expected_result = "
x==0 ? 1 : (x * factorial(x - 1) )
120
true
    ";

    // Tokenize the expression
    let mut state : ParserState = ParserState::new();
    let lines = Token::new(expression, &mut state)?;
    
    assert_eq!(lines.text(), expected_result);
    assert_eq!(lines.child(1).unwrap().value(), 120);

    Ok(())
}