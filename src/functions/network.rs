use std::net::ToSocketAddrs;
use std::time::Duration;
use crate::value::{AtomicValue};
use crate::errors::*;

pub fn builtin_resolve(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("resolve(hostname)", 1, 1)));
    }

    let mut hostname = args[0].as_string();
    if !hostname.contains(":") {
        hostname = hostname + ":80";
    }

    match hostname.to_socket_addrs() {
        Ok(mut addresses) => {
            let address = addresses.next().unwrap().to_string();
            let suffix = ":".to_string() + address.split(":").last().unwrap();

            Ok(AtomicValue::String(address.replace(&suffix, "")))
        },
        Err(e) => Err(ParserError::General(e.to_string()))
    }
}

pub fn builtin_get(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() < 1 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("get(url, [\"header-name=value\", ...])", 1, 1)));
    }

    match reqwest::blocking::Client::builder().timeout(Duration::from_millis(500)).build() {
        Ok(client) => {
            let mut request = client.get(args[0].as_string());
            for i in 1..args.len() {
                let header = args[i].as_string().split("=").map(|e|e.to_string()).collect::<Vec<String>>();
                if header.len() < 2 { return Err(ParserError::General("malformed header".to_string())); }
                request = request.header(header[0].clone(), header[1..].join("="));
            }

            match request.send() {
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
        },
        Err(e) => Err(ParserError::General(e.to_string()))
    }
}

pub fn builtin_post(args: &[AtomicValue]) -> Result<AtomicValue, ParserError> {
    if args.len() != 2 {
        return Err(ParserError::FunctionNArg(FunctionNArgError::new("post(url, body)", 1, 1)));
    }

    match reqwest::blocking::Client::builder().timeout(Duration::from_millis(500)).build() {
        Ok(client) => {
            let mut request = client.post(args[0].as_string()).body(args[1].as_string());
            for i in 2..args.len() {
                let header = args[i].as_string().split("=").map(|e|e.to_string()).collect::<Vec<String>>();
                if header.len() < 2 { return Err(ParserError::General("malformed header".to_string())); }
                request = request.header(header[0].clone(), header[1..].join("="));
            }

            match request.send() {
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
        },
        Err(e) => Err(ParserError::General(e.to_string()))
    }
}

#[cfg(test)]
mod test_builtin_table {
    use super::*;
    
    #[test]
    fn test_get() {
        let result = builtin_get(&[AtomicValue::String("https://google.com".to_string()), AtomicValue::String("authorization=5".to_string())]).unwrap();
        assert_eq!(true, result.as_string().to_lowercase().starts_with("<!doctype"));
    }
    
    #[test]
    fn test_post() {
        let result = builtin_post(&[AtomicValue::String("https://google.com".to_string()), AtomicValue::String("body".to_string())]).unwrap();
        assert_eq!(true, result.as_string().to_lowercase().starts_with("<!doctype"));
    }
    
    #[test]
    fn test_resolve() {
        assert_eq!("127.0.0.1", builtin_resolve(&[AtomicValue::String("localhost".to_string())]).unwrap().as_string());
    }
}