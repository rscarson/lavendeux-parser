//! Builtin functions for trigonometry

use super::*;
use crate::value::{Value, FloatType};

fn builtin_trig(method: fn(FloatType) -> FloatType, args: FunctionArgumentCollection) -> Result<Value, ParserError> {
    let n = args.get("n").required().as_float().unwrap();
    Ok(Value::Float(method(n)))
}

/// Macro to shorten definitions
#[macro_use]
mod trig_fn_macro {
    macro_rules! trig_fn {
        ($a:ident, $b:ident, $c:literal) => {
            const $a : FunctionDefinition = FunctionDefinition {
                name: stringify!($b),
                category: Some("math"),
                description: concat!("Calculate the ", $c, " of n"),
                arguments: || vec![
                    FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat)
                ],
                handler: |_function, _token, _state, args| builtin_trig(FloatType::$b, args)
            };
        };
    }
}

trig_fn!(TAN, tan, "tangent");
trig_fn!(ATAN, atan, "arctangent");
trig_fn!(TANH, tanh, "hyperbolic tangent");

trig_fn!(COS, cos, "cosine");
trig_fn!(ACOS, acos, "arccosine");
trig_fn!(COSH, cosh, "hyperbolic cosine");

trig_fn!(SIN, sin, "sine");
trig_fn!(ASIN, asin, "arcsine");
trig_fn!(SINH, sinh, "hyperbolic sine");

const TO_RADIANS : FunctionDefinition = FunctionDefinition {
    name: "to_radians",
    category: Some("math"),
    description: "Convert the given degree value into radians",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat)
    ],
    handler: |_function, _token, _state, args| {
        let n = args.get("n").required().as_float().unwrap();
        Ok(Value::Float(n * (std::f64::consts::PI / 180.0)))
    }
};

const TO_DEGREES : FunctionDefinition = FunctionDefinition {
    name: "to_degrees",
    category: Some("math"),
    description: "Convert the given radian value into degrees",
    arguments: || vec![
        FunctionArgument::new_required("n", ExpectedTypes::IntOrFloat)
    ],
    handler: |_function, _token, _state, args| {
        let n = args.get("n").required().as_float().unwrap();
        Ok(Value::Float(n * 180.0 / std::f64::consts::PI))
    }
};

/// Register trig functions
pub fn register_functions(table: &mut FunctionTable) {
    table.register(TO_RADIANS);
    table.register(TO_DEGREES);

    table.register(TAN);
    table.register(ATAN);
    table.register(TANH);
    
    table.register(COS);
    table.register(ACOS);
    table.register(COSH);
    
    table.register(SIN);
    table.register(ASIN);
    table.register(SINH);
}

#[cfg(test)]
mod test_builtin_functions {
    use super::*;

    /// Macro to shorten test definitions
    #[macro_use]
    mod trig_test_macro {
        macro_rules! trig_test_fn {
            ($test_name:ident, $test_fn:ident, $vl1:expr, $vr1:expr, $vl2:expr, $vr2:expr) => {
                #[test]
                fn $test_name() {
                    let mut state = ParserState::new();
                    let vr1 = $test_fn.call(&Token::dummy(""), &mut state, &[Value::Float($vr1)]).unwrap().as_float().unwrap();
                    let vr2 = $test_fn.call(&Token::dummy(""), &mut state, &[Value::Float($vr2)]).unwrap().as_float().unwrap();
    
                    assert_eq!(Value::Float($vl1), (100.0 * vr1).floor() / 100.0);
                    assert_eq!(Value::Float($vl2), (100.0 * vr2).floor() / 100.0);
                }
            };
        }
    }
        
    #[test]
    fn test_to_radians() {
        let mut state = ParserState::new();

        assert_eq!(Value::Float(std::f64::consts::PI), TO_RADIANS.call(&Token::dummy(""), &mut state, &[Value::Integer(180)]).unwrap());
        assert_eq!(Value::Float(4.0 * std::f64::consts::PI), TO_RADIANS.call(&Token::dummy(""), &mut state, &[Value::Integer(720)]).unwrap());
    }
        
    #[test]
    fn test_to_degrees() {
        let mut state = ParserState::new();

        assert_eq!(Value::Float(180.0), TO_DEGREES.call(&Token::dummy(""), &mut state, &[Value::Float(std::f64::consts::PI)]).unwrap());
        assert_eq!(Value::Float(90.0), TO_DEGREES.call(&Token::dummy(""), &mut state, &[Value::Float(std::f64::consts::PI / 2.0)]).unwrap());
    }

    trig_test_fn!(test_tan, TAN, 
        0.00, 0.0, 
        0.99, std::f64::consts::PI / 4.0
    );

    trig_test_fn!(test_cos, COS, 
        1.00, 0.0, 
        0.00, std::f64::consts::PI / 2.0
    );

    trig_test_fn!(test_sin, SIN, 
        0.00, 0.0, 
        1.00, std::f64::consts::PI / 2.0
    );

    trig_test_fn!(test_atan, ATAN, 
        0.00, 0.0, 
        0.66, std::f64::consts::PI / 4.0
    );

    trig_test_fn!(test_acos, ACOS, 
        0.00, 1.0, 
        0.66, std::f64::consts::PI / 4.0
    );

    trig_test_fn!(test_asin, ASIN, 
        0.00, 0.0, 
        0.90, std::f64::consts::PI / 4.0
    );

    trig_test_fn!(test_tanh, TANH, 
        0.00, 0.0, 
        0.65, std::f64::consts::PI / 4.0
    );

    trig_test_fn!(test_cosh, COSH, 
        1.00, 0.0, 
        2.50, std::f64::consts::PI / 2.0
    );

    trig_test_fn!(test_sinh, SINH, 
        0.00, 0.0, 
        2.30, std::f64::consts::PI / 2.0
    );
}