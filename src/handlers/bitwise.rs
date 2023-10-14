use std::collections::HashMap;

use super::{perform_int_calculation, RuleHandler};
use crate::{
    state::ParserState,
    token::{Rule, Token},
    Error, ExpectedTypes, IntegerType,
};

pub fn handler_table() -> HashMap<Rule, RuleHandler> {
    HashMap::from([
        (Rule::sh_expression, rule_sh_expression as RuleHandler),
        (Rule::and_expression, rule_and_expression as RuleHandler),
        (Rule::xor_expression, rule_xor_expression as RuleHandler),
        (Rule::or_expression, rule_or_expression as RuleHandler),
    ])
}

/// A bitwise shift expression
/// x << 3
/// x >> 3
fn rule_sh_expression(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    token.set_value(token.child(0).unwrap().value());

    if token.children().len() > 1 {
        let mut i = 2;
        while i < token.children().len() {
            let ih = match token.child(i - 1).unwrap().rule() {
                Rule::lshift => |l: IntegerType, r: IntegerType| Some(l << r),
                Rule::rshift => |l: IntegerType, r: IntegerType| Some(l >> r),
                _ => return Some(Error::Internal(token.clone())),
            };

            if token.value().is_float() {
                return Some(Error::ValueType {
                    value: token.value(),
                    expected_type: ExpectedTypes::Int,
                    token: token.clone(),
                });
            } else if token.child(i).unwrap().value().is_float() {
                let token = token.child(i).unwrap();
                return Some(Error::ValueType {
                    value: token.value(),
                    expected_type: ExpectedTypes::Int,
                    token: token.clone(),
                });
            }

            match perform_int_calculation(token, token.value(), token.child(i).unwrap().value(), ih)
            {
                Ok(n) => token.set_value(n),
                Err(e) => return Some(e),
            }

            i += 2;
        }
    }

    None
}

/// A bitwise and expression
/// x & 3
fn rule_and_expression(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    token.set_value(token.child(0).unwrap().value());

    if token.children().len() > 1 {
        let mut i = 2;
        while i < token.children().len() {
            if token.value().is_float() || token.child(i).unwrap().value().is_float() {
                let token = token.child(i).unwrap();
                return Some(Error::ValueType {
                    value: token.value(),
                    expected_type: ExpectedTypes::IntOrFloat,
                    token: token.clone(),
                });
            }

            match perform_int_calculation(
                token,
                token.value(),
                token.child(i).unwrap().value(),
                |l: IntegerType, r: IntegerType| Some(l & r),
            ) {
                Ok(n) => token.set_value(n),
                Err(e) => return Some(e),
            }

            i += 2;
        }
    }

    None
}

/// A bitwise xor expression
/// x ^ 3
fn rule_xor_expression(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    token.set_value(token.child(0).unwrap().value());

    if token.children().len() > 1 {
        let mut i = 2;
        while i < token.children().len() {
            if token.value().is_float() || token.child(i).unwrap().value().is_float() {
                return Some(Error::ValueType {
                    value: token.value(),
                    expected_type: ExpectedTypes::Int,
                    token: token.clone(),
                });
            }

            match perform_int_calculation(
                token,
                token.value(),
                token.child(i).unwrap().value(),
                |l: IntegerType, r: IntegerType| Some(l ^ r),
            ) {
                Ok(n) => token.set_value(n),
                Err(e) => return Some(e),
            }

            i += 2;
        }
    }

    None
}

/// A bitwise or expression
/// x | 3
fn rule_or_expression(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    token.set_value(token.child(0).unwrap().value());

    if token.children().len() > 1 {
        let mut i = 2;
        while i < token.children().len() {
            if token.value().is_float() || token.child(i).unwrap().value().is_float() {
                return Some(Error::ValueType {
                    value: token.value(),
                    expected_type: ExpectedTypes::Int,
                    token: token.clone(),
                });
            }

            match perform_int_calculation(
                token,
                token.value(),
                token.child(i).unwrap().value(),
                |l: IntegerType, r: IntegerType| Some(l | r),
            ) {
                Ok(n) => token.set_value(n),
                Err(e) => return Some(e),
            }

            i += 2;
        }
    }

    None
}

#[cfg(test)]
mod test_token {
    use super::*;
    use crate::{test::*, Value};

