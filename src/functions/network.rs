use std::net::ToSocketAddrs;
use std::time::Duration;
use crate::value::{Value};
use crate::errors::*;

pub fn builtin_resolve(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("resolve(hostname)", 1, 1)));
    }

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

pub fn builtin_get(args: &[Value]) -> Result<Value, ParserError> {
    if args.is_empty() {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("get(url, [\"header-name=value\", ...])", 1, 1)));
    }

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

pub fn builtin_post(args: &[Value]) -> Result<Value, ParserError> {
    if args.len() != 2 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("post(url, body)", 1, 1)));
    }

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

#[cfg(test)]
mod test_builtin_table {
    use super::*;
    
    #[test]
    fn test_get() {
        let result = builtin_get(&[Value::String("https://google.com".to_string()), Value::String("authorization=5".to_string())]).unwrap();
        assert_eq!(true, result.as_string().to_lowercase().starts_with("<!doctype"));
    }
    
    #[test]
    fn test_post() {
        let result = builtin_post(&[Value::String("https://google.com".to_string()), Value::String("body".to_string())]).unwrap();
        assert_eq!(true, result.as_string().to_lowercase().starts_with("<!doctype"));
    }
    
    #[test]
    fn test_resolve() {
        let result = builtin_resolve(&[Value::String("localhost".to_string())]).unwrap().as_string();
        assert_eq!(true, result == "127.0.0.1" || result == "[::1]");
    }
}