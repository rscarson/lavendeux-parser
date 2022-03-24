use crate::token::{Rule, Token};
use crate::value::{AtomicValue, IntegerType, FloatType};
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
fn perform_int_calculation(l: AtomicValue, r: AtomicValue, handler: IntHandler) -> Result<AtomicValue, ParserError> {
    // Perform datatype conversions
    let lv = l.as_int(); let rv = r.as_int();
    if matches!(lv, None) || matches!(rv, None) {
        return Err(ParserError::ValueType(ValueTypeError {
            expected: ExpectedTypes::IntOrFloat
        }))
    }
    
    // Detect overflow and return resulting value
    match handler(lv.unwrap(), rv.unwrap()) {
        Some(n) => Ok(AtomicValue::Integer(n)),
        None => Err(ParserError::Overflow(OverflowError{}))
    }
}

/// Perform a floating point calculation against 2 values
/// 
/// # Arguments
/// * `l` - Left value
/// * `r` - Right value
/// * `handler` - checked_* function
fn perform_float_calculation(l: AtomicValue, r: AtomicValue, handler: FloatHandler) -> Result<AtomicValue, ParserError> {
    // Perform datatype conversions
    let lv = l.as_float(); let rv = r.as_float();
    if matches!(lv, None) || matches!(rv, None) { 
        return Err(ParserError::ValueType(ValueTypeError {
            expected: ExpectedTypes::IntOrFloat
        }))
    }
    
    // Detect overflow
    let r = handler(lv.unwrap(), rv.unwrap());
    if r == FloatType::INFINITY {
        return Err(ParserError::Overflow(OverflowError{}))
    } else if r == FloatType::NEG_INFINITY {
        return Err(ParserError::Underflow(UnderflowError{}))
    }

    // Return resulting value
    Ok(AtomicValue::Float(r))
}

