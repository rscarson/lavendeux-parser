use std::collections::HashMap;

use super::RuleHandler;
use crate::{
    state::ParserState,
    token::{OutputFormat, Rule, Token},
    Error, Value,
};

pub fn handler_table() -> HashMap<Rule, RuleHandler> {
    HashMap::from([
        (
            Rule::bool_cmp_expression,
            rule_bool_cmp_expression as RuleHandler,
        ),
        (
            Rule::bool_and_expression,
            rule_bool_and_expression as RuleHandler,
        ),
        (
            Rule::bool_or_expression,
            rule_bool_or_expression as RuleHandler,
        ),
    ])
}

/// A boolean comparison
/// x < 3
/// x == 3
fn rule_bool_cmp_expression(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    let mut i = 0;
    token.set_value(token.child(i).unwrap().value());
    while i < token.children().len() - 2 {
        let l = token.value();
        let r = token.child(i + 2).unwrap().value();

        token.set_value(Value::Boolean(match token.child(i + 1).unwrap().rule() {
            Rule::lt => l.lt(&r),
            Rule::gt => l.gt(&r),
            Rule::eq => l.eq(&r),
            Rule::ne => l.ne(&r),
            Rule::ge => l.ge(&r),
            Rule::le => l.le(&r),
            _ => return Some(Error::Internal(token.clone())),
        }));

        i += 2;
    }

    token.set_format(OutputFormat::Default); // Revert to boolean type
    None
}

/// A boolean and expression
/// a && b
fn rule_bool_and_expression(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    let mut i = 0;
    token.set_value(token.child(i).unwrap().value());
    while i < token.children().len() - 2 {
        token.set_value(Value::Boolean(
            token.value().as_bool() && token.child(i + 2).unwrap().value().as_bool(),
        ));
        i += 2
    }

    token.set_format(OutputFormat::Default); // Revert to boolean type
    None
}

/// A boolean or expression
/// a || b
fn rule_bool_or_expression(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    let mut i = 0;
    token.set_value(token.child(i).unwrap().value());
    while i < token.children().len() - 2 {
        token.set_value(Value::Boolean(
            token.value().as_bool() || token.child(i + 2).unwrap().value().as_bool(),
        ));
        i += 2
    }

    token.set_format(OutputFormat::Default); // Revert to boolean type
    None
}

#[cfg(test)]
mod test_token {
    use super::*;
    use crate::test::*;
    use crate::{test::assert_token_value, Value};

    #[test]
    fn rule_bool_cmp_expression() {
        assert_token_value!("'a' < 'b'", Value::from(true));
        assert_token_value!("'b' < 'a'", Value::from(false));
        assert_token_value!("'a' > 'b'", Value::from(false));
        assert_token_value!("'b' > 'a'", Value::from(true));
        assert_token_value!("'a' == 'b'", Value::from(false));
        assert_token_value!("'a' == 'a'", Value::from(true));
        assert_token_value!("'a' != 'b'", Value::from(true));
        assert_token_value!("'a' != 'a'", Value::from(false));
        assert_token_value!("'a' >= 'a'", Value::from(true));
        assert_token_value!("'a' <= 'b'", Value::from(true));

        assert_token_value!("false < true", Value::from(true));
        assert_token_value!("true < false", Value::from(false));
        assert_token_value!("false > true", Value::from(false));
        assert_token_value!("true > false", Value::from(true));
        assert_token_value!("false == true", Value::from(false));
        assert_token_value!("false == false", Value::from(true));
        assert_token_value!("false != true", Value::from(true));
        assert_token_value!("false != false", Value::from(false));
        assert_token_value!("false >= false", Value::from(true));
        assert_token_value!("false <= true", Value::from(true));

        assert_token_value!("1 < 2", Value::from(true));
        assert_token_value!("2 < 1", Value::from(false));
        assert_token_value!("1 > 2", Value::from(false));
        assert_token_value!("2 > 1", Value::from(true));
        assert_token_value!("1 == 2", Value::from(false));
        assert_token_value!("1 == 1", Value::from(true));
        assert_token_value!("1 != 2", Value::from(true));
        assert_token_value!("1 != 1", Value::from(false));
        assert_token_value!("1 >= 1", Value::from(true));
        assert_token_value!("1 <= 1", Value::from(true));

        assert_token_value!("1.3 < 2", Value::from(true));
        assert_token_value!("2 < 1.3", Value::from(false));
        assert_token_value!("1.3 > 2", Value::from(false));
        assert_token_value!("2 > 1.3", Value::from(true));
        assert_token_value!("1.3 == 2", Value::from(false));
        assert_token_value!("1.3 == 1.3", Value::from(true));
        assert_token_value!("1.3 != 2", Value::from(true));
        assert_token_value!("1.3 != 1.3", Value::from(false));
        assert_token_value!("1.3 >= 1.3", Value::from(true));
        assert_token_value!("1.3 <= 1.3", Value::from(true));

        assert_token_value!("'test' == 1", Value::from(false));
    }

    #[test]
    fn rule_bool_and_expression() {
        assert_token_value!("false && false", Value::from(false));
        assert_token_value!("false && true", Value::from(false));
        assert_token_value!("true && false", Value::from(false));
        assert_token_value!("true && true", Value::from(true));
        assert_token_value!("true && true && true && true", Value::from(true));
        assert_token_value!("true && true && true && false", Value::from(false));
    }

    #[test]
    fn rule_bool_or_expression() {
        assert_token_value!("false || false", Value::from(false));
        assert_token_value!("false || true", Value::from(true));
        assert_token_value!("true || false", Value::from(true));
        assert_token_value!("true || true", Value::from(true));
        assert_token_value!("false || false || false || false", Value::from(false));
        assert_token_value!("false || false || false || true", Value::from(true));
    }
}
