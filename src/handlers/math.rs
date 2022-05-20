use crate::token::{Rule, Token};
use crate::value::{Value, IntegerType, FloatType};
use crate::state::ParserState;
use crate::errors::*;

type IntHandler = fn(l:IntegerType, r:IntegerType) -> Option<IntegerType>;
type FloatHandler = fn(l:FloatType, r:FloatType) -> FloatType;

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

/// Perform an integer calculation against 2 values
/// 
/// # Arguments
/// * `l` - Left value
/// * `r` - Right value
/// * `handler` - checked_* function
fn perform_int_calculation(expression: &Token, l: Value, r: Value, handler: IntHandler) -> Result<Value, ParserError> {
    if l.is_array() && r.is_array() {
        let mut la = l.as_array();
        let ra = r.as_array();

        if la.len() != ra.len() {
            Err(ParserError::ArrayLength(ArrayLengthError::new_with_token(expression)))
        } else {
            for (pos, e) in la.clone().iter().enumerate() {
                match perform_int_calculation(expression, e.clone(), ra[pos].clone(), handler) {
                    Ok(n) => la[pos] = n,
                    Err(e) => return Err(e) 
                }
            }
            Ok(Value::Array(la))
        }
    } else if l.is_array() {
        let mut la = l.as_array();
        for (pos, e) in la.clone().iter().enumerate() {
            match perform_int_calculation(expression, e.clone(), r.clone(), handler) {
                Ok(n) => la[pos] = n,
                Err(e) => return Err(e) 
            }
        }
        Ok(Value::Array(la))
    } else if r.is_array() {
        let mut ra = r.as_array();
        for (pos, e) in ra.clone().iter().enumerate() {
            match perform_int_calculation(expression, l.clone(), e.clone(), handler) {
                Ok(n) => ra[pos] = n,
                Err(e) => return Err(e) 
            }
        }
        Ok(Value::Array(ra))
    } else {
        // Perform datatype conversions
        let lv = l.as_int(); let rv = r.as_int();
        if matches!(lv, None) || matches!(rv, None) {
            Err(ParserError::ValueType(ValueTypeError::new_with_token(expression, ExpectedTypes::IntOrFloat)))
        } else {
            // Detect overflow and return resulting value
            match handler(lv.unwrap(), rv.unwrap()) {
                Some(n) => Ok(Value::Integer(n)),
                None => Err(ParserError::Overflow(OverflowError::new_with_token(expression)))
            }
        }
    }
}

/// Perform a floating point calculation against 2 values
/// 
/// # Arguments
/// * `l` - Left value
/// * `r` - Right value
/// * `handler` - checked_* function
fn perform_float_calculation(expression: &Token, l: Value, r: Value, handler: FloatHandler) -> Result<Value, ParserError> {
    if l.is_array() && r.is_array() {
        let mut la = l.as_array();
        let ra = r.as_array();

        if la.len() != ra.len() {
            Err(ParserError::ArrayLength(ArrayLengthError::new_with_token(expression)))
        } else {
            for (pos, e) in la.clone().iter().enumerate() {
                match perform_float_calculation(expression, e.clone(), ra[pos].clone(), handler) {
                    Ok(n) => la[pos] = n,
                    Err(e) => return Err(e) 
                }
            }
            Ok(Value::Array(la))
        }
    } else if l.is_array() {
        let mut la = l.as_array();
        for (pos, e) in la.clone().iter().enumerate() {
            match perform_float_calculation(expression, e.clone(), r.clone(), handler) {
                Ok(n) => la[pos] = n,
                Err(e) => return Err(e) 
            }
        }
        Ok(Value::Array(la))
    } else if r.is_array() {
        let mut ra = r.as_array();
        for (pos, e) in ra.clone().iter().enumerate() {
            match perform_float_calculation(expression, l.clone(), e.clone(), handler) {
                Ok(n) => ra[pos] = n,
                Err(e) => return Err(e) 
            }
        }
        Ok(Value::Array(ra))
    } else {
        // Perform datatype conversions
        let lv = l.as_float(); let rv = r.as_float();
        if matches!(lv, None) || matches!(rv, None) { 
            return Err(ParserError::ValueType(ValueTypeError::new_with_token(expression, ExpectedTypes::IntOrFloat)))
        }
        
        // Detect overflow
        let r = handler(lv.unwrap(), rv.unwrap());
        if r == FloatType::INFINITY {
            return Err(ParserError::Overflow(OverflowError::new_with_token(expression)))
        } else if r == FloatType::NEG_INFINITY {
            return Err(ParserError::Underflow(UnderflowError::new_with_token(expression)))
        }
    
        // Return resulting value
        Ok(Value::Float(r))
    }
}

