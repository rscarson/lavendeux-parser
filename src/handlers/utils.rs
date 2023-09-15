use crate::{
    token::Token,
    Value,
    IntegerType,
    FloatType,
    errors::*
};

pub type IntHandler = fn(l:IntegerType, r:IntegerType) -> Option<IntegerType>;
pub type FloatHandler = fn(l:FloatType, r:FloatType) -> FloatType;

/// Perform an integer calculation against 2 values
/// 
/// # Arguments
/// * `l` - Left value
/// * `r` - Right value
/// * `handler` - checked_* function
pub fn perform_int_calculation(expression: &Token, l: Value, r: Value, handler: IntHandler) -> Result<Value, ParserError> {
    if l.is_identifier() {
        return Err(VariableNameError::new(expression, &l.to_string()).into())
    } else if r.is_identifier() {
        return Err(VariableNameError::new(expression, &r.to_string()).into())
    }
    
    if l.is_array() && r.is_array() {
        let mut la = l.as_array();
        let ra = r.as_array();

        if la.len() != ra.len() {
            Err(ArrayLengthError::new(expression).into())
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
        if lv.is_none() || rv.is_none() {
            Err(ValueTypeError::new(expression, ExpectedTypes::IntOrFloat).into())
        } else {
            // Detect overflow and return resulting value
            match handler(lv.unwrap(), rv.unwrap()) {
                Some(n) => Ok(Value::Integer(n)),
                None => Err(OverflowError::new(expression).into())
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
pub fn perform_float_calculation(expression: &Token, l: Value, r: Value, handler: FloatHandler) -> Result<Value, ParserError> {
    if l.is_identifier() {
        return Err(VariableNameError::new(expression, &l.to_string()).into())
    } else if r.is_identifier() {
        return Err(VariableNameError::new(expression, &r.to_string()).into())
    }

    if l.is_array() && r.is_array() {
        let mut la = l.as_array();
        let ra = r.as_array();

        if la.len() != ra.len() {
            Err(ArrayLengthError::new(expression).into())
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
        if lv.is_none() || rv.is_none() { 
            return Err(ValueTypeError::new(expression, ExpectedTypes::IntOrFloat).into())
        }
        
        // Detect overflow
        let r = handler(lv.unwrap(), rv.unwrap());
        if r == FloatType::INFINITY {
            return Err(OverflowError::new(expression).into())
        } else if r == FloatType::NEG_INFINITY {
            return Err(UnderflowError::new(expression).into())
        }
    
        // Return resulting value
        Ok(Value::Float(r))
    }
}

/// Perform a calculation against 2 values
/// 
/// # Arguments
/// * `l` - Left value
/// * `r` - Right value
/// * `handler` - checked_* function
pub fn perform_calculation(expression: &Token, l: Value, r: Value, i_handler: IntHandler, f_handler: FloatHandler) -> Result<Value, ParserError> {
    if l.as_array().iter().any(|e| e.is_float()) || r.as_array().iter().any(|e| e.is_float()) {
        perform_float_calculation(expression, l, r, f_handler)
    } else {
        perform_int_calculation(expression, l, r, i_handler)
    }
}

#[cfg(test)]
mod test_token {
    use crate::{ ParserState, Value };
    use super::*;

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
    fn test_perform_calculation() {
        let mut state = ParserState::new();
        let token = Token::new("1.0 + 1.0", &mut state).unwrap();
        assert_eq!(Value::Array(vec![Value::Integer(1), 
            Value::Integer(1)]), 
            perform_calculation(&token, 
                Value::Array(vec![Value::Integer(2), Value::Integer(2)]), 
                Value::Integer(1), 
                |l,r| Some(l-r), |l,r| l-r
            ).unwrap()
        );
        assert_eq!(Value::Integer(1), perform_calculation(&token, Value::Integer(2), Value::Integer(1), |l,r| Some(l-r), |l,r| l-r).unwrap());
        assert_eq!(Value::Float(1.0), perform_calculation(&token, Value::Integer(2), Value::Float(1.0), |l,r| Some(l-r), |l,r| l-r).unwrap());
        assert_eq!(Value::Float(1.0), perform_calculation(&token, Value::Float(2.0), Value::Integer(1), |l,r| Some(l-r), |l,r| l-r).unwrap());
        assert_eq!(Value::Float(1.0), perform_calculation(&token, Value::Float(2.0), Value::Float(1.0), |l,r| Some(l-r), |l,r| l-r).unwrap());
    }
}