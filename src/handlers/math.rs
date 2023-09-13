use std::collections::HashMap;

use super::{ RuleHandler, perform_calculation };
use crate::{
    token::{Rule, Token},
    state::ParserState,
    Value,
    FloatType,
    IntegerType,
    errors::*, errors::ValueTypeError
};

/// Perform overflow checked exponentiation
/// 
/// # Arguments
/// * `l` - Left value
/// * `r` - Right value
fn integer_type_checked_pow(l:IntegerType, r:IntegerType) -> Option<IntegerType> {
    if r > u32::MAX as IntegerType { return None; }
    if r == IntegerType::MIN { return None; }
    match l.checked_pow(r.checked_abs().unwrap() as u32) {
        Some(v) => {
            if r < 0 {
                Some(1/v)
            } else {
                Some(v)
            }
        },
        None => None
    }
}

/// Perform a checked factorial
/// 
/// # Arguments
/// * `source` - Source token
/// * `input` - input value
pub fn factorial(source: &Token, input: &Value) -> Result<Value, ParserError> {
    if input.is_identifier() {
        return Err(VariableNameError::new( source, &input.as_string()).into())
    }

    if let Some(v) = input.as_int() {
        match v {
            0 => Ok(Value::Integer(1)),
            1.. => {
                let mut acc : IntegerType = 1;
                for i in 1..=v {
                    if let Some(acc_) = acc.checked_mul(i as IntegerType) {
                        acc = acc_;
                    } else {
                        return Err(OverflowError::new(source).into())
                    }
                }
    
                Ok(Value::Integer(acc))
            },
    
            _ => Err(UnderflowError::new(source).into())
        }
    } else if input.is_array() {
        let mut out = input.as_array();
        for (i, e) in out.clone().iter().enumerate() {
            match factorial(source, e) {
                Ok(v) => out[i] = v,
                Err(e) => return Err(e)
            }
        }
        Ok(Value::Array(out))
    } else {
        Err(ValueTypeError::new(source, ExpectedTypes::IntOrFloat).into())
    }
}

/// Trim a binary value to match the precision of a given base. Useful for inversion
/// 
/// # Arguments
/// * `input` - Source value
/// * `base` - Number to check against
fn trim_binary(input: Value, base: IntegerType) -> Option<Value> {
    match input.as_int() {
        Some(n) => {
            let mask : IntegerType = ((2_u32).pow( ((base as FloatType).log2().floor() + 1.0) as u32) - 1) as IntegerType;
            Some(Value::Integer(n & if mask==0 {!mask} else {mask}))
        },
        None => None
    }
}

/// Perform a unary arithmetic negation
/// 
/// # Arguments
/// * `expression` - Source token
/// * `value` - Value to process
fn unary_minus(expression: &Token, value: Value) -> Result<Value, ParserError> {
    match value {
        Value::Integer(n) => Ok(Value::Integer(-n)),
        Value::Float(n) => Ok(Value::Float(-n)),
        Value::Boolean(n) => Ok(Value::Boolean(!n)),
        Value::Identifier(s) => Err(VariableNameError::new(expression, &s).into()),
        Value::Array(a) => {
            let mut ra = a;
            for (pos, e) in ra.clone().iter().enumerate() {
                match unary_minus(expression, e.clone()) {
                    Ok(n) => ra[pos] = n,
                    Err(e) => return Err(e) 
                }
            }
            Ok(Value::Array(ra))
        },
        _ => Err(ValueTypeError::new(expression, ExpectedTypes::IntOrFloat).into())
    }
}