/// Perform a bitwise calculation against 2 values
/// 
/// # Arguments
/// * `l` - Left value
/// * `r` - Right value
/// * `i_handler` - integer handler function
/// * `f_handler` - float handler function
fn perform_binary_calculation(expression: &Token, l: Value, r: Value, i_handler: IntHandler, f_handler: FloatHandler) -> Result<Value, ParserError> {
    if l.as_array().iter().any(|e| e.is_float()) || r.as_array().iter().any(|e| e.is_float()) {
        match perform_float_calculation(expression, l, r, f_handler) {
            Ok(n) => Ok(n),
            Err(e) => Err(e)
        }
    } else {
        match perform_int_calculation(expression, l, r, i_handler) {
            Ok(n) => Ok(n),
            Err(e) => Err(e)
        }
    }
}

/// Perform a checked factorial
/// 
/// # Arguments
/// * `input` - input value
pub fn factorial(input: IntegerType) -> Option<IntegerType> {
    match input {
        0 => Some(1),
        1.. => {
            let mut acc : IntegerType = 1;
            for i in 1..=input {
                if let Some(acc_) = acc.checked_mul(i as IntegerType) {
                    acc = acc_;
                } else {
                    return None
                }
            }

            Some(acc)
        },

        _ => None
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
        _ => Err(ParserError::ValueType(ValueTypeError::new_with_token(expression, ExpectedTypes::IntOrFloat)))
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
                None => Err(ParserError::ValueType(ValueTypeError::new_with_token(expression, ExpectedTypes::Int)))
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
        _ => Err(ParserError::ValueType(ValueTypeError::new_with_token(expression, ExpectedTypes::Int)))
    }
}

/// Perform a unary factorial
/// 
/// # Arguments
/// * `expression` - Source token
/// * `value` - Value to process
fn unary_factorial(expression: &Token, value: Value) -> Result<Value, ParserError> {
    if let Some(input) = value.as_int() {
        match factorial(input) {
            Some(n) => Ok(Value::Integer(n)),
            None => Err(ParserError::Overflow(OverflowError::new_with_token(expression)))
        }
    } else if value.is_array() {
        let mut ra = value.as_array();
        for (pos, e) in ra.clone().iter().enumerate() {
            match unary_factorial(expression, e.clone()) {
                Ok(n) => ra[pos] = n,
                Err(e) => return Err(e) 
            }
        }
        Ok(Value::Array(ra))
    } else {
        Err(ParserError::ValueType(ValueTypeError::new_with_token(expression, ExpectedTypes::Int)))
    }
}

