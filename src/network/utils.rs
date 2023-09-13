use crate::value::{Value};

use std::collections::HashMap;
use std::net::ToSocketAddrs;
use std::time::Duration;

/// Resolve a hostname to an IP address
/// 
/// # Arguments
/// * `hostname` - Host to resolve
pub fn resolve(hostname: &str) -> Result<Value, std::io::Error> {
    match (hostname, 0).to_socket_addrs() {
        Ok(mut addresses) => {
            let address = addresses.next().unwrap().to_string();
            let suffix = ":".to_string() + address.split(':').last().unwrap_or("80");

            Ok(Value::String(address.replace(&suffix, "")))
        },
        Err(e) => Err(e)
    }
}

/// Fetch from a given URL
/// 
/// # Arguments
/// * `url` - Target URL
/// * `body` - Body if POST
/// * `headers` - Array of header=value strings
pub fn request(url: &str, body: Option<String>, headers: HashMap<String, String>) -> Result<Value, reqwest::Error> {
    match reqwest::blocking::Client::builder().timeout(Duration::from_millis(1500)).build() {
        Ok(client) => {
            let mut request = match body {
                None => client.get(url),
                Some(s) => client.post(url).body(s)
            };

            for (header, value) in headers.iter() {
                request = request.header(header, value);
            }

            match request.send() {
                Ok(res) => {
                    match res.text() {
                        Ok(s) => Ok(Value::String(s)),
                        Err(e) => Err(e)
                    }
                },
                Err(e) => Err(e)
            }
        },
        Err(e) => Err(e)
    }
}