/// Perform a bitwise calculation against 2 values
/// 
/// # Arguments
/// * `l` - Left value
/// * `r` - Right value
/// * `i_handler` - integer handler function
/// * `f_handler` - float handler function
fn perform_binary_calculation(l: AtomicValue, r: AtomicValue, i_handler: IntHandler, f_handler: FloatHandler) -> Result<AtomicValue, ParserError> {
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
pub fn factorial(input: AtomicValue) -> Option<AtomicValue> {
    match input.as_int() {
        Some(n) => {
            match n {
                0  => Some(AtomicValue::Integer(1)),
                1.. => {
                    match panic::catch_unwind(|| {
                        (1..n+1).product()
                    }) {
                        Ok(p) => Some(AtomicValue::Integer(p)),
                        Err(_) => None
                    }
                },
                _ => factorial(AtomicValue::Integer(-n))
            }
        },
        None => None
    }
}

/// Trim a binary value to match the precision of a given base. Useful for inversion
/// 
/// # Arguments
/// * `input` - Source value
/// * `base` - Number to check against
fn trim_binary(input: AtomicValue, base: IntegerType) -> Option<AtomicValue> {
    match input.as_int() {
        Some(n) => {
            let mask : IntegerType = ((2_u32).pow( ((base as FloatType).log2().floor() + 1.0) as u32) - 1) as IntegerType;
            Some(AtomicValue::Integer(n & if mask==0 {!mask} else {mask}))
        },
        None => None
    }
}

pub fn math_expression_handler(token: &mut Token, _state: &mut ParserState) -> Option<ParserError> {
    match token.rule {
        Rule::prefix_unary_expression => {
            if token.children.len() >= 2 {
                let mut idx = token.children.len() - 1;
                token.value = token.children[idx].value.clone();
                while idx >0 {
                    idx-=1;

                    if token.children[idx].rule == Rule::minus {
                        match token.value {
                            AtomicValue::Integer(n) => token.value = AtomicValue::Integer(-n),
                            AtomicValue::Float(n) => token.value = AtomicValue::Float(-n),
                            _ => return Some(ParserError::ValueType(ValueTypeError {
                                expected: ExpectedTypes::IntOrFloat
                            }))
                        }
                    } else if token.children[idx].rule == Rule::not {
                        match token.value {
                            AtomicValue::Integer(n) => {
                                match trim_binary(AtomicValue::Integer(!n), n) {
                                    Some(v) => token.value = v,
                                    None => return Some(ParserError::ValueType(ValueTypeError {
                                        expected: ExpectedTypes::Int
                                    }))
                                }
                            },
                            AtomicValue::Boolean(n) => {
                                token.value = AtomicValue::Boolean(!n);
                            },
                            _ => return Some(ParserError::ValueType(ValueTypeError {
                                expected: ExpectedTypes::Int
                            }))
                        }
                    }
                }
            }
        },

        Rule::postfix_unary_expression => {
            token.value = token.children[0].value.clone();
            if token.children.len() >= 2 {
                let mut i = 1;
                while i < token.children.len() {
                    if token.children[i].rule == Rule::factorial {
                        match factorial(token.value.clone()) {
                            Some(n) => token.value = n,
                            None => return Some(ParserError::ValueType(ValueTypeError {
                                expected: ExpectedTypes::IntOrFloat
                            }))
                        }
                    }

                    i+=1;
                }
            }
        },

        Rule::power_expression => {
            token.value = token.children[0].value.clone();
        
            if token.children.len() > 1 {
                let mut i = 2;
                while i < token.children.len() {
                    match perform_binary_calculation(token.value.clone(), token.children[i].value.clone(), integer_type_checked_pow, FloatType::powf) {
                        Ok(n) => token.value = n,
                        Err(e) => return Some(e)
                    }
        
                    i += 2;
                }
            }
        
            return None;
        }

        Rule::md_expression => {
            token.value = token.children[0].value.clone();
        
            if token.children.len() > 1 {
                let mut i = 2;
                while i < token.children.len() {
                    let ih = match token.children[i - 1].rule {
                        Rule::multiply => IntegerType::checked_mul,
                        Rule::divide => IntegerType::checked_div,
                        Rule::modulus => IntegerType::checked_rem_euclid,
                        _ => return Some(ParserError::Pest(PestError::new("internal error")))
                    };
                    
                    let fh = match token.children[i - 1].rule {
                        Rule::multiply => |l: FloatType, r: FloatType| l * r,
                        Rule::divide => |l: FloatType, r: FloatType| l / r,
                        Rule::modulus => FloatType::rem_euclid,
                        _ => return Some(ParserError::Pest(PestError::new("internal error")))
                    };

                    match perform_binary_calculation(token.value.clone(), token.children[i].value.clone(), ih, fh) {
                        Ok(n) => token.value = n,
                        Err(e) => return Some(e)
                    }
        
                    i += 2;
                }
            }
        
            return None;
        }

        Rule::as_expression => {
            token.value = token.children[0].value.clone();
            if token.children.len() > 1 {
                let mut i = 2;
                while i < token.children.len() {
                    match token.children[i - 1].rule {
                        Rule::plus => {
                            if token.value.is_string() || token.children[i].value.is_string() {
                                token.value = AtomicValue::String(format!("{}{}", token.value.as_string(), token.children[i].value.as_string()));
                            } else {
                                match perform_binary_calculation(
                                    token.value.clone(), token.children[i].value.clone(), 
                                    IntegerType::checked_add, |l: FloatType, r: FloatType| l + r
                                ) {
                                    Ok(n) => token.value = n,
                                    Err(e) => return Some(e)
                                };
                            }
                        },

                        Rule::minus => {
                            match perform_binary_calculation(
                                token.value.clone(), token.children[i].value.clone(), 
                                IntegerType::checked_sub, |l: FloatType, r: FloatType| l - r
                            ) {
                                Ok(n) => token.value = n,
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
    match token.rule {
        Rule::sh_expression => {
            token.value = token.children[0].value.clone();
        
            if token.children.len() > 1 {
                let mut i = 2;
                while i < token.children.len() {
                    let ih = match token.children[i - 1].rule {
                        Rule::lshift => |l:IntegerType, r:IntegerType| Some(l << r),
                        Rule::rshift => |l:IntegerType, r:IntegerType| Some(l >> r),
                        _ => return Some(ParserError::Pest(PestError::new("internal error")))
                    };

                    if token.value.is_float() || token.children[i].value.is_float() {
                        return Some(ParserError::ValueType(ValueTypeError {
                            expected: ExpectedTypes::Int
                        }));
                    }

                    match perform_int_calculation(token.value.clone(), token.children[i].value.clone(), ih) {
                        Ok(n) => token.value = n,
                        Err(e) => return Some(e)
                    }
        
                    i += 2;
                }
            }
        
            return None;
        }

        Rule::and_expression => {
            token.value = token.children[0].value.clone();
        
            if token.children.len() > 1 {
                let mut i = 2;
                while i < token.children.len() {
                    if token.value.is_float() || token.children[i].value.is_float() {
                        return Some(ParserError::ValueType(ValueTypeError {
                            expected: ExpectedTypes::Int
                        }));
                    }

                    match perform_int_calculation(token.value.clone(), token.children[i].value.clone(), |l:IntegerType, r:IntegerType| Some(l & r)) {
                        Ok(n) => token.value = n,
                        Err(e) => return Some(e)
                    }
        
                    i += 2;
                }
            }
        
            return None;
        }

        Rule::xor_expression => {
            token.value = token.children[0].value.clone();
        
            if token.children.len() > 1 {
                let mut i = 2;
                while i < token.children.len() {
                    if token.value.is_float() || token.children[i].value.is_float() {
                        return Some(ParserError::ValueType(ValueTypeError {
                            expected: ExpectedTypes::Int
                        }));
                    }

                    match perform_int_calculation(token.value.clone(), token.children[i].value.clone(), |l:IntegerType, r:IntegerType| Some(l ^ r)) {
                        Ok(n) => token.value = n,
                        Err(e) => return Some(e)
                    }
        
                    i += 2;
                }
            }
        
            return None;
        }

        Rule::or_expression => {
            token.value = token.children[0].value.clone();
        
            if token.children.len() > 1 {
                let mut i = 2;
                while i < token.children.len() {
                    if token.value.is_float() || token.children[i].value.is_float() {
                        return Some(ParserError::ValueType(ValueTypeError {
                            expected: ExpectedTypes::Int
                        }));
                    }

                    match perform_int_calculation(token.value.clone(), token.children[i].value.clone(), |l:IntegerType, r:IntegerType| Some(l | r)) {
                        Ok(n) => token.value = n,
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