pub fn math_expression_handler(token: &mut Token, _state: &mut ParserState) -> Option<ParserError> {
    match token.rule() {
        Rule::prefix_unary_expression => {
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
        },

        Rule::postfix_unary_expression => {
            token.set_value(token.child(0).unwrap().value());
            if token.children().len() >= 2 {
                let mut i = 1;
                while i < token.children().len() {
                    if token.child(i).unwrap().rule() == Rule::factorial {
                        match unary_factorial(token, token.value()) {
                            Ok(n) => token.set_value(n),
                            Err(e) => return Some(e)
                        }
                    }

                    i+=1;
                }
            }
        },

        Rule::power_expression => {
            token.set_value(token.child(0).unwrap().value());
        
            if token.children().len() > 1 {
                let mut i = 2;
                while i < token.children().len() {
                    match perform_binary_calculation(token, token.value(), token.child(i).unwrap().value(), integer_type_checked_pow, FloatType::powf) {
                        Ok(n) => token.set_value(n),
                        Err(e) => return Some(e)
                    }
        
                    i += 2;
                }
            }
        
            return None;
        }

        Rule::md_expression => {
            token.set_value(token.child(0).unwrap().value());
        
            if token.children().len() > 1 {
                let mut i = 2;
                while i < token.children().len() {
                    let ih = match token.child(i - 1).unwrap().rule() {
                        Rule::multiply => IntegerType::checked_mul,
                        Rule::divide => IntegerType::checked_div,
                        Rule::modulus => IntegerType::checked_rem_euclid,
                        _ => return Some(ParserError::Pest(PestError::new_with_token(token, "internal error")))
                    };
                    
                    let fh = match token.child(i - 1).unwrap().rule() {
                        Rule::multiply => |l: FloatType, r: FloatType| l * r,
                        Rule::divide => |l: FloatType, r: FloatType| l / r,
                        Rule::modulus => FloatType::rem_euclid,
                        _ => return Some(ParserError::Pest(PestError::new_with_token(token, "internal error")))
                    };

                    match perform_binary_calculation(token, token.value(), token.child(i).unwrap().value(), ih, fh) {
                        Ok(n) => token.set_value(n),
                        Err(e) => return Some(e)
                    }
        
                    i += 2;
                }
            }
        
            return None;
        }

        Rule::implied_mul_expression => {
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

                    match perform_binary_calculation(token, token.value(), token.child(i).unwrap().value(), ih, fh) {
                        Ok(n) => token.set_value(n),
                        Err(e) => return Some(e)
                    }
        
                    i += 1;
                }
            }
            
            return None;
        }

        Rule::as_expression => {
            token.set_value(token.child(0).unwrap().value());
            if token.children().len() > 1 {
                let mut i = 2;
                while i < token.children().len() {
                    match token.child(i - 1).unwrap().rule() {
                        Rule::plus => {
                            if token.value().is_string() || token.child(i).unwrap().value().is_string() {
                                token.set_value(Value::String(format!("{}{}", token.value().as_string(), token.child(i).unwrap().value().as_string())));
                            } else {
                                match perform_binary_calculation(
                                    token, token.value(), token.child(i).unwrap().value(), 
                                    IntegerType::checked_add, |l: FloatType, r: FloatType| l + r
                                ) {
                                    Ok(n) => token.set_value(n),
                                    Err(e) => return Some(e)
                                };
                            }
                        },

                        Rule::minus => {
                            match perform_binary_calculation(
                                token, token.value(), token.child(i).unwrap().value(), 
                                IntegerType::checked_sub, |l: FloatType, r: FloatType| l - r
                            ) {
                                Ok(n) => token.set_value(n),
                                Err(e) => return Some(e)
                            };
                        },

                        _ => return Some(ParserError::Pest(PestError::new_with_token(token, "internal error")))
                    }
        
                    i += 2;
                }
            }
        
            return None;
        }

        _ => { }
    }

    None
}

pub fn bitwise_expression_handler(token: &mut Token, _state: &mut ParserState) -> Option<ParserError> {
    match token.rule() {
        Rule::sh_expression => {
            token.set_value(token.child(0).unwrap().value());
        
            if token.children().len() > 1 {
                let mut i = 2;
                while i < token.children().len() {
                    let ih = match token.child(i - 1).unwrap().rule() {
                        Rule::lshift => |l:IntegerType, r:IntegerType| Some(l << r),
                        Rule::rshift => |l:IntegerType, r:IntegerType| Some(l >> r),
                        _ => return Some(ParserError::Pest(PestError::new_with_token(token, "internal error")))
                    };

                    if token.value().is_float() || token.child(i).unwrap().value().is_float() {
                        return Some(ParserError::ValueType(ValueTypeError::new_with_token(token, ExpectedTypes::IntOrFloat)));
                    }

                    match perform_int_calculation(token, token.value(), token.child(i).unwrap().value(), ih) {
                        Ok(n) => token.set_value(n),
                        Err(e) => return Some(e)
                    }
        
                    i += 2;
                }
            }
        
            return None;
        }

        Rule::and_expression => {
            token.set_value(token.child(0).unwrap().value());
        
            if token.children().len() > 1 {
                let mut i = 2;
                while i < token.children().len() {
                    if token.value().is_float() || token.child(i).unwrap().value().is_float() {
                        return Some(ParserError::ValueType(ValueTypeError::new_with_token(token, ExpectedTypes::IntOrFloat)));
                    }

                    match perform_int_calculation(token, token.value(), token.child(i).unwrap().value(), |l:IntegerType, r:IntegerType| Some(l & r)) {
                        Ok(n) => token.set_value(n),
                        Err(e) => return Some(e)
                    }
        
                    i += 2;
                }
            }
        
            return None;
        }

        Rule::xor_expression => {
            token.set_value(token.child(0).unwrap().value());
        
            if token.children().len() > 1 {
                let mut i = 2;
                while i < token.children().len() {
                    if token.value().is_float() || token.child(i).unwrap().value().is_float() {
                        return Some(ParserError::ValueType(ValueTypeError::new_with_token(token, ExpectedTypes::Int)));
                    }

                    match perform_int_calculation(token, token.value(), token.child(i).unwrap().value(), |l:IntegerType, r:IntegerType| Some(l ^ r)) {
                        Ok(n) => token.set_value(n),
                        Err(e) => return Some(e)
                    }
        
                    i += 2;
                }
            }
        
            return None;
        }

        Rule::or_expression => {
            token.set_value(token.child(0).unwrap().value());
        
            if token.children().len() > 1 {
                let mut i = 2;
                while i < token.children().len() {
                    if token.value().is_float() || token.child(i).unwrap().value().is_float() {
                        return Some(ParserError::ValueType(ValueTypeError::new_with_token(token, ExpectedTypes::Int)));
                    }

                    match perform_int_calculation(token, token.value(), token.child(i).unwrap().value(), |l:IntegerType, r:IntegerType| Some(l | r)) {
                        Ok(n) => token.set_value(n),
                        Err(e) => return Some(e)
                    }
        
                    i += 2;
                }
            }
        
            return None;
        }

        _ => { }
    }

    None
}

