use super::pluralized_decorator;
use crate::{Error, ExpectedTypes};
use chrono::{DateTime, NaiveDateTime, Utc};

define_decorator!(
    name = hex,
    description = "Base 16 number formatting, such as 0xFF",
    input = ExpectedTypes::IntOrFloat,
    handler = |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            Ok(format!("{:#0x}", input.as_int().unwrap()))
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
);

define_decorator!(
    name = oct,
    description = "Base 8 number formatting, such as 0b77",
    input = ExpectedTypes::IntOrFloat,
    handler = |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            Ok(format!("{:#0o}", input.as_int().unwrap()))
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
);

define_decorator!(
    name = bin,
    description = "Base 2 number formatting, such as 0b11",
    input = ExpectedTypes::IntOrFloat,
    handler = |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            Ok(format!("{:#0b}", input.as_int().unwrap()))
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
);

define_decorator!(
    name = sci,
    description = "Scientific number formatting, such as 1.2Ee-3",
    input = ExpectedTypes::IntOrFloat,
    handler = |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            Ok(format!("{:e}", input.as_float().unwrap()))
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
);

define_decorator!(
    name = utc,
    description = "Interprets an integer as a timestamp, and formats it in UTC standard",
    input = ExpectedTypes::IntOrFloat,
    handler = |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            let n = input.as_int().unwrap();
            match NaiveDateTime::from_timestamp_millis(n * 1000) {
                Some(t) => {
                    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(t, Utc);
                    Ok(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
                }
                None => Err(Error::Range {
                    value: input.clone(),
                    token: token.clone(),
                }),
            }
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
);

#[cfg(test)]
mod test_builtin_functions {
    use super::*;
    use crate::{Token, Value};

    #[test]
    fn test_hex() {
        assert_eq!(
            "0xff",
            hex.call(&Token::dummy(""), &Value::Integer(255)).unwrap()
        );
        assert_eq!(
            "0xff",
            hex.call(&Token::dummy(""), &Value::Float(255.1)).unwrap()
        );
    }

    #[test]
    fn test_bin() {
        assert_eq!(
            "0b11111111",
            bin.call(&Token::dummy(""), &Value::Integer(255)).unwrap()
        );
        assert_eq!(
            "0b11111111",
            bin.call(&Token::dummy(""), &Value::Float(255.1)).unwrap()
        );
    }

    #[test]
    fn test_oct() {
        assert_eq!(
            "0o10",
            oct.call(&Token::dummy(""), &Value::Integer(8)).unwrap()
        );
        assert_eq!(
            "0o10",
            oct.call(&Token::dummy(""), &Value::Float(8.1)).unwrap()
        );
    }

    #[test]
    fn test_sci() {
        assert_eq!(
            "8e0",
            sci.call(&Token::dummy(""), &Value::Integer(8)).unwrap()
        );
        assert_eq!(
            "-8.1e1",
            sci.call(&Token::dummy(""), &Value::Float(-81.0)).unwrap()
        );
        assert_eq!(
            "8.1e-2",
            sci.call(&Token::dummy(""), &Value::Float(0.081)).unwrap()
        );
    }
}
