use std::collections::HashMap;

use super::RuleHandler;
use crate::{
    state::ParserState,
    token::{Rule, Token},
    Error,
};

pub fn handler_table() -> HashMap<Rule, RuleHandler> {
    HashMap::from([
        (
            Rule::error_unterminated_literal,
            rule_error_unterminated_literal as RuleHandler,
        ),
        (
            Rule::error_unterminated_linebreak,
            rule_error_unterminated_linebreak as RuleHandler,
        ),
        (
            Rule::error_unterminated_array,
            rule_error_unterminated_array as RuleHandler,
        ),
        (
            Rule::error_unterminated_object,
            rule_error_unterminated_object as RuleHandler,
        ),
        (
            Rule::error_unterminated_paren,
            rule_error_unterminated_paren as RuleHandler,
        ),
        (
            Rule::error_unexpected_decorator,
            rule_error_unexpected_decorator as RuleHandler,
        ),
        (
            Rule::error_unexpected_postfix,
            rule_error_unexpected_postfix as RuleHandler,
        ),
    ])
}

/// Catches unterminated string literals
fn rule_error_unterminated_literal(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    Some(Error::UnterminatedLiteral(token.clone()))
}

/// Catches unterminated linebreaks
fn rule_error_unterminated_linebreak(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    Some(Error::UnterminatedLinebreak(token.clone()))
}

/// Catches unterminated arrays
fn rule_error_unterminated_array(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    Some(Error::UnterminatedArray(token.clone()))
}

/// Catches unterminated objects
fn rule_error_unterminated_object(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    Some(Error::UnterminatedObject(token.clone()))
}

/// Catches unterminated parens
fn rule_error_unterminated_paren(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    Some(Error::UnterminatedParen(token.clone()))
}

/// Catches decorator errors
fn rule_error_unexpected_decorator(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    Some(Error::UnexpectedDecorator(token.clone()))
}

/// Catches postfix errors
fn rule_error_unexpected_postfix(token: &mut Token, _state: &mut ParserState) -> Option<Error> {
    Some(Error::UnexpectedPostfix(token.clone()))
}

#[cfg(test)]
mod test_token {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_rule_error_unterminated_literal() {
        assert_token_error!("'test", UnterminatedLiteral);
    }

    #[test]
    fn test_rule_error_unterminated_linebreak() {
        assert_token_error!("33 \\", UnterminatedLinebreak);
    }

    #[test]
    fn test_rule_error_unterminated_array() {
        assert_token_error!("[1", UnterminatedArray);
    }

    #[test]
    fn test_rule_error_unterminated_object() {
        assert_token_error!("{1: 1", UnterminatedObject);
    }

    #[test]
    fn test_rule_error_unterminated_paren() {
        assert_token_error!("test(1", UnterminatedParen);
    }

    #[test]
    fn test_token_from_input() {
        assert_token_error!("@test", UnexpectedDecorator);
        assert_token_error!("test @test test", UnexpectedDecorator);
    }

    #[test]
    fn test_rule_error_unexpected_decorator() {
        assert_token_error!("@test", UnexpectedDecorator);
        assert_token_error!("test @test test", UnexpectedDecorator);
    }

    #[test]
    fn test_rule_error_unexpected_postfix() {
        assert_token_error!("!1", UnexpectedPostfix);
    }
}