#[cfg(test)]
mod test_token {
    use super::*;

    #[test]
    fn test_integer_type_checked_pow() {
        assert_eq!(1, integer_type_checked_pow(10, 0).unwrap());
        assert_eq!(10, integer_type_checked_pow(10, 1).unwrap());
        assert_eq!(100, integer_type_checked_pow(10, 2).unwrap());
        assert_eq!(0, integer_type_checked_pow(100, -1).unwrap());
    }

    #[test]
    fn test_perform_int_calculation() {
        let mut state = ParserState::new();
        assert_eq!(
            Value::Integer(1), 
            perform_int_calculation(&Token::new("2 - 1", &mut state).unwrap(), 
                Value::Integer(2), 
                Value::Integer(1), 
                |l,r| Some(l-r)
            ).unwrap()
        );
        
        assert_eq!(
            Value::Array(vec![Value::Integer(1), Value::Integer(1)]), 
            perform_int_calculation(&Token::new("[2, 2] - 1", &mut state).unwrap(), 
                Value::Array(vec![Value::Integer(2), Value::Integer(2)]), 
                Value::Integer(1), 
                |l,r| Some(l-r)
            ).unwrap()
        );
        
        assert_eq!(
            Value::Array(vec![Value::Integer(-1), Value::Integer(-1)]), 
            perform_int_calculation(&Token::new("1 - [2, 2]", &mut state).unwrap(), 
                Value::Integer(1), 
                Value::Array(vec![Value::Integer(2), Value::Integer(2)]), 
                |l,r| Some(l-r)
            ).unwrap()
        );
        
        assert_eq!(
            Value::Array(vec![Value::Integer(1), Value::Integer(1)]), 
            perform_int_calculation(&Token::new("[2, 2] - [1, 1]", &mut state).unwrap(), 
            Value::Array(vec![Value::Integer(2), Value::Integer(2)]), 
                Value::Array(vec![Value::Integer(1), Value::Integer(1)]), 
                |l,r| Some(l-r)
            ).unwrap()
        );
    }