    #[test]
    fn rule_sh_expression() {
        // Array values
        assert_token_value!(
            "4 >> [1,2]",
            Value::from(vec![Value::from(2), Value::from(1)])
        );
        assert_token_value!(
            "[4,16] >> [1,2]",
            Value::from(vec![Value::from(2), Value::from(4)])
        );
        assert_token_value!(
            "[4,8] >> 2",
            Value::from(vec![Value::from(1), Value::from(2)])
        );

        // Integer values
        assert_token_value!("4 >> 1", Value::from(2));
        assert_token_value!("2 << 2", Value::from(8));
        assert_token_value!("2 << 2 >> 2", Value::from(2));

        // Other typed values
        assert_token_error!("4.0 >> 1", ValueType);
        assert_token_error!("false >> 1.0", ValueType);
        assert_token_error!("4.0 >> 'test'", ValueType);
    }

    #[test]
    fn rule_and_expression() {
        // Array values
        assert_token_value!(
            "0xFF & [0xF0,0x0F]",
            Value::from(vec![Value::from(0xF0), Value::from(0x0F)])
        );
        assert_token_value!(
            "[0xF0,0x0F] & [0xA0,0x0A]",
            Value::from(vec![Value::from(0xA0), Value::from(0x0A)])
        );
        assert_token_value!(
            "[0xF0,0x0F] & 0xFF",
            Value::from(vec![Value::from(0xF0), Value::from(0x0F)])
        );

        // Integer values
        assert_token_value!("0xA & 0xF", Value::from(0xA));
        assert_token_value!("2 & 6", Value::from(2));
        assert_token_value!("3 & 2 & 7", Value::from(2));

        // Other typed values
        assert_token_error!("4.0 & 1", ValueType);
        assert_token_error!("false & 1", ValueType);
        assert_token_error!("4 & 'test'", ValueType);

        let mut state = ParserState::new();

        assert_eq!(
            Value::Array(vec![Value::Integer(15), Value::Integer(0),]),
            Token::new("0xFF & [0x0F, 0]", &mut state).unwrap().value()
        );

        assert_eq!(
            Value::Integer(15),
            Token::new("0xFF & 0x0F", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Integer(8),
            Token::new("0b1100 & 0b1110 & 0b1000", &mut state)
                .unwrap()
                .value()
        );
    }

    #[test]
    fn rule_xor_expression() {
        let mut state = ParserState::new();

        assert_eq!(
            Value::Array(vec![Value::Integer(240), Value::Integer(255),]),
            Token::new("0xFF ^ [0x0F, 0]", &mut state).unwrap().value()
        );

        assert_eq!(
            Value::Integer(240),
            Token::new("0xFF ^ 0x0F", &mut state).unwrap().value()
        );
        assert_eq!(
            Value::Integer(80),
            Token::new("0xFF ^ 0x0F ^ 0xA0", &mut state)
                .unwrap()
                .value()
        );

        // Array values
        assert_token_value!(
            "0xFF ^ [0x0F, 0]",
            Value::from(vec![Value::from(0xF0), Value::from(0xFF)])
        );
        assert_token_value!(
            "[0x0F, 0] ^ [0xFF, 0xFF]",
            Value::from(vec![Value::from(0xF0), Value::from(0xFF)])
        );
        assert_token_value!(
            "[0x0F, 0] ^ 0xFF",
            Value::from(vec![Value::from(0xF0), Value::from(0xFF)])
        );

        // Integer values
        assert_token_value!("0xFF ^ 0x0F", Value::from(0xF0));
        assert_token_value!("0xFF ^ 0x0F ^ 0xA0", Value::from(0x50));

        // Other typed values
        assert_token_error!("4.0 | 1", ValueType);
        assert_token_error!("false | 1", ValueType);
        assert_token_error!("4 | 'test'", ValueType);
    }

    #[test]
    fn rule_or_expression() {
        // Array values
        assert_token_value!(
            "0xFF00 | [0x00F0,0x000F]",
            Value::from(vec![Value::from(0xFFF0), Value::from(0xFF0F)])
        );
        assert_token_value!(
            "[0x00F0,0x000F] | [0xF000,0x0F00]",
            Value::from(vec![Value::from(0xF0F0), Value::from(0x0F0F)])
        );
        assert_token_value!(
            "[0x00F0,0x000F] | 0xFF00",
            Value::from(vec![Value::from(0xFFF0), Value::from(0xFF0F)])
        );

        // Integer values
        assert_token_value!("0x0A | 0xF0", Value::from(0xFA));
        assert_token_value!("2 | 4", Value::from(6));
        assert_token_value!("0x1 | 0x2 | 0x4", Value::from(7));

        // Other typed values
        assert_token_error!("4.0 | 1", ValueType);
        assert_token_error!("false | 1", ValueType);
        assert_token_error!("4 | 'test'", ValueType);
    }
}
