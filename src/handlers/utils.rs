use crate::{token::Token, Error, ExpectedTypes, FloatType, IntegerType, Value};

pub type IntHandler = fn(l: IntegerType, r: IntegerType) -> Option<IntegerType>;
pub type FloatHandler = fn(l: FloatType, r: FloatType) -> FloatType;

/// Perform an integer calculation against 2 values
///
/// # Arguments
/// * `l` - Left value
/// * `r` - Right value
/// * `handler` - checked_* function
pub fn perform_int_calculation(
    expression: &Token,
    l: Value,
    r: Value,
    handler: IntHandler,
) -> Result<Value, Error> {
    if l.is_identifier() {
        return Err(Error::VariableName {
            name: l.to_string(),
            token: expression.clone(),
        });
    } else if r.is_identifier() {
        return Err(Error::VariableName {
            name: r.to_string(),
            token: expression.clone(),
        });
    }

    if l.is_array() && r.is_array() {
        let mut la = l.as_array();
        let ra = r.as_array();

        if la.len() != ra.len() {
            Err(Error::ArrayLengths(expression.clone()))
        } else {
            for (pos, e) in la.clone().iter().enumerate() {
                match perform_int_calculation(expression, e.clone(), ra[pos].clone(), handler) {
                    Ok(n) => la[pos] = n,
                    Err(e) => return Err(e),
                }
            }
            Ok(Value::Array(la))
        }
    } else if l.is_array() {
        let mut la = l.as_array();
        for (pos, e) in la.clone().iter().enumerate() {
            match perform_int_calculation(expression, e.clone(), r.clone(), handler) {
                Ok(n) => la[pos] = n,
                Err(e) => return Err(e),
            }
        }
        Ok(Value::Array(la))
    } else if r.is_array() {
        let mut ra = r.as_array();
        for (pos, e) in ra.clone().iter().enumerate() {
            match perform_int_calculation(expression, l.clone(), e.clone(), handler) {
                Ok(n) => ra[pos] = n,
                Err(e) => return Err(e),
            }
        }
        Ok(Value::Array(ra))
    } else {
        // Perform datatype conversions
        let lv = l.as_int().ok_or(Error::ValueType {
            value: l,
            expected_type: ExpectedTypes::IntOrFloat,
            token: expression.clone(),
        })?;
        let rv = r.as_int().ok_or(Error::ValueType {
            value: r,
            expected_type: ExpectedTypes::IntOrFloat,
            token: expression.clone(),
        })?;

        // Detect overflow and return resulting value
        match handler(lv, rv) {
            Some(n) => Ok(Value::Integer(n)),
            None => Err(Error::Overflow(expression.clone())),
        }
    }
}