    #[test]
    fn test_perform_float_calculation() {
        let mut state = ParserState::new();

        assert_eq!(
            Value::Float(1.0), 
            perform_float_calculation(&Token::new("2.0 - 1.0", &mut state).unwrap(), 
                Value::Float(2.0), 
                Value::Float(1.0), 
                |l,r| l-r
            ).unwrap()
        );
        
        assert_eq!(
            Value::Array(vec![Value::Float(1.0), Value::Float(1.0)]), 
            perform_float_calculation(&Token::new("[2, 2] - 1", &mut state).unwrap(), 
                Value::Array(vec![Value::Integer(2), Value::Float(2.0)]), 
                Value::Integer(1), 
                |l,r| l-r
            ).unwrap()
        );
        
        assert_eq!(
            Value::Array(vec![Value::Float(-1.0), Value::Float(-1.0)]), 
            perform_float_calculation(&Token::new("1.0 - [2, 2]", &mut state).unwrap(), 
                Value::Float(1.0), 
                Value::Array(vec![Value::Integer(2), Value::Integer(2)]), 
                |l,r| l-r
            ).unwrap()
        );
        
        assert_eq!(
            Value::Array(vec![Value::Float(1.0), Value::Float(1.0)]), 
            perform_float_calculation(&Token::new("[2, 2] - [1, 1.0]", &mut state).unwrap(), 
                Value::Array(vec![Value::Integer(2), Value::Integer(2)]), 
                Value::Array(vec![Value::Integer(1), Value::Float(1.0)]), 
                |l,r| l-r
            ).unwrap()
        );
    }

    #[test]
    fn test_perform_binary_calculation() {
        let mut state = ParserState::new();
        let token = Token::new("1.0 + 1.0", &mut state).unwrap();
        assert_eq!(Value::Array(vec![Value::Integer(1), 
            Value::Integer(1)]), 
            perform_binary_calculation(&token, 
                Value::Array(vec![Value::Integer(2), Value::Integer(2)]), 
                Value::Integer(1), 
                |l,r| Some(l-r), |l,r| l-r
            ).unwrap()
        );
        assert_eq!(Value::Integer(1), perform_binary_calculation(&token, Value::Integer(2), Value::Integer(1), |l,r| Some(l-r), |l,r| l-r).unwrap());
        assert_eq!(Value::Float(1.0), perform_binary_calculation(&token, Value::Integer(2), Value::Float(1.0), |l,r| Some(l-r), |l,r| l-r).unwrap());
        assert_eq!(Value::Float(1.0), perform_binary_calculation(&token, Value::Float(2.0), Value::Integer(1), |l,r| Some(l-r), |l,r| l-r).unwrap());
        assert_eq!(Value::Float(1.0), perform_binary_calculation(&token, Value::Float(2.0), Value::Float(1.0), |l,r| Some(l-r), |l,r| l-r).unwrap());
    }

    #[test]
    fn test_factorial() {
        assert_eq!(1, factorial(0).unwrap());
        assert_eq!(1, factorial(1).unwrap());
        assert_eq!(2, factorial(2).unwrap());
        assert_eq!(24, factorial(4).unwrap());
        assert_eq!(true, factorial(-1).is_none());
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
        let mut state = ParserState::new();
        assert_eq!(Value::Array(vec![
            Value::Integer(1), Value::Integer(2), Value::Integer(24), 
        ]), Token::new("[0, 2, 4]!", &mut state).unwrap().value());
        assert_eq!(Value::Integer(1), Token::new("0!", &mut state).unwrap().value());
        assert_eq!(Value::Integer(1), Token::new("1!", &mut state).unwrap().value());
        assert_eq!(Value::Integer(2), Token::new("2!", &mut state).unwrap().value());
        assert_eq!(Value::Integer(24), Token::new("4!", &mut state).unwrap().value());
        assert_eq!(true, Token::new("(-1)!", &mut state).is_err());
    }

    #[test]
    fn test_power_expression() {
        let mut state = ParserState::new();
        assert_eq!(Value::Array(vec![
            Value::Integer(4), Value::Integer(16), Value::Integer(0), 
        ]), Token::new("[2, 2**2, 0]**2", &mut state).unwrap().value());
        assert_eq!(Value::Array(vec![
            Value::Integer(1), Value::Integer(2), Value::Integer(4), 
        ]), Token::new("2**[0, 1, 2]", &mut state).unwrap().value());
        assert_eq!(Value::Integer(4), Token::new("2**2", &mut state).unwrap().value());
        assert_eq!(Value::Integer(16), Token::new("2**2**2", &mut state).unwrap().value());
        assert_eq!(Value::Integer(16), Token::new("2**2**(2)", &mut state).unwrap().value());
    }

