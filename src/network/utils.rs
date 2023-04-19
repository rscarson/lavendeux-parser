use crate::value::{Value};
use crate::errors::*;

use std::net::ToSocketAddrs;
use std::time::Duration;

/// Resolve a hostname to an IP address
/// 
/// # Arguments
/// * `hostname` - Host to resolve
pub fn resolve(hostname: &str) -> Result<Value, ParserError> {
    match (hostname, 0).to_socket_addrs() {
        Ok(mut addresses) => {
            let address = addresses.next().unwrap().to_string();
            let suffix = ":".to_string() + address.split(':').last().unwrap_or("80");

            Ok(Value::String(address.replace(&suffix, "")))
        },
        Err(e) => Err(ParserError::General(e.to_string()))
    }
}

/// Fetch from a given URL
/// 
/// # Arguments
/// * `url` - Target URL
/// * `body` - Body if POST
/// * `headers` - Array of header=value strings
pub fn request(url: &str, body: Option<String>, headers: Vec<String>) -> Result<Value, ParserError> {
    match reqwest::blocking::Client::builder().timeout(Duration::from_millis(1500)).build() {
        Ok(client) => {
            let mut request = match body {
                None => client.get(url),
                Some(s) => client.post(url).body(s)
            };

            for header in headers.iter() {
                let header = header.split('=').map(|e|e.to_string()).collect::<Vec<String>>();
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