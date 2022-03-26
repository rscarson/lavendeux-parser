use crate::token::{Rule, Token};
use crate::value::{Value, IntegerType, FloatType};
use crate::state::ParserState;
use crate::errors::*;
use std::panic;

type IntHandler = fn(l:IntegerType, r:IntegerType) -> Option<IntegerType>;
type FloatHandler = fn(l:FloatType, r:FloatType) -> FloatType;

/// Perform overflow checked exponentiation
/// 
/// # Arguments
/// * `l` - Left value
/// * `r` - Right value
fn integer_type_checked_pow(l:IntegerType, r:IntegerType) -> Option<IntegerType> {
    if r > u32::MAX as IntegerType { return None; }
    match l.checked_pow(r as u32) {
        Some(mut v) => {
            if r<0 { v = 1/v; }
            Some(v)
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
fn perform_int_calculation(l: Value, r: Value, handler: IntHandler) -> Result<Value, ParserError> {
    // Perform datatype conversions
    let lv = l.as_int(); let rv = r.as_int();
    if matches!(lv, None) || matches!(rv, None) {
        return Err(ParserError::ValueType(ValueTypeError::new(ExpectedTypes::IntOrFloat)))
    }
    
    // Detect overflow and return resulting value
    match handler(lv.unwrap(), rv.unwrap()) {
        Some(n) => Ok(Value::Integer(n)),
        None => Err(ParserError::Overflow(OverflowError{}))
    }
}

/// Perform a floating point calculation against 2 values
/// 
/// # Arguments
/// * `l` - Left value
/// * `r` - Right value
/// * `handler` - checked_* function
fn perform_float_calculation(l: Value, r: Value, handler: FloatHandler) -> Result<Value, ParserError> {
    // Perform datatype conversions
    let lv = l.as_float(); let rv = r.as_float();
    if matches!(lv, None) || matches!(rv, None) { 
        return Err(ParserError::ValueType(ValueTypeError::new(ExpectedTypes::IntOrFloat)))
    }
    
    // Detect overflow
    let r = handler(lv.unwrap(), rv.unwrap());
    if r == FloatType::INFINITY {
        return Err(ParserError::Overflow(OverflowError{}))
    } else if r == FloatType::NEG_INFINITY {
        return Err(ParserError::Underflow(UnderflowError{}))
    }

    // Return resulting value
    Ok(Value::Float(r))
}

/// Perform a bitwise calculation against 2 values
/// 
/// # Arguments
/// * `l` - Left value
/// * `r` - Right value
/// * `i_handler` - integer handler function
/// * `f_handler` - float handler function
fn perform_binary_calculation(l: Value, r: Value, i_handler: IntHandler, f_handler: FloatHandler) -> Result<Value, ParserError> {
    if l.is_float() || r.is_float() {
        match perform_float_calculation(l, r, f_handler) {
            Ok(n) => Ok(n),
            Err(e) => Err(e)
        }
    } else {
        match perform_int_calculation(l, r, i_handler) {
            Ok(n) => Ok(n),
            Err(e) => Err(e)
        }
    }
}

/// Perform a factorial
/// 
/// # Arguments
/// * `input` - input value
pub fn factorial(input: Value) -> Result<Value, ParserError> {
    if let Some(n) = input.as_int() {
        match n {
            0  => Ok(Value::Integer(1)),
            1.. => {
                match panic::catch_unwind(|| {
                    (1..n+1).product()
                }) {
                    Ok(p) => Ok(Value::Integer(p)),
                    Err(_) => Err(ParserError::Overflow(
                        OverflowError::new()
                    ))
                }
            },
            _ => factorial(Value::Integer(-n))
        }
    } else {
        Err(ParserError::ValueType(ValueTypeError::new(ExpectedTypes::IntOrFloat)))
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

pub fn math_expression_handler(token: &mut Token, _state: &mut ParserState) -> Option<ParserError> {
    match token.rule() {
        Rule::prefix_unary_expression => {
            if token.children().len() >= 2 {
                let mut idx = token.children().len() - 1;
                token.set_value(token.child(idx).unwrap().value());
                while idx >0 {
                    idx-=1;

                    if token.child(idx).unwrap().rule() == Rule::minus {
                        match token.value() {
                            Value::Integer(n) => token.set_value(Value::Integer(-n)),
                            Value::Float(n) => token.set_value(Value::Float(-n)),
                            _ => return Some(ParserError::ValueType(ValueTypeError::new(ExpectedTypes::IntOrFloat)))
                        }
                    } else if token.child(idx).unwrap().rule() == Rule::not {
                        match token.value() {
                            Value::Integer(n) => {
                                match trim_binary(Value::Integer(!n), n) {
                                    Some(v) => token.set_value(v),
                                    None => return Some(ParserError::ValueType(ValueTypeError::new(ExpectedTypes::IntOrFloat)))
                                }
                            },
                            Value::Boolean(n) => {
                                token.set_value(Value::Boolean(!n));
                            },
                            _ => return Some(ParserError::ValueType(ValueTypeError::new(ExpectedTypes::IntOrFloat)))
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
                        match factorial(token.value()) {
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
                    match perform_binary_calculation(token.value(), token.child(i).unwrap().value(), integer_type_checked_pow, FloatType::powf) {
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
                        _ => return Some(ParserError::Pest(PestError::new("internal error")))
                    };
                    
                    let fh = match token.child(i - 1).unwrap().rule() {
                        Rule::multiply => |l: FloatType, r: FloatType| l * r,
                        Rule::divide => |l: FloatType, r: FloatType| l / r,
                        Rule::modulus => FloatType::rem_euclid,
                        _ => return Some(ParserError::Pest(PestError::new("internal error")))
                    };

                    match perform_binary_calculation(token.value(), token.child(i).unwrap().value(), ih, fh) {
                        Ok(n) => token.set_value(n),
                        Err(e) => return Some(e)
                    }
        
                    i += 2;
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
                                    token.value(), token.child(i).unwrap().value(), 
                                    IntegerType::checked_add, |l: FloatType, r: FloatType| l + r
                                ) {
                                    Ok(n) => token.set_value(n),
                                    Err(e) => return Some(e)
                                };
                            }
                        },

                        Rule::minus => {
                            match perform_binary_calculation(
                                token.value(), token.child(i).unwrap().value(), 
                                IntegerType::checked_sub, |l: FloatType, r: FloatType| l - r
                            ) {
                                Ok(n) => token.set_value(n),
                                Err(e) => return Some(e)
                            };
                        },

                        _ => return Some(ParserError::Pest(PestError::new("internal error")))
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
                        _ => return Some(ParserError::Pest(PestError::new("internal error")))
                    };

                    if token.value().is_float() || token.child(i).unwrap().value().is_float() {
                        return Some(ParserError::ValueType(ValueTypeError::new(ExpectedTypes::IntOrFloat)));
                    }

                    match perform_int_calculation(token.value(), token.child(i).unwrap().value(), ih) {
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
                        return Some(ParserError::ValueType(ValueTypeError::new(ExpectedTypes::IntOrFloat)));
                    }

                    match perform_int_calculation(token.value(), token.child(i).unwrap().value(), |l:IntegerType, r:IntegerType| Some(l & r)) {
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
                        return Some(ParserError::ValueType(ValueTypeError::new(ExpectedTypes::Int)));
                    }

                    match perform_int_calculation(token.value(), token.child(i).unwrap().value(), |l:IntegerType, r:IntegerType| Some(l ^ r)) {
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
                        return Some(ParserError::ValueType(ValueTypeError::new(ExpectedTypes::Int)));
                    }

                    match perform_int_calculation(token.value(), token.child(i).unwrap().value(), |l:IntegerType, r:IntegerType| Some(l | r)) {
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