/// Perform a floating point calculation against 2 values
///
/// # Arguments
/// * `l` - Left value
/// * `r` - Right value
/// * `handler` - checked_* function
pub fn perform_float_calculation(
    expression: &Token,
    l: Value,
    r: Value,
    handler: FloatHandler,
) -> Result<Value, Error> {
    if l.is_identifier() {
        return Err(Error::VariableName {
            name: l.to_string(),
            token: expression.clone(),
        });
    } else if r.is_identifier() {
        return Err(Error::VariableName {
            name: r.to_string(),
            token: expression.clone(),
        });
    }

    if l.is_array() && r.is_array() {
        let mut la = l.as_array();
        let ra = r.as_array();

        if la.len() != ra.len() {
            Err(Error::ArrayLengths(expression.clone()))
        } else {
            for (pos, e) in la.clone().iter().enumerate() {
                match perform_float_calculation(expression, e.clone(), ra[pos].clone(), handler) {
                    Ok(n) => la[pos] = n,
                    Err(e) => return Err(e),
                }
            }
            Ok(Value::Array(la))
        }
    } else if l.is_array() {
        let mut la = l.as_array();
        for (pos, e) in la.clone().iter().enumerate() {
            match perform_float_calculation(expression, e.clone(), r.clone(), handler) {
                Ok(n) => la[pos] = n,
                Err(e) => return Err(e),
            }
        }
        Ok(Value::Array(la))
    } else if r.is_array() {
        let mut ra = r.as_array();
        for (pos, e) in ra.clone().iter().enumerate() {
            match perform_float_calculation(expression, l.clone(), e.clone(), handler) {
                Ok(n) => ra[pos] = n,
                Err(e) => return Err(e),
            }
        }
        Ok(Value::Array(ra))
    } else {
        // Perform datatype conversions
        let lv = l.as_float().ok_or(Error::ValueType {
            value: l,
            expected_type: ExpectedTypes::IntOrFloat,
            token: expression.clone(),
        })?;
        let rv = r.as_float().ok_or(Error::ValueType {
            value: r,
            expected_type: ExpectedTypes::IntOrFloat,
            token: expression.clone(),
        })?;

        // Detect overflow
        let r = handler(lv, rv);
        if r == FloatType::INFINITY {
            return Err(Error::Overflow(expression.clone()));
        } else if r == FloatType::NEG_INFINITY {
            return Err(Error::Underflow(expression.clone()));
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
pub fn perform_calculation(
    expression: &Token,
    l: Value,
    r: Value,
    i_handler: IntHandler,
    f_handler: FloatHandler,
) -> Result<Value, Error> {
    if l.as_array().iter().any(|e| e.is_float()) || r.as_array().iter().any(|e| e.is_float()) {
        perform_float_calculation(expression, l, r, f_handler)
    } else {
        perform_int_calculation(expression, l, r, i_handler)
    }
}

#[cfg(test)]
mod test_token {
    use super::*;
    use crate::{ParserState, Value};

    #[test]
    fn test_perform_int_calculation() {
        let mut state = ParserState::new();
        assert_eq!(
            Value::Integer(1),
            perform_int_calculation(
                &Token::new("2 - 1", &mut state).unwrap(),
                Value::Integer(2),
                Value::Integer(1),
                |l, r| Some(l - r)
            )
            .unwrap()
        );

        assert_eq!(
            Value::Array(vec![Value::Integer(1), Value::Integer(1)]),
            perform_int_calculation(
                &Token::new("[2, 2] - 1", &mut state).unwrap(),
                Value::Array(vec![Value::Integer(2), Value::Integer(2)]),
                Value::Integer(1),
                |l, r| Some(l - r)
            )
            .unwrap()
        );

        assert_eq!(
            Value::Array(vec![Value::Integer(-1), Value::Integer(-1)]),
            perform_int_calculation(
                &Token::new("1 - [2, 2]", &mut state).unwrap(),
                Value::Integer(1),
                Value::Array(vec![Value::Integer(2), Value::Integer(2)]),
                |l, r| Some(l - r)
            )
            .unwrap()
        );

        assert_eq!(
            Value::Array(vec![Value::Integer(1), Value::Integer(1)]),
            perform_int_calculation(
                &Token::new("[2, 2] - [1, 1]", &mut state).unwrap(),
                Value::Array(vec![Value::Integer(2), Value::Integer(2)]),
                Value::Array(vec![Value::Integer(1), Value::Integer(1)]),
                |l, r| Some(l - r)
            )
            .unwrap()
        );
    }

    #[test]
    fn test_perform_float_calculation() {
        let mut state = ParserState::new();

        assert_eq!(
            Value::Float(1.0),
            perform_float_calculation(
                &Token::new("2.0 - 1.0", &mut state).unwrap(),
                Value::Float(2.0),
                Value::Float(1.0),
                |l, r| l - r
            )
            .unwrap()
        );

        assert_eq!(
            Value::Array(vec![Value::Float(1.0), Value::Float(1.0)]),
            perform_float_calculation(
                &Token::new("[2, 2] - 1", &mut state).unwrap(),
                Value::Array(vec![Value::Integer(2), Value::Float(2.0)]),
                Value::Integer(1),
                |l, r| l - r
            )
            .unwrap()
        );

        assert_eq!(
            Value::Array(vec![Value::Float(-1.0), Value::Float(-1.0)]),
            perform_float_calculation(
                &Token::new("1.0 - [2, 2]", &mut state).unwrap(),
                Value::Float(1.0),
                Value::Array(vec![Value::Integer(2), Value::Integer(2)]),
                |l, r| l - r
            )
            .unwrap()
        );

        assert_eq!(
            Value::Array(vec![Value::Float(1.0), Value::Float(1.0)]),
            perform_float_calculation(
                &Token::new("[2, 2] - [1, 1.0]", &mut state).unwrap(),
                Value::Array(vec![Value::Integer(2), Value::Integer(2)]),
                Value::Array(vec![Value::Integer(1), Value::Float(1.0)]),
                |l, r| l - r
            )
            .unwrap()
        );
    }

    #[test]
    fn test_perform_calculation() {
        let mut state = ParserState::new();
        let token = Token::new("1.0 + 1.0", &mut state).unwrap();
        assert_eq!(
            Value::Array(vec![Value::Integer(1), Value::Integer(1)]),
            perform_calculation(
                &token,
                Value::Array(vec![Value::Integer(2), Value::Integer(2)]),
                Value::Integer(1),
                |l, r| Some(l - r),
                |l, r| l - r
            )
            .unwrap()
        );
        assert_eq!(
            Value::Integer(1),
            perform_calculation(
                &token,
                Value::Integer(2),
                Value::Integer(1),
                |l, r| Some(l - r),
                |l, r| l - r
            )
            .unwrap()
        );
        assert_eq!(
            Value::Float(1.0),
            perform_calculation(
                &token,
                Value::Integer(2),
                Value::Float(1.0),
                |l, r| Some(l - r),
                |l, r| l - r
            )
            .unwrap()
        );
        assert_eq!(
            Value::Float(1.0),
            perform_calculation(
                &token,
                Value::Float(2.0),
                Value::Integer(1),
                |l, r| Some(l - r),
                |l, r| l - r
            )
            .unwrap()
        );
        assert_eq!(
            Value::Float(1.0),
            perform_calculation(
                &token,
                Value::Float(2.0),
                Value::Float(1.0),
                |l, r| Some(l - r),
                |l, r| l - r
            )
            .unwrap()
        );
    }
}
