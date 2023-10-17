use crate::{DecoratorDefinition, Error, ExpectedTypes, Value};

use super::pluralized_decorator;

pub const PERCENTAGE: DecoratorDefinition = DecoratorDefinition {
    name: &["percentage", "percent"],
    description: "Format a floating point number as a percentage",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            Ok(format!("{}%", input.as_float().unwrap() * 100.0))
        } else {
            pluralized_decorator(decorator, token, input)
        }
    },
};

pub const ORDINAL: DecoratorDefinition = DecoratorDefinition {
    name: &["ordinal"],
    description: "Format an integer as an ordinal (1st, 38th, etc)",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            let v = Value::Integer(input.as_int().unwrap()).as_string();
            let suffix = if v.ends_with('1') {
                "st"
            } else if v.ends_with('2') {
                "nd"
            } else if v.ends_with('3') {
                "rd"
            } else {
                "th"
            };
            Ok(format!("{}{}", v, suffix))
        } else {
            pluralized_decorator(decorator, token, input)
        }
    },
};

pub const ROMAN: DecoratorDefinition = DecoratorDefinition {
    name: &["roman"],
    description: "Format an integer as a roman numeral",
    argument: ExpectedTypes::IntOrFloat,
    handler: |decorator, token, input| {
        if decorator.arg().strict_matches(input) {
            let mut value = input.as_int().unwrap();
            if value > 3999 {
                return Err(Error::Overflow(token.clone()));
            }

            let roman_numerals = vec![
                (1000, "M"),
                (900, "CM"),
                (500, "D"),
                (400, "CD"),
                (100, "C"),
                (90, "XC"),
                (50, "L"),
                (40, "XL"),
                (10, "X"),
                (9, "IX"),
                (5, "V"),
                (4, "IV"),
                (1, "I"),
            ];
            let mut roman_numeral = String::new();
            for (n, r) in roman_numerals {
                while value >= n {
                    roman_numeral.push_str(r);
                    value -= n;
                }
            }
            Ok(roman_numeral)
        } else {
            pluralized_decorator(decorator, token, input)
        }
    },
};

#[cfg(test)]
mod test_builtin_functions {
    use crate::Token;

    use super::*;

    #[test]
    fn test_ordinal() {
        assert_eq!(
            "32nd",
            ORDINAL
                .call(&Token::dummy(""), &Value::Integer(32))
                .unwrap()
        );
    }

    #[test]
    fn test_percentage() {
        assert_eq!(
            "32.5%",
            PERCENTAGE
                .call(&Token::dummy(""), &Value::Float(0.325))
                .unwrap()
        );
    }

    #[test]
    fn test_roman() {
        assert_eq!(
            "XXVI",
            ROMAN.call(&Token::dummy(""), &Value::Integer(26)).unwrap()
        );
    }
}