/// Perform a unary bitwise negation
/// 
/// # Arguments
/// * `expression` - Source token
/// * `value` - Value to process
fn unary_not(expression: &Token, value: Value) -> Result<Value, ParserError> {
    match value {
        Value::Boolean(n) => Ok(Value::Boolean(!n)),
        Value::Integer(n) => {
            match trim_binary(Value::Integer(!n), n) {
                Some(v) => Ok(v),
                None => Err(ValueTypeError::new(expression, ExpectedTypes::Int).into())
            }
        },
        Value::Array(a) => {
            let mut ra = a;
            for (pos, e) in ra.clone().iter().enumerate() {
                match unary_not(expression, e.clone()) {
                    Ok(n) => ra[pos] = n,
                    Err(e) => return Err(e) 
                }
            }
            Ok(Value::Array(ra))
        },
        _ => Err(ValueTypeError::new(expression, ExpectedTypes::Int).into())
    }
}

pub fn handler_table() -> HashMap<Rule, RuleHandler> {
    HashMap::from([
        (Rule::as_expression, rule_as_expression as RuleHandler),
        (Rule::implied_mul_expression, rule_implied_mul_expression as RuleHandler),
        (Rule::md_expression, rule_md_expression as RuleHandler),
        (Rule::power_expression, rule_power_expression as RuleHandler),
        (Rule::postfix_unary_expression, rule_postfix_unary_expression as RuleHandler),
        (Rule::prefix_unary_expression, rule_prefix_unary_expression as RuleHandler),
    ])
}

fn rule_as_expression(token: &mut Token, _state: &mut ParserState) -> Option<ParserError> {
    token.set_value(token.child(0).unwrap().value());
    if token.children().len() > 1 {
        let mut i = 2;
        while i < token.children().len() {
            match token.child(i - 1).unwrap().rule() {
                Rule::plus => {
                    if token.value().is_string() || token.child(i).unwrap().value().is_string() {
                        token.set_value(Value::String(format!("{}{}", token.value().as_string(), token.child(i).unwrap().value().as_string())));
                    } else {
                        match perform_calculation(
                            token, token.value(), token.child(i).unwrap().value(), 
                            IntegerType::checked_add, |l: FloatType, r: FloatType| l + r
                        ) {
                            Ok(n) => token.set_value(n),
                            Err(e) => return Some(e)
                        };
                    }
                },

                Rule::minus => {
                    match perform_calculation(
                        token, token.value(), token.child(i).unwrap().value(), 
                        IntegerType::checked_sub, |l: FloatType, r: FloatType| l - r
                    ) {
                        Ok(n) => token.set_value(n),
                        Err(e) => return Some(e)
                    };
                },

                _ => return Some(InternalError::new(token).into())
            }

            i += 2;
        }
    }

    None
}

fn rule_implied_mul_expression(token: &mut Token, _state: &mut ParserState) -> Option<ParserError> {
    token.set_value(token.child(0).unwrap().value());
    if token.children().len() > 1 {
        let mut i = 1;
        while i < token.children().len() {
            let next_child = token.child(i).unwrap();
            if next_child.text() == "(" || next_child.text() == ")" {
                continue;
            }

            let ih = IntegerType::checked_mul;
            let fh = |l: FloatType, r: FloatType| l * r;

            match perform_calculation(token, token.value(), token.child(i).unwrap().value(), ih, fh) {
                Ok(n) => token.set_value(n),
                Err(e) => return Some(e)
            }

            i += 1;
        }
    }
    
    None
}

fn rule_md_expression(token: &mut Token, _state: &mut ParserState) -> Option<ParserError> {
    token.set_value(token.child(0).unwrap().value());

    if token.children().len() > 1 {
        let mut i = 2;
        while i < token.children().len() {
            let ih = match token.child(i - 1).unwrap().rule() {
                Rule::multiply => IntegerType::checked_mul,
                Rule::divide => IntegerType::checked_div,
                Rule::modulus => IntegerType::checked_rem_euclid,
                _ => return Some(InternalError::new(token).into())
            };
            
            let fh = match token.child(i - 1).unwrap().rule() {
                Rule::multiply => |l: FloatType, r: FloatType| l * r,
                Rule::divide => |l: FloatType, r: FloatType| l / r,
                Rule::modulus => FloatType::rem_euclid,
                _ => return Some(InternalError::new(token).into())
            };

            match perform_calculation(token, token.value(), token.child(i).unwrap().value(), ih, fh) {
                Ok(n) => token.set_value(n),
                Err(e) => return Some(e)
            }

            i += 2;
        }
    }

    None
}

