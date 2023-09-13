//! Builtin functions for API manipulation
use crate::{ApiInstance, Value};
use super::*;

use std::collections::HashMap;

const LIST : FunctionDefinition = FunctionDefinition {
    name: "api_list",
    category: Some("network"),
    description: "List all registered APIs",
    arguments: Vec::new,
    handler: |_function, _token, state, _args| {
        let mut keys = state.apis.keys().collect::<Vec<&String>>();
        keys.sort();
        
        let definitions = keys.iter().map(|k| format!("{}: {}", k, state.apis.get(*k).unwrap()));
        let t = definitions.collect::<Vec<String>>().join("\n");
        
        Ok(Value::String(t))
    }
};

const REGISTER : FunctionDefinition = FunctionDefinition {
    name: "api_register",
    category: Some("network"),
    description: "Register a new API for quick usage",
    arguments: || vec![
        FunctionArgument::new_required("name", ExpectedTypes::String),
        FunctionArgument::new_required("base_url", ExpectedTypes::String),
        FunctionArgument::new_optional("api_key", ExpectedTypes::String),
    ],
    handler: |_function, token, state, args| {
        let name = args.get("name").required().as_string();
        let base_url = args.get("base_url").required().as_string();
        
        let mut instance = ApiInstance::new(base_url);
        if let Some(s) = args.get("api_key").optional() {
            instance.set_key(s.as_string());
        }

        state.apis.insert(name, instance);

        let list = LIST.call(token, state, &[]).unwrap().as_string();
        Ok(Value::String(list))
    }
};

const DELETE : FunctionDefinition = FunctionDefinition {
    name: "api_delete",
    category: Some("network"),
    description: "Remove a registered API from the list",
    arguments: || vec![
        FunctionArgument::new_required("name", ExpectedTypes::String)
    ],
    handler: |_function, token, state, args| {
        let name = args.get("name").required().as_string();
        state.apis.remove(&name);
        
        let list = LIST.call(token, state, &[]).unwrap().as_string();
        Ok(Value::String(list))
    }
};

const CALL : FunctionDefinition = FunctionDefinition {
    name: "api",
    category: Some("network"),
    description: "Make a call to a registered API",
    arguments: || vec![
        FunctionArgument::new_required("name", ExpectedTypes::String),
        FunctionArgument::new_optional("endpoint", ExpectedTypes::String)
    ],
    handler: |_function, token, state, args| {
        let api_name = args.get("name").required().as_string();
        let endpoint = args.get("endpoint").optional_or(Value::String("".to_string())).as_string();

        match state.apis.get(&api_name) {
            Some(api) => {
                match api.request(&endpoint, None, HashMap::from([("Accept".to_string(),"text/plain".to_string())])) {
                    Ok(result) => {
                        Ok(Value::String(result.as_string()))
                    },
                    Err(e) => {
                        Err(NetworkError::from_reqwesterror(token, e).into())
                    }
                }
            },

            None => {
                Err(IOError::new(token, "API {} was not found. Add it with api_register(name, base_url, [optional api key])").into())
            }
        }
    }
};

/// Register api functions
pub fn register_functions(table: &mut FunctionTable) {
    table.register(REGISTER);
    table.register(DELETE);
    table.register(LIST);
    table.register(CALL);
}

#[cfg(test)]
mod test_builtin_functions {
    use super::*;

    fn hardy_net_test(test: fn() -> Result<Value, ParserError>) -> Value {
        let results = [
            test(), test(), test(), test(), test()
        ];
        assert_eq!(true, results.iter().filter(|r| r.is_ok()).count() > 0);
        return results.iter().filter(|r| r.is_ok()).next().unwrap().as_ref().unwrap().clone()
    }

    #[test]
    fn test_register() {
        let mut state = ParserState::new();
        let name = "dictionary2".to_string();
        let url = "https://api.dictionaryapi.dev/api/v2/entries/en".to_string();

        assert_eq!(false, state.apis.contains_key(&name));

        assert_eq!(true, REGISTER.call(&Token::dummy(""), &mut state, &[
            Value::String(name.clone()),
            Value::String(url)
        ]).unwrap().as_string().contains(&name));

        assert_eq!(true, state.apis.contains_key(&name));

    }

    #[test]
    fn test_delete() {
        let mut state = ParserState::new();
        let name = "dictionary".to_string();

        assert_eq!(true, state.apis.contains_key(&name));

        assert_eq!(false, DELETE.call(&Token::dummy(""), &mut state, &[
            Value::String(name.clone())
        ]).unwrap().as_string().contains(&name));

        assert_eq!(false, state.apis.contains_key(&name));

    }

    #[test]
    fn test_list() {
        let mut state = ParserState::new();
        let name = "dictionary".to_string();

        assert_eq!(true, LIST.call(&Token::dummy(""), &mut state, &[
        ]).unwrap().as_string().contains(&name));

    }

    #[test]
    fn test_call() {        
        assert_eq!(true, hardy_net_test(|| {
            let mut state = ParserState::new();
            let name = "dictionary".to_string();
            return CALL.call(&Token::dummy(""), &mut state, &[
                Value::String(name.clone()),
                Value::String("en/fart".to_string())
            ]);
        }).as_string().contains("the anus"));

    }
}