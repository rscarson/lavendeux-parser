use crate::value::{AtomicValue};
use crate::errors::*;

pub fn builtin_get(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("get(url)", 1, 1)));
    }

    match reqwest::blocking::get(args[0].as_string()) {
        Ok(res) => {
            match res.text() {
                Ok(s) => Ok(AtomicValue::String(s)),
                Err(e) => Err(ParserError::General(e.to_string()))
            }
        },
        Err(e) => {
            Err(ParserError::General(e.to_string()))
        }
    }
}

#[cfg(test)]
mod test_builtin_table {
    use super::*;
    
    #[test]
    fn test_choose() {
        let result = builtin_get(&[AtomicValue::String("https://google.com".to_string())]).unwrap();
        assert_eq!(true, result.as_string().starts_with("<!doctype"));
    }
}