fn rule_power_expression(token: &mut Token, _state: &mut ParserState) -> Option<ParserError> {
    token.set_value(token.child(0).unwrap().value());

    if token.children().len() > 1 {
        let mut i = 2;
        while i < token.children().len() {
            match perform_calculation(token, token.value(), token.child(i).unwrap().value(), integer_type_checked_pow, FloatType::powf) {
                Ok(n) => token.set_value(n),
                Err(e) => return Some(e)
            }

            i += 2;
        }
    }

    None
}

fn rule_prefix_unary_expression(token: &mut Token, _state: &mut ParserState) -> Option<ParserError> {
    if token.children().len() >= 2 {
        let mut idx = token.children().len() - 1;
        token.set_value(token.child(idx).unwrap().value());
        while idx >0 {
            idx-=1;

            if token.child(idx).unwrap().rule() == Rule::minus {
                match unary_minus(token, token.value()) {
                    Ok(n) => token.set_value(n),
                    Err(e) => return Some(e)
                }
            } else if token.child(idx).unwrap().rule() == Rule::not {
                match unary_not(token, token.value()) {
                    Ok(n) => token.set_value(n),
                    Err(e) => return Some(e)
                }
            }
        }
    }

    None
}

fn rule_postfix_unary_expression(token: &mut Token, _state: &mut ParserState) -> Option<ParserError> {
    if token.children().last().unwrap().text() == "!" {
        token.set_value(token.child(0).unwrap().value());
        if token.children().len() >= 2 {
            let mut i = 1;
            while i < token.children().len() {
                if token.child(i).unwrap().rule() == Rule::factorial {
                    match factorial(token, &token.value()) {
                        Ok(v) => token.set_value(v),
                        Err(e) => return Some(e)
                        
                    }
                }

                i+=1;
            }
        }
    }
    
    None
}

#[cfg(test)]
mod test_token {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_integer_type_checked_pow() {
        assert_eq!(1, integer_type_checked_pow(10, 0).unwrap());
        assert_eq!(10, integer_type_checked_pow(10, 1).unwrap());
        assert_eq!(100, integer_type_checked_pow(10, 2).unwrap());
        assert_eq!(0, integer_type_checked_pow(100, -1).unwrap());
    }

    #[test]
    fn test_factorial() {
        let mut state = ParserState::new();
        let token = Token::new("1", &mut state).unwrap();

        assert_eq!(1, factorial(&token, &Value::Integer(0)).unwrap().as_int().unwrap());
        assert_eq!(1, factorial(&token, &Value::Integer(1)).unwrap().as_int().unwrap());
        assert_eq!(2, factorial(&token, &Value::Integer(2)).unwrap().as_int().unwrap());
        assert_eq!(24, factorial(&token, &Value::Integer(4)).unwrap().as_int().unwrap());
        assert_eq!(24, factorial(&token, &Value::Float(4.0)).unwrap().as_int().unwrap());
        assert_eq!(true, factorial(&token, &Value::Integer(99)).is_err());
        assert_eq!(true, factorial(&token, &Value::Integer(-1)).is_err());
    }

    #[test]
    fn test_trim_binary() {
        assert_eq!(Value::Integer(255), trim_binary(Value::Integer(65535), 255).unwrap());
        assert_eq!(Value::Integer(9999), trim_binary(Value::Integer(9999), 9999).unwrap());
    }

