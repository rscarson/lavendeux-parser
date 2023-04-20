//! Builtin functions that don't fit nicely into other categories

use super::*;
use crate::value::{Value, IntegerType};

use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[cfg(feature = "encoding-functions")]
use base64::{Engine as _, engine::general_purpose};

const TIME : FunctionDefinition = FunctionDefinition {
    name: "time",
    category: None,
    description: "Returns a unix timestamp for the current system time",
    arguments: Vec::new,
    handler: |_function, _state, _args| {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => Ok(Value::Integer(n.as_secs() as IntegerType)),
            Err(_) => Ok(Value::Integer(0))
        }
    }
};

const DEFAULT_TAIL_LINES: IntegerType = 1;
const TAIL : FunctionDefinition = FunctionDefinition {
    name: "tail",
    category: None,
    description: "Returns the last [lines] lines from a given file",
    arguments: || vec![
        FunctionArgument::new_required("filename", ExpectedTypes::String),
        FunctionArgument::new_optional("lines", ExpectedTypes::Int),
    ],
    handler: |_function, _state, args| {
        let n_lines: IntegerType = args.get("lines").optional_or(Value::Integer(DEFAULT_TAIL_LINES))
            .as_int().unwrap_or(DEFAULT_TAIL_LINES);

        let f = File::open(args.get("filename").required().as_string())?;
        let mut lines : Vec<String> = Vec::new();
        for line in BufReader::new(f).lines() {
            lines.push(line?);
            if lines.len() as IntegerType > n_lines {
                lines.remove(0);
            }
        }

        Ok(Value::String(lines.join("\n")))
    }
};

#[cfg(feature = "encoding-functions")]
const URLENCODE  : FunctionDefinition = FunctionDefinition {
    name: "urlencode",
    category: None,
    description: "Escape characters in a string for use in a URL",
    arguments: || vec![
        FunctionArgument::new_required("input", ExpectedTypes::String)
    ],
    handler: |_function, _state, args| {
        let input = args.get("input").required().as_string();
        Ok(Value::String(urlencoding::encode(&input).into_owned()))
    }
};

#[cfg(feature = "encoding-functions")]
const URLDECODE : FunctionDefinition = FunctionDefinition {
    name: "urldecode",
    category: None,
    description: "Decode urlencoded character escape sequences in a string",
    arguments: || vec![
        FunctionArgument::new_required("input", ExpectedTypes::String)
    ],
    handler: |_function, _state, args| {
        let input = args.get("input").required().as_string();
        match urlencoding::decode(&input) {
            Ok(s) => Ok(Value::String(s.into_owned())),
            Err(_) => Err(ParserError::General("Value was not a valid urlencoded string".to_string()))
        }
    }
};

#[cfg(feature = "encoding-functions")]
const BASE64ENCODE : FunctionDefinition = FunctionDefinition {
    name: "atob",
    category: None,
    description: "Convert a string into a base64 encoded string",
    arguments: || vec![
        FunctionArgument::new_required("input", ExpectedTypes::String)
    ],
    handler: |_function, _state, args| {
        let input = args.get("input").required().as_string();
        let mut buf = String::new();
        general_purpose::STANDARD.encode_string(&input, &mut buf);
        Ok(Value::String(buf))
    }
};

#[cfg(feature = "encoding-functions")]
const BASE64DECODE : FunctionDefinition = FunctionDefinition {
    name: "btoa",
    category: None,
    description: "Convert a base64 encoded string to an ascii encoded string",
    arguments: || vec![
        FunctionArgument::new_required("input", ExpectedTypes::String)
    ],
    handler: |_function, _state, args| {
        let input = args.get("input").required().as_string();
        if let Ok(bytes) = general_purpose::STANDARD.decode(input) {
            if let Ok(s) = std::str::from_utf8(&bytes) {
                return Ok(Value::String(s.to_string()))
            }
        }

        Err(ParserError::General("Value was not a valid base64 string".to_string()))
    }
};

/// Register developper functions
pub fn register_functions(table: &mut FunctionTable) {
    table.register(TIME);
    table.register(TAIL);
    table.register(URLDECODE);
    table.register(URLENCODE);
    table.register(BASE64DECODE);
    table.register(BASE64ENCODE);
}

#[cfg(test)]
mod test_builtin_table {
    use super::*;
    const WAS_NOW : IntegerType = 1647531435;
    
    #[test]
    fn test_time() {
        let mut state = ParserState::new();

        let result = TIME.call(&mut state, &[]).unwrap();
        assert_eq!(true, result.as_int().unwrap() > WAS_NOW);
    }
    
    #[test]
    fn test_tail() {
        let mut state = ParserState::new();

        let result = TAIL.call(&mut state, &[Value::String("README.md".to_string()), Value::Integer(5)]).unwrap();
        assert_eq!(4, result.as_string().matches("\n").count());
    }
    
    #[cfg(feature = "encoding-functions")]
    #[test]
    fn test_urlencode_decode() {
        let mut state = ParserState::new();

        let result = URLENCODE.call(&mut state, &[Value::String("TES % T =".to_string())]).unwrap();
        assert_eq!("TES%20%25%20T%20%3D", result.as_string());

        let result = URLDECODE.call(&mut state, &[Value::String("TES%20%25%20T%20%3D".to_string())]).unwrap();
        assert_eq!("TES % T =", result.as_string());
    }
    
    #[cfg(feature = "encoding-functions")]
    #[test]
    fn test_base64encode_decode() {
        let mut state = ParserState::new();

        let result = BASE64ENCODE.call(&mut state, &[Value::String("TES % T =".to_string())]).unwrap();
        assert_eq!("VEVTICUgVCA9", result.as_string());

        let result = BASE64DECODE.call(&mut state, &[Value::String("VEVTICUgVCA9".to_string())]).unwrap();
        assert_eq!("TES % T =", result.as_string());
    }
}