    #[test]
    fn test_md_expression() {
        let mut state = ParserState::new();
        assert_eq!(Value::Array(vec![
            Value::Integer(4), Value::Integer(8), 
        ]), Token::new("2*[2, 4]", &mut state).unwrap().value());
        assert_eq!(Value::Array(vec![
            Value::Integer(1), Value::Integer(0), 
        ]), Token::new("2/[2, 4]", &mut state).unwrap().value());
        assert_eq!(Value::Integer(4), Token::new("2*2", &mut state).unwrap().value());
        assert_eq!(Value::Integer(1), Token::new("2/2", &mut state).unwrap().value());
        assert_eq!(Value::Integer(1), Token::new("11%10", &mut state).unwrap().value());
        assert_eq!(Value::Integer(2), Token::new("12%10 * 2 / 2", &mut state).unwrap().value());
    }

    #[test]
    fn test_implied_mul_expression() {
        let mut state = ParserState::new();
        Token::new("x=4", &mut state).unwrap();
        assert_eq!(Value::Array(vec![
            Value::Integer(4), Value::Integer(4), 
        ]), Token::new("(2)([2,2])", &mut state).unwrap().value());
        assert_eq!(Value::Integer(16), Token::new("4x", &mut state).unwrap().value());
        assert_eq!(Value::Integer(16), Token::new("(4)(x)", &mut state).unwrap().value());
        assert_eq!(Value::Integer(16), Token::new("4(x)", &mut state).unwrap().value());
        assert_eq!(Value::Integer(16), Token::new("(4)x", &mut state).unwrap().value());
        assert_eq!(Value::Integer(64), Token::new("2(2)(2)(2)(2)(2)", &mut state).unwrap().value());
    }

    #[test]
    fn test_as_expression() {
        let mut state = ParserState::new();
        assert_eq!(Value::Array(vec![
            Value::Integer(4), Value::Integer(6), 
        ]), Token::new("2+[2, 4]", &mut state).unwrap().value());
        assert_eq!(Value::Array(vec![
            Value::Integer(0), Value::Integer(-2), 
        ]), Token::new("2-[2, 4]", &mut state).unwrap().value());
        assert_eq!(Value::Integer(4), Token::new("2+2", &mut state).unwrap().value());
        assert_eq!(Value::Integer(0), Token::new("2-2", &mut state).unwrap().value());
        assert_eq!(Value::Integer(2), Token::new("2 - 2 + 2", &mut state).unwrap().value());
    }

    #[test]
    fn test_sh_expression() {
        let mut state = ParserState::new();
        assert_eq!(Value::Array(vec![
            Value::Integer(2), Value::Integer(1), 
        ]), Token::new("4 >> [1,2]", &mut state).unwrap().value());
        assert_eq!(Value::Integer(2), Token::new("4 >> 1", &mut state).unwrap().value());
        assert_eq!(Value::Integer(8), Token::new("2 << 2", &mut state).unwrap().value());
        assert_eq!(Value::Integer(2), Token::new("2 << 2 >> 2", &mut state).unwrap().value());
    }

    #[test]
    fn test_and_expression() {
        let mut state = ParserState::new();
        assert_eq!(Value::Array(vec![
            Value::Integer(15), Value::Integer(0), 
        ]), Token::new("0xFF & [0x0F, 0]", &mut state).unwrap().value());
        assert_eq!(Value::Integer(15), Token::new("0xFF & 0x0F", &mut state).unwrap().value());
        assert_eq!(Value::Integer(8), Token::new("0b1100 & 0b1110 & 0b1000", &mut state).unwrap().value());
    }

    #[test]
    fn test_xor_expression() {
        let mut state = ParserState::new();
        assert_eq!(Value::Array(vec![
            Value::Integer(240), Value::Integer(255), 
        ]), Token::new("0xFF ^ [0x0F, 0]", &mut state).unwrap().value());
        assert_eq!(Value::Integer(240), Token::new("0xFF ^ 0x0F", &mut state).unwrap().value());
        assert_eq!(Value::Integer(80), Token::new("0xFF ^ 0x0F ^ 0xA0", &mut state).unwrap().value());
    }

    #[test]
    fn test_or_expression() {
        let mut state = ParserState::new();
        assert_eq!(Value::Array(vec![
            Value::Integer(255), Value::Integer(255), 
        ]), Token::new("0xFF | [0x0F, 0]", &mut state).unwrap().value());
        assert_eq!(Value::Integer(255), Token::new("0xF0 | 0x0F", &mut state).unwrap().value());
        assert_eq!(Value::Integer(15), Token::new("0b1100 | 0b1110 | 0b1", &mut state).unwrap().value());
    }
}