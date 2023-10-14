use super::pluralized_decorator;
use crate::{Error, ExpectedTypes, Value};

fn decorator_currency(input: &Value, symbol: &str) -> Result<String, Error> {
    let n = input.as_float().unwrap();
    let mut f = format!("{}{:.2}", symbol, n);
    if !f.contains('.') {
        f += ".0";
    }
    f = f
        .chars()
        .rev()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(",")
        .replacen(',', "", 1)
        .chars()
        .rev()
        .collect::<String>();
    if f.chars().nth(1).unwrap() == ',' {
        f = f.replacen(',', "", 1);
    }
    Ok(f)
}

define_decorator!(
    name = dollar,
    aliases = ["dollars", "usd", "aud", "cad"],
    description = "Format a number as a dollar amount",
    input = ExpectedTypes::IntOrFloat,
    handler = |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            decorator_currency(input, "$")
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
);

define_decorator!(
    name = euro,
    aliases = ["euros"],
    description = "Format a number as a euro amount",
    input = ExpectedTypes::IntOrFloat,
    handler = |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            decorator_currency(input, "€")
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
);

define_decorator!(
    name = pound,
    aliases = ["pounds"],
    description = "Format a number as a pound amount",
    input = ExpectedTypes::IntOrFloat,
    handler = |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            decorator_currency(input, "£")
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
);

define_decorator!(
    name = yen,
    description = "Format a number as a yen amount",
    input = ExpectedTypes::IntOrFloat,
    handler = |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            decorator_currency(input, "¥")
        } else {
            pluralized_decorator(decorator, token, input)
        }
    }
);

#[cfg(test)]
mod test_builtin_functions {
    use crate::Token;

    use super::*;

    #[test]
    fn test_currencies() {
        assert_eq!(
            "¥100.00",
            yen.call(&Token::dummy(""), &Value::Integer(100)).unwrap()
        );
        assert_eq!(
            "$1,000.00",
            dollar
                .call(&Token::dummy(""), &Value::Integer(1000))
                .unwrap()
        );
        assert_eq!(
            "€10,000.00",
            euro.call(&Token::dummy(""), &Value::Integer(10000))
                .unwrap()
        );
        assert_eq!(
            "£100,000.00",
            pound
                .call(&Token::dummy(""), &Value::Integer(100000))
                .unwrap()
        );
        assert_eq!(
            "£1,000,000.00",
            pound
                .call(&Token::dummy(""), &Value::Integer(1000000))
                .unwrap()
        );
    }
}
