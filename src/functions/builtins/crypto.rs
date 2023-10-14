//! Builtin cryptographic functions

use super::*;
use crate::{define_function, ExpectedTypes};
use rand::prelude::*;

#[cfg(feature = "crypto-functions")]
define_function!(
    name = sha256,
    category = "cryptography",
    description = "Returns the SHA256 hash of a given string",
    arguments = [function_arg!("plural", "input":Any)],
    handler = |function, token, state, args| {
        use sha2::{Digest, Sha256};
        let input = args.get("input").required().as_string();

        let mut hasher = Sha256::new();
        hasher.update(input);

        let s = format!("{:X}", hasher.finalize());
        Ok(Value::String(s))
    }
);

#[cfg(feature = "crypto-functions")]
const MD5: FunctionDefinition = FunctionDefinition {
    name: "md5",
    category: Some("cryptography"),
    description: "Returns the MD5 hash of a given string",
    arguments: || {
        vec![FunctionArgument::new_plural(
            "input",
            ExpectedTypes::Any,
            false,
        )]
    },
    handler: |_function, _token, _state, args| {
        use md5::{Digest, Md5};
        let input = args.get("input").required().as_string();

        let mut hasher = Md5::new();
        hasher.update(input);

        let s = format!("{:X}", hasher.finalize());
        Ok(Value::String(s))
    },
};

const CHOOSE: FunctionDefinition = FunctionDefinition {
    name: "choose",
    category: Some("cryptography"),
    description: "Returns any one of the provided arguments at random",
    arguments: || {
        vec![FunctionArgument::new_plural(
            "option",
            ExpectedTypes::Any,
            false,
        )]
    },
    handler: |_function, _token, _state, args| {
        let mut rng = rand::thread_rng();
        let arg = rng.gen_range(0..args.len());
        Ok(args[arg].clone())
    },
};

const RAND : FunctionDefinition = FunctionDefinition {
    name: "rand",
    category: Some("cryptography"),
    description: "With no arguments, return a float from 0 to 1. Otherwise return an integer from 0 to m, or m to n",
    arguments: || vec![
        FunctionArgument::new_optional("m", ExpectedTypes::Int),
        FunctionArgument::new_optional("n", ExpectedTypes::Int)
    ],
    handler: |_function, _token, _state, args| {
        let mut rng = rand::thread_rng();
        let m = args.get("m").optional_or(Value::Integer(0)).as_int().unwrap_or(0);
        let n = args.get("n").optional_or(Value::Integer(0)).as_int().unwrap_or(0);

        if m+n == 0 {
            // Generate a float between 0 and 1
            Ok(Value::Float(rng.gen()))
        } else if n>m {
            Ok(Value::Integer(rng.gen_range(m..n)))
        } else {
            Ok(Value::Integer(rng.gen_range(n..m)))
        }
    }
};

/// Register developper functions
pub fn register_functions(table: &mut FunctionTable) {
    #[cfg(feature = "crypto-functions")]
    table.register(sha256);

    #[cfg(feature = "crypto-functions")]
    table.register(MD5);

    table.register(CHOOSE);
    table.register(RAND);
}

#[cfg(test)]
mod test_builtin_table {
    use super::*;

    #[cfg(feature = "crypto-functions")]
    #[test]
    fn test_sha256() {
        let mut state = ParserState::new();

        let result = sha256
            .call(
                &Token::dummy(""),
                &mut state,
                &[Value::String("foobar".to_string())],
            )
            .unwrap()
            .as_string();

        assert_eq!(
            "C3AB8FF13720E8AD9047DD39466B3C8974E592C2FA383D4A3960714CAEF0C4F2".to_string(),
            result
        );
    }

    #[cfg(feature = "crypto-functions")]
    #[test]
    fn test_md5() {
        let mut state = ParserState::new();

        let result = MD5
            .call(
                &Token::dummy(""),
                &mut state,
                &[Value::String("foobar".to_string())],
            )
            .unwrap()
            .as_string();

        assert_eq!("3858F62230AC3C915F300C664312C63F".to_string(), result);
    }

    #[test]
    fn test_choose() {
        let mut state = ParserState::new();

        let mut result;
        for _ in 0..30 {
            result = CHOOSE
                .call(
                    &Token::dummy(""),
                    &mut state,
                    &[Value::String("test".to_string()), Value::Integer(5)],
                )
                .unwrap();
            assert_eq!(
                true,
                result.is_string() || result == Value::Integer(5).is_int()
            );
        }
    }

    #[test]
    fn test_rand() {
        let mut state = ParserState::new();

        let mut result;

        for _ in 0..30 {
            result = RAND.call(&Token::dummy(""), &mut state, &[]).unwrap();
            assert_eq!(
                true,
                result.as_float().unwrap() >= 0.0 && result.as_float().unwrap() <= 1.0
            );
        }

        for _ in 0..30 {
            result = RAND
                .call(&Token::dummy(""), &mut state, &[Value::Integer(5)])
                .unwrap();
            assert_eq!(
                true,
                result.as_int().unwrap() >= 0 && result.as_int().unwrap() <= 5
            );
        }

        for _ in 0..30 {
            result = RAND
                .call(
                    &Token::dummy(""),
                    &mut state,
                    &[Value::Integer(5), Value::Integer(10)],
                )
                .unwrap();
            assert_eq!(
                true,
                result.as_int().unwrap() >= 5 && result.as_int().unwrap() <= 10
            );
        }
    }
}
