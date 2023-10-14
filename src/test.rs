//! Contains macros to test the parser
//!

/// Assert that a token parses succesfully into a given string
/// assert_token_text_stateful!("input text", target_value)
#[allow(unused_macros)]
macro_rules! assert_token_text_stateful {
    ($input:expr, $text:expr, $state:expr) => {
        match Token::new($input, $state) {
            Ok(result) => assert_eq!($text, result.text()),
            Err(e) => panic!("{}", e),
        }
    };
}
#[allow(unused_imports)]
pub(crate) use assert_token_text_stateful;

/// Assert that a token parses succesfully into a given string
/// assert_token_text!("input text", target_text)
#[allow(unused_macros)]
macro_rules! assert_token_text {
    ($input:expr, $text:expr) => {
        assert_token_text_stateful!($input, $text, &mut crate::ParserState::new())
    };
}
#[allow(unused_imports)]
pub(crate) use assert_token_text;

/// Assert that a token parses succesfully into a given value
/// assert_token_value!("input text", target_value)
#[allow(unused_macros)]
macro_rules! assert_token_value_stateful {
    ($input:expr, $value:expr, $state:expr) => {
        match Token::new($input, $state) {
            Ok(result) => assert_eq!($value, result.value()),
            Err(e) => panic!("{}", e),
        }
    };
}
#[allow(unused_imports)]
pub(crate) use assert_token_value_stateful;

/// Assert that a token parses succesfully into a given value
/// assert_token_value!("input text", target_value)
#[allow(unused_macros)]
macro_rules! assert_token_value {
    ($input:expr, $value:expr) => {
        assert_token_value_stateful!($input, $value, &mut crate::ParserState::new())
    };
}
#[allow(unused_imports)]
pub(crate) use assert_token_value;

/// Assert that a token results in a given error
/// assert_token_error!("input text", Value)
#[allow(unused_macros)]
macro_rules! assert_token_error_stateful {
    ($input:expr, $error:ident, $state:expr) => {
        match Token::new($input, $state) {
            Ok(result) => panic!("No error in {} (result was {})", $input, result.text()),
            Err(e) => {
                if !matches!(e, $crate::Error::$error { .. }) {
                    panic!(
                        "Error '{}' does not match {:?} for token {}",
                        e,
                        stringify!($error),
                        $input
                    )
                }
            }
        }
    };
}
#[allow(unused_imports)]
pub(crate) use assert_token_error_stateful;

/// Assert that a token results in a given error
/// assert_token_error!("input text", Value)
#[allow(unused_macros)]
macro_rules! assert_token_error {
    ($input:expr, $error:ident) => {
        assert_token_error_stateful!($input, $error, &mut crate::ParserState::new())
    };
}
#[allow(unused_imports)]
pub(crate) use assert_token_error;
