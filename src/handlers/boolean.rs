use crate::token::{Rule, Token, OutputFormat};
use crate::value::Value;
use crate::state::ParserState;
use crate::errors::*;

pub fn bool_expression_handler(token: &mut Token, _state: &mut ParserState) -> Option<ParserError> {
    match token.rule() {
        Rule::bool_cmp_expression => {
            let mut i = 0;
            token.set_value(token.child(i).unwrap().value());
            while i < token.children().len() - 2 {
                let l = token.value();
                let r = token.child(i+2).unwrap().value();

                if l.is_string() && r.is_string() {
                    match token.child(i+1).unwrap().rule() {
                        Rule::lt => token.set_value(Value::Boolean(l.as_string() < r.as_string())),
                        Rule::gt => token.set_value(Value::Boolean(l.as_string() > r.as_string())),
                        Rule::eq => token.set_value(Value::Boolean(l.as_string() == r.as_string())),
                        Rule::ne => token.set_value(Value::Boolean(l.as_string() != r.as_string())),
                        Rule::ge => token.set_value(Value::Boolean(l.as_string() >= r.as_string())),
                        Rule::le => token.set_value(Value::Boolean(l.as_string() <= r.as_string())),
                        _ => {}
                    }
                } else if l.is_bool() && r.is_bool() {
                    match token.child(i+1).unwrap().rule() {
                        Rule::lt => token.set_value(Value::Boolean(!l.as_bool() & r.as_bool())),
                        Rule::gt => token.set_value(Value::Boolean(l.as_bool() & !r.as_bool())),
                        Rule::eq => token.set_value(Value::Boolean(l.as_bool() == r.as_bool())),
                        Rule::ne => token.set_value(Value::Boolean(l.as_bool() != r.as_bool())),
                        Rule::ge => token.set_value(Value::Boolean(l.as_bool() >= r.as_bool())),
                        Rule::le => token.set_value(Value::Boolean(l.as_bool() <= r.as_bool())),
                        _ => {}
                    }
                } else if l.is_int() && r.is_int() {
                    match token.child(i+1).unwrap().rule() {
                        Rule::lt => token.set_value(Value::Boolean(l.as_int().unwrap() < r.as_int().unwrap())),
                        Rule::gt => token.set_value(Value::Boolean(l.as_int().unwrap() > r.as_int().unwrap())),
                        Rule::eq => token.set_value(Value::Boolean(l.as_int().unwrap() == r.as_int().unwrap())),
                        Rule::ne => token.set_value(Value::Boolean(l.as_int().unwrap() != r.as_int().unwrap())),
                        Rule::ge => token.set_value(Value::Boolean(l.as_int().unwrap() >= r.as_int().unwrap())),
                        Rule::le => token.set_value(Value::Boolean(l.as_int().unwrap() <= r.as_int().unwrap())),
                        _ => {}
                    }
                } else if l.is_numeric() && r.is_numeric() {
                    match token.child(i+1).unwrap().rule() {
                        Rule::lt => token.set_value(Value::Boolean(l.as_float().unwrap() < r.as_float().unwrap())),
                        Rule::gt => token.set_value(Value::Boolean(l.as_float().unwrap() > r.as_float().unwrap())),
                        Rule::eq => token.set_value(Value::Boolean(l.as_float().unwrap() == r.as_float().unwrap())),
                        Rule::ne => token.set_value(Value::Boolean(l.as_float().unwrap() != r.as_float().unwrap())),
                        Rule::ge => token.set_value(Value::Boolean(l.as_float().unwrap() >= r.as_float().unwrap())),
                        Rule::le => token.set_value(Value::Boolean(l.as_float().unwrap() <= r.as_float().unwrap())),
                        _ => {}
                    }
                } else {
                    token.set_value(Value::Boolean(false));
                }

                i += 2;
            }

            token.set_format(OutputFormat::Default); // Revert to boolean type
        },
        
        Rule::bool_and_expression => {
            let mut i = 0;
            token.set_value(token.child(i).unwrap().value());
            while i < token.children().len() - 2 {
                token.set_value(Value::Boolean(token.value().as_bool() && token.child(i+2).unwrap().value().as_bool()));
                i += 2
            }

            token.set_format(OutputFormat::Default); // Revert to boolean type
        },
        
        Rule::bool_or_expression => {
            let mut i = 0;
            token.set_value(token.child(i).unwrap().value());
            while i < token.children().len() - 2 {
                token.set_value(Value::Boolean(token.value().as_bool() || token.child(i+2).unwrap().value().as_bool()));
                i += 2
            }

            token.set_format(OutputFormat::Default); // Revert to boolean type
        },

        _ => { }
    }

    None
}

#[cfg(test)]
mod test_token {
    use super::*;

    #[test]
    fn test_bool_cmp_expression() {
        let mut state: ParserState = ParserState::new();
        
        assert_eq!(Value::Boolean(true), Token::new(" 'a' < 'b' ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" 'b' < 'a' ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" 'a' > 'b' ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" 'b' > 'a' ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" 'a' == 'b' ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" 'a' == 'a' ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" 'a' != 'b' ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" 'a' != 'a' ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" 'a' >= 'a' ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" 'a' <= 'b' ", &mut state).unwrap().value());
        
        assert_eq!(Value::Boolean(true), Token::new(" false < true ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" true < false ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" false > true ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" true > false ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" false == true ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" false == false ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" false != true ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" false != false ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" false >= false ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" false <= true ", &mut state).unwrap().value());
        
        assert_eq!(Value::Boolean(true), Token::new(" 1 < 2 ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" 2 < 1 ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" 1 > 2 ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" 2 > 1 ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" 1 == 2 ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" 1 == 1 ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" 1 != 2 ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" 1 != 1 ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" 1 >= 1 ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" 1 <= 1 ", &mut state).unwrap().value());
        
        assert_eq!(Value::Boolean(true), Token::new(" 1.3 < 2 ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" 2 < 1.3 ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" 1.3 > 2 ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" 2 > 1.3 ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" 1.3 == 2 ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" 1.3 == 1.3 ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" 1.3 != 2 ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" 1.3 != 1.3 ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" 1.3 >= 1.3 ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" 1.3 <= 1.3 ", &mut state).unwrap().value());
        
        assert_eq!(Value::Boolean(false), Token::new(" '1' == 1 ", &mut state).unwrap().value());
    }

    #[test]
    fn test_bool_and_expression() {
        let mut state: ParserState = ParserState::new();
        assert_eq!(Value::Boolean(false), Token::new(" false && false ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" false && true ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" true && false ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" true && true ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" true && true && true && true ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" true && true && true && false ", &mut state).unwrap().value());
    }

    #[test]
    fn test_bool_or_expression() {
        let mut state: ParserState = ParserState::new();
        assert_eq!(Value::Boolean(false), Token::new(" false || false ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" false || true ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" true || false ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" true || true ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new(" false || false || false || false ", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new(" false || false || false || true ", &mut state).unwrap().value());
    }
}