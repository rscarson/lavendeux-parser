use super::{FunctionDefinition, FunctionArgument, FunctionTable};
use crate::value::{Value};
use crate::errors::*;
use std::net::ToSocketAddrs;
use std::time::Duration;

const RESOLVE : FunctionDefinition = FunctionDefinition {
    name: "resolve",
    description: "Returns the IP address associated to a given hostname",
    arguments: || vec![
        FunctionArgument::new_required("hostname", ExpectedTypes::String),
    ],
    handler: |_, args: &[Value]| {
        let mut hostname = args[0].as_string();
        if !hostname.contains(':') {
            hostname += ":80";
        }

        match hostname.to_socket_addrs() {
            Ok(mut addresses) => {
                let address = addresses.next().unwrap().to_string();
                let suffix = ":".to_string() + address.split(':').last().unwrap();

                Ok(Value::String(address.replace(&suffix, "")))
            },
            Err(e) => Err(ParserError::General(e.to_string()))
        }
    }
};

const GET : FunctionDefinition = FunctionDefinition {
    name: "get",
    description: "Return the resulting text-format body of an HTTP GET call",
    arguments: || vec![
        FunctionArgument::new_required("url", ExpectedTypes::String),
        FunctionArgument::new_plural("header-name=value", ExpectedTypes::String, true)
    ],
    handler: |_, args: &[Value]| {
        match reqwest::blocking::Client::builder().timeout(Duration::from_millis(500)).build() {
            Ok(client) => {
                let mut request = client.get(args[0].as_string());
                for arg in args.iter().skip(1) {
                    let header = arg.as_string().split('=').map(|e|e.to_string()).collect::<Vec<String>>();
                    if header.len() < 2 { return Err(ParserError::General("malformed header".to_string())); }
                    request = request.header(header[0].clone(), header[1..].join("="));
                }

                match request.send() {
                    Ok(res) => {
                        match res.text() {
                            Ok(s) => Ok(Value::String(s)),
                            Err(e) => Err(ParserError::General(e.to_string()))
                        }
                    },
                    Err(e) => {
                        Err(ParserError::General(e.to_string()))
                    }
                }
            },
            Err(e) => Err(ParserError::General(e.to_string()))
        }
    }
};

const POST : FunctionDefinition = FunctionDefinition {
    name: "post",
    description: "Return the resulting text-format body of an HTTP POST call",
    arguments: || vec![
        FunctionArgument::new_required("url", ExpectedTypes::String),
        FunctionArgument::new_required("body", ExpectedTypes::String),
        FunctionArgument::new_plural("header-name=value", ExpectedTypes::String, true)
    ],
    handler: |_, args: &[Value]| {
        match reqwest::blocking::Client::builder().timeout(Duration::from_millis(500)).build() {
            Ok(client) => {
                let mut request = client.post(args[0].as_string()).body(args[1].as_string());
                for arg in args.iter().skip(2) {
                    let header = arg.as_string().split('=').map(|e|e.to_string()).collect::<Vec<String>>();
                    if header.len() < 2 { return Err(ParserError::General("malformed header".to_string())); }
                    request = request.header(header[0].clone(), header[1..].join("="));
                }
    
                match request.send() {
                    Ok(res) => {
                        match res.text() {
                            Ok(s) => Ok(Value::String(s)),
                            Err(e) => Err(ParserError::General(e.to_string()))
                        }
                    },
                    Err(e) => {
                        Err(ParserError::General(e.to_string()))
                    }
                }
            },
            Err(e) => Err(ParserError::General(e.to_string()))
        }
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
    
    #[test]
    fn test_get() {
        let result = (GET.handler)(&GET, &[Value::String("https://google.com".to_string()), Value::String("authorization=5".to_string())]).unwrap();
        assert_eq!(true, result.as_string().to_lowercase().starts_with("<!doctype"));
    }
    
    #[test]
    fn test_post() {
        let result = (POST.handler)(&POST, &[Value::String("https://google.com".to_string()), Value::String("body".to_string())]).unwrap();
        assert_eq!(true, result.as_string().to_lowercase().starts_with("<!doctype"));
    }
    
    #[test]
    fn test_resolve() {
        let result = (RESOLVE.handler)(&RESOLVE, &[Value::String("localhost".to_string())]).unwrap().as_string();
        assert_eq!(true, result == "127.0.0.1" || result == "[::1]");
    }
}