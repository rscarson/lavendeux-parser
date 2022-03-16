use super::token::{Rule, Token};
use super::value::{AtomicValue, IntegerType, FloatType};
use super::state::ParserState;
use super::errors::*;

type IntHandler = fn(l:IntegerType, r:IntegerType) -> Option<IntegerType>;
type FloatHandler = fn(l:FloatType, r:FloatType) -> FloatType;


pub fn factorial(input: AtomicValue) -> Option<AtomicValue> {
    match input.as_int() {
        Some(n) => {
            match n {
                0  => Some(AtomicValue::Integer(1)),
                1.. => Some(AtomicValue::Integer((1..n+1).product())),
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
            let mask : IntegerType = ((2 as u32).pow( ((base as FloatType).log2().floor() + 1.0) as u32) - 1) as IntegerType;
            Some(AtomicValue::Integer(n & if mask==0 {!mask} else {mask}))
        },
        None => None
    }
}

/// Parse a string as an integer of a given base
/// 
/// # Arguments
/// * `input` - Source string
/// * `prefix` - Number prefix to remove from the string
/// * `base` - Numeric base
fn parse_radix(input: &String, prefix: &[&str], base: u32) -> Result<IntegerType, ParserError> {
    let mut trimmed = input.to_string();
    for p in prefix {
        trimmed = trimmed.trim_start_matches(p).to_string();
    }
    
    match IntegerType::from_str_radix(&trimmed, base) {
        Ok(n) => Ok(n),
        Err(e) => Err(ParserError::ParseInt(e))
    }
}

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
            return Some(v);
        },
        None => return None
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
        Some(n) => return Ok(AtomicValue::Integer(n)),
        None => return Err(ParserError::Overflow(OverflowError{}))
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
    return Ok(AtomicValue::Float(r));
}

fn perform_binary_calculation(l: AtomicValue, r: AtomicValue, i_handler: IntHandler, f_handler: FloatHandler) -> Result<AtomicValue, ParserError> {
    if l.is_float() || r.is_float() {
        match perform_float_calculation(l.clone(), r.clone(), f_handler) {
            Ok(n) => Ok(n),
            Err(e) => return Err(e)
        }
    } else {
        match perform_int_calculation(l.clone(), r.clone(), i_handler) {
            Ok(n) => Ok(n),
            Err(e) => return Err(e)
        }
    }
}

fn atomicvalue_handler(token: &mut Token, state: &mut ParserState) -> Option<ParserError> {
    match token.rule {
        Rule::hex => {
            match parse_radix(&token.text, &["0x","0X"], 16) {
                Ok(n) => token.value = AtomicValue::Integer(n),
                Err(e) => return Some(e)
            }
        },

        Rule::bin => {
            match parse_radix(&token.text, &["0b","0B"], 2) {
                Ok(n) => token.value = AtomicValue::Integer(n),
                Err(e) => return Some(e)
            }
        },

        Rule::oct => {
            match parse_radix(&token.text, &["0o","0O"], 8) {
                Ok(n) => token.value = AtomicValue::Integer(n),
                Err(e) => return Some(e)
            }
        },

        Rule::sci|Rule::float => match token.text.replace(",", "").parse::<FloatType>() {
            Ok(n) => token.value = AtomicValue::Float(n),
            Err(e) => return Some(ParserError::ParseFloat(e)),
        },

        Rule::int => match token.text.replace(",", "").parse::<IntegerType>() {
            Ok(n) => token.value = AtomicValue::Integer(n),
            Err(e) => return Some(ParserError::ParseInt(e)),
        },

        Rule::string => {
            token.value = AtomicValue::String(
                token.text[1..token.text.len()-1].to_string()
                .replace("\\'", "\'")
                .replace("\\\"", "\"")
                .replace("\\n", "\n")
                .replace("\\r", "\r")
                .replace("\\t", "\t")
            );
        },

        Rule::identifier => {
            match state.constants.get(&token.text) {
                Some(v) => token.value = v.clone(),
                None => match state.variables.get(&token.text) {
                    Some(v) => token.value = v.clone(),
                    None => { }
                }
            }
        },
        
        Rule::atomic_value => {
            token.value = token.children[0].value.clone();
            if matches!(token.value, AtomicValue::None) {
                return Some(ParserError::VariableName(VariableNameError {
                    name: token.text.clone()
                }));
            }
        },

        _ => { }
    }

    return None;
}