    #[test]
    fn test_prefix_unary_expression_minus() {
        let mut state = ParserState::new();
        assert_eq!(Value::Array(vec![
            Value::Integer(-1), Value::Integer(1), Value::Float(1.0), 
        ]), Token::new("-[1,-1, -1.0]", &mut state).unwrap().value());
        assert_eq!(Value::Integer(-255), Token::new("-255", &mut state).unwrap().value());
        assert_eq!(Value::Float(-255.0), Token::new("-255.0", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(true), Token::new("-false", &mut state).unwrap().value());
        assert_eq!(true, Token::new("-'test'", &mut state).is_err());
    }

    #[test]
    fn test_prefix_unary_expression_not() {
        let mut state = ParserState::new();
        assert_eq!(Value::Array(vec![
            Value::Integer(0), Value::Integer(3), Value::Boolean(false), 
        ]), Token::new("~[255, 0b1100, true]", &mut state).unwrap().value());
        assert_eq!(Value::Boolean(false), Token::new("~true", &mut state).unwrap().value());
        assert_eq!(Value::Integer(0), Token::new("~255", &mut state).unwrap().value());
        assert_eq!(Value::Integer(3), Token::new("~0b1100", &mut state).unwrap().value());
        assert_eq!(true, Token::new("~1.2", &mut state).is_err());
        assert_eq!(true, Token::new("~'test'", &mut state).is_err());
    }

    #[test]
    fn test_postfix_unary_expression_factorial() {
        assert_token_value!("[0, 2, 4]!", Value::from(vec![
            Value::from(1), Value::from(2), Value::from(24), 
        ]));
        assert_token_value!("0!", Value::from(1));
        assert_token_value!("1!", Value::from(1));
        assert_token_value!("2!", Value::from(2));
        assert_token_value!("4!", Value::from(24));
        assert_token_value!("3!!", Value::from(720));
        assert_token_error!("(-1)!", Underflow);
    }

    #[test]
    fn test_power_expression() {
        assert_token_value!("[2, 2**2, 0]**2", Value::from(vec![
            Value::from(4), Value::from(16), Value::from(0), 
        ]));
        assert_token_value!("2**[0, 1, 2]", Value::from(vec![
            Value::from(1), Value::from(2), Value::from(4), 
        ]));
        assert_token_value!("2**2", Value::from(4));
        assert_token_value!("2**2**2", Value::from(16));
        assert_token_value!("2**2**(2)", Value::from(16));
    }

    #[test]
    fn test_md_expression() {
        assert_token_value!("[2, 4]*2", Value::from(vec![
            Value::from(4), Value::from(8), 
        ]));
        assert_token_value!("2/[2, 4]", Value::from(vec![
            Value::from(1), Value::from(0), 
        ]));
        assert_token_value!("2*2", Value::from(4));
        assert_token_value!("2/2", Value::from(1));
        assert_token_value!("11%10", Value::from(1));
        assert_token_value!("12%10 * 2 / 2", Value::from(2));
        
        
    }

    #[test]
    fn test_implied_mul_expression() {
        let mut state = ParserState::new();

        state.variables.insert("x".to_string(), Value::from(4));
        assert_token_value_stateful!("4x", Value::from(16), &mut state);
        assert_token_error!("x4", VariableName);
        assert_token_value_stateful!("(4)(x)", Value::from(16), &mut state);
        assert_token_value_stateful!("4(x)", Value::from(16), &mut state);
        assert_token_value_stateful!("(4)x", Value::from(16), &mut state);

        assert_token_value!("2[2,4]2", Value::from(vec![Value::from(8), Value::from(16)]));
        assert_token_value!("[2,4][3,3]", Value::from(vec![Value::from(6), Value::from(12)]));
        assert_token_value!("2(2)(2)(2)(2)(2)", Value::from(64));
    }

    #[test]
    fn test_as_expression() {
        
        assert_token_text!("2*$2", "$4.00");
        assert_token_value!("2+2", Value::Integer(4));
        assert_token_value!("2+2+2", Value::Integer(6));
        assert_token_value!("2+2-2/2", Value::Integer(3));
        assert_token_value!("2-[2,4]", Value::from(vec![Value::from(0), Value::from(-2)]));
        assert_token_value!("[2,4] - 2", Value::from(vec![Value::from(0), Value::from(2)]));
        assert_token_value!("[2,4] - [2,3]", Value::from(vec![Value::from(0), Value::from(1)]));
    }
}