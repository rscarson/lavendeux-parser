//! Builtin functions for network OPs
use super::*;
use crate::{network::*, value::ObjectType, ExpectedTypes};

use std::collections::HashMap;

const RESOLVE: FunctionDefinition = FunctionDefinition {
    name: "resolve",
    category: Some("network"),
    description: "Returns the IP address associated to a given hostname",
    arguments: || {
        vec![FunctionArgument::new_required(
            "hostname",
            ExpectedTypes::String,
        )]
    },
    handler: |_function, token, _state, args| {
        let hostname = args.get("hostname").required().as_string();
        match resolve(&hostname) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::Io(e, token.clone())),
        }
    },
};

const GET: FunctionDefinition = FunctionDefinition {
    name: "get",
    category: Some("network"),
    description: "Return the resulting text-format body of an HTTP GET call",
    arguments: || {
        vec![
            FunctionArgument::new_required("url", ExpectedTypes::String),
            FunctionArgument::new_optional("headers", ExpectedTypes::Object),
        ]
    },
    handler: |_function, token, _state, args| {
        let url = args.get("url").required().as_string();
        let arg_headers = match args.get("headers").optional() {
            Some(v) => v.as_object(),
            None => ObjectType::new(),
        };
        let headers = HashMap::from_iter(
            arg_headers
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string())),
        );

        match request(&url, None, headers) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::Network(e, token.clone())),
        }
    },
};

const POST: FunctionDefinition = FunctionDefinition {
    name: "post",
    category: Some("network"),
    description: "Return the resulting text-format body of an HTTP POST call",
    arguments: || {
        vec![
            FunctionArgument::new_required("url", ExpectedTypes::String),
            FunctionArgument::new_required("body", ExpectedTypes::String),
            FunctionArgument::new_optional("headers", ExpectedTypes::Object),
        ]
    },
    handler: |_function, token, _state, args| {
        let url = args.get("url").required().as_string();
        let body = args.get("body").required().as_string();
        let arg_headers = match args.get("headers").optional() {
            Some(v) => v.as_object(),
            None => ObjectType::new(),
        };
        let headers = HashMap::from_iter(
            arg_headers
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string())),
        );

        match request(&url, Some(body), headers) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::Network(e, token.clone())),
        }
    },
};

/// Register network functions
pub fn register_functions(table: &mut FunctionTable) {
    table.register(RESOLVE);
    table.register(GET);
    table.register(POST);
}

#[cfg(test)]
mod test_builtin_table {
    use super::*;

    fn hardy_net_test(test: fn() -> Result<Value, Error>) -> Value {
        let results = [test(), test(), test(), test(), test()];
        assert_eq!(true, results.iter().filter(|r| r.is_ok()).count() > 0);
        return results
            .iter()
            .filter(|r| r.is_ok())
            .next()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone();
    }

    #[test]
    fn test_get() {
        assert_eq!(
            true,
            hardy_net_test(|| {
                let mut state = ParserState::new();
                return GET.call(
                    &Token::dummy(""),
                    &mut state,
                    &[
                        Value::String("https://google.com".to_string()),
                        Value::String("authorization=5".to_string()),
                    ],
                );
            })
            .as_string()
            .to_lowercase()
            .starts_with("<!doctype")
        );
    }

    #[test]
    fn test_post() {
        assert_eq!(
            true,
            hardy_net_test(|| {
                let mut state = ParserState::new();
                return POST.call(
                    &Token::dummy(""),
                    &mut state,
                    &[
                        Value::String("https://google.com".to_string()),
                        Value::String("body".to_string()),
                    ],
                );
            })
            .as_string()
            .to_lowercase()
            .starts_with("<!doctype")
        );
    }

    #[test]
    fn test_resolve() {
        let mut state = ParserState::new();

        let result = RESOLVE
            .call(
                &Token::dummy(""),
                &mut state,
                &[Value::String("localhost".to_string())],
            )
            .unwrap()
            .as_string();
        assert_eq!(true, result == "127.0.0.1" || result == "[::1]");
    }
}