fn expression_handler(token: &mut Token, state: &mut ParserState) -> Option<ParserError> {
    match token.rule {
        Rule::script => {
            token.text = token.children.clone().into_iter().map(|t| t.text).collect::<Vec<String>>().join("\n");
        },

        Rule::line => {
            token.value = token.children[0].value.clone();

            if token.children.len() > 2 {
                let name = &token.children[2].text;
                match state.decorators.call(&name, &token.value) {
                    Ok(s) => token.text = s,
                    Err(e) => {
                        for extension in &mut state.extensions {
                            if extension.has_decorator(&name) {
                                match extension.call_decorator(&name, &token.value) {
                                    Ok(s) => {
                                        token.text = s;
                                        return None;
                                    },
                                    Err(e) => return Some(e)
                                }
                            }
                        }

                        return Some(e);
                    }
                }
            } else {
                match state.decorators.call("default", &token.value) {
                    Ok(s) => token.text = s,
                    Err(e) => return Some(e)
                }
            }
        },

        Rule::term => {
            if token.children.len() == 3 {
                token.value = token.children[1].value.clone();
            } else if token.children.len() == 1 {
                token.value = token.children[0].value.clone();
            }
        },

        Rule::assignment_expression => {
            if token.children.len() == 1 {
                token.value = token.children[0].value.clone();
            } else if state.constants.contains_key(&token.children[0].text.to_string()) {
                return Some(ParserError::ContantValue(ConstantValueError {
                    name: token.children[0].text.clone()
                }))
            } else {
                state.variables.insert(token.children[0].text.to_string(), token.children[2].value.clone());
                token.value = token.children[2].value.clone();
            }
        },
        Rule::prefix_unary_expression => {
            if token.children.len() == 1 {
                token.value = token.children[0].value.clone();
            } else if token.children.len() >= 2 {
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
                        Rule::multiply => |l: FloatType, r: FloatType| return l * r,
                        Rule::divide => |l: FloatType, r: FloatType| return l / r,
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
                                    IntegerType::checked_add, |l: FloatType, r: FloatType| return l + r
                                ) {
                                    Ok(n) => token.value = n,
                                    Err(e) => return Some(e)
                                };
                            }
                        },

                        Rule::minus => {
                            match perform_binary_calculation(
                                token.value.clone(), token.children[i].value.clone(), 
                                IntegerType::checked_sub, |l: FloatType, r: FloatType| return l - r
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

    return None;
}

fn call_expression_handler(token: &mut Token, state: &mut ParserState) -> Option<ParserError> {
    match token.rule {
        Rule::call_expression => {
            if token.children.len() == 1 {
                token.value = token.children[0].value.clone();
            } else {
                let name = token.children[0].text.to_string();
                let mut args : Vec<AtomicValue> = Vec::new();
                match token.children[2].rule {
                    Rule::or_expression => args.push(token.children[2].value.clone()),
                    Rule::expression_list => {
                        let mut i = 0;
                        while i < token.children[2].children.len() {
                            args.push(token.children[2].children[i].value.clone());
                            i += 2;
                        }
                    },
                    _ => {}
                }

                match state.functions.call(&name, &args[..]) {
                    Ok(v) => token.value = v,
                    Err(e) => {
                        for extension in &mut state.extensions {
                            if extension.has_function(&name) {
                                match extension.call_function(&name, &args[..]) {
                                    Ok(v) => {
                                        token.value = v;
                                        return None;
                                    },
                                    Err(e) => return Some(e)
                                }
                            }
                        }

                        return Some(e);
                    }
                }
            }
        }

        _ => { }
    }

    return None;
}

fn bitwise_expression_handler(token: &mut Token, _state: &mut ParserState) -> Option<ParserError> {
    match token.rule {
        Rule::sh_expression => {
            token.value = token.children[0].value.clone();
        
            if token.children.len() > 1 {
                let mut i = 2;
                while i < token.children.len() {
                    let ih = match token.children[i - 1].rule {
                        Rule::lshift => |l:IntegerType, r:IntegerType| return Some(l << r),
                        Rule::rshift => |l:IntegerType, r:IntegerType| return Some(l >> r),
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

                    match perform_int_calculation(token.value.clone(), token.children[i].value.clone(), |l:IntegerType, r:IntegerType| return Some(l & r)) {
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

                    match perform_int_calculation(token.value.clone(), token.children[i].value.clone(), |l:IntegerType, r:IntegerType| return Some(l ^ r)) {
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

                    match perform_int_calculation(token.value.clone(), token.children[i].value.clone(), |l:IntegerType, r:IntegerType| return Some(l | r)) {
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

    return None;
}

pub fn handler(token: &mut Token, state: &mut ParserState) -> Option<ParserError> {
    match atomicvalue_handler(token, state) {
        Some(e) => return Some(e),
        _ => { }
    }
    
    match expression_handler(token, state) {
        Some(e) => return Some(e),
        _ => { }
    }
    
    match call_expression_handler(token, state) {
        Some(e) => return Some(e),
        _ => { }
    }
    
    match bitwise_expression_handler(token, state) {
        Some(e) => return Some(e),
        _ => { }
    }

    return None;
}