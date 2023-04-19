//! Builtin functions for network OPs

use super::*;
use crate::network::*;

const RESOLVE : FunctionDefinition = FunctionDefinition {
    name: "resolve",
    category: Some("network"),
    description: "Returns the IP address associated to a given hostname",
    arguments: || vec![
        FunctionArgument::new_required("hostname", ExpectedTypes::String),
    ],
    handler: |_function, _state, args| {
        let hostname = args.get("hostname").required().as_string();
        resolve(&hostname)
    }
};

const GET : FunctionDefinition = FunctionDefinition {
    name: "get",
    category: Some("network"),
    description: "Return the resulting text-format body of an HTTP GET call",
    arguments: || vec![
        FunctionArgument::new_required("url", ExpectedTypes::String),
        FunctionArgument::new_plural("headers", ExpectedTypes::String, true)
    ],
    handler: |_function, _state, args| {
        let url = args.get("url").required().as_string();
        let headers = args.get("headers").plural().iter().map(|v| v.as_string()).collect();
        request(&url, None, headers)
    }
};

const POST : FunctionDefinition = FunctionDefinition {
    name: "post",
    category: Some("network"),
    description: "Return the resulting text-format body of an HTTP POST call",
    arguments: || vec![
        FunctionArgument::new_required("url", ExpectedTypes::String),
        FunctionArgument::new_required("body", ExpectedTypes::String),
        FunctionArgument::new_plural("header-name=value", ExpectedTypes::String, true)
    ],
    handler: |_function, _state, args| {
        let url = args.get("url").required().as_string();
        let body = args.get("body").required().as_string();
        let headers = args.get("headers").plural().iter().map(|v| v.as_string()).collect();
        request(&url, Some(body), headers) // Once told me
    }
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

    fn hardy_net_test(test: fn() -> Result<Value, ParserError>) -> Value {
        let results = [
            test(), test(), test()
        ];
        assert_eq!(true, results.iter().filter(|r| r.is_ok()).count() > 0);
        return results.iter().filter(|r| r.is_ok()).next().unwrap().clone().unwrap();
    }
    
    #[test]
    fn test_get() {
        assert_eq!(true, hardy_net_test(|| {
            let mut state = ParserState::new();
            return GET.call(&mut state, &[Value::String("https://google.com".to_string()), Value::String("authorization=5".to_string())]);
        }).as_string().to_lowercase().starts_with("<!doctype"));
    }
    
    #[test]
    fn test_post() {
        assert_eq!(true, hardy_net_test(|| {
            let mut state = ParserState::new();
            return POST.call(&mut state,  &[Value::String("https://google.com".to_string()), Value::String("body".to_string())]);
        }).as_string().to_lowercase().starts_with("<!doctype"));
    }
    
    #[test]
    fn test_resolve() {
        let mut state = ParserState::new();

        let result = RESOLVE.call(&mut state, 
            &[Value::String("localhost".to_string())]).unwrap().as_string();
        assert_eq!(true, result == "127.0.0.1" || result == "[::1]");
    }
}