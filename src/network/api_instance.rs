use crate::value::{Value};
use crate::errors::*;
use crate::network::utils::*;

use std::fmt;

/// Represents an instance of an API
#[derive(Clone)]
pub struct ApiInstance {
    base_url: String,
    description: String,
    examples: String,
    key: Option<String>,
}

impl ApiInstance {
    /// Create a new API instance
    /// 
    /// # Arguments
    /// * `base_url` - base url for the API
    pub fn new(base_url: String) -> Self {
        Self { base_url: base_url.trim_end_matches('/').to_string(), description: "".to_string(), examples: "".to_string(), key: None }
    }

    /// Create a new API instance with an API key
    /// 
    /// # Arguments
    /// * `base_url` - base url for the API
    /// * `key` - API key
    pub fn new_with_key(base_url: String, key: String) -> Self {
        let mut i = Self::new(base_url);
        i.set_key(key);
        i
    }

    /// Create a new API instance with a description
    /// 
    /// # Arguments
    /// * `base_url` - base url for the API
    /// * `description` - API description
    /// * `description` - API examples
    pub fn new_with_description(base_url: String, description: String, examples: String) -> Self {
        let mut i = Self::new(base_url);
        i.set_description(description);
        i.set_examples(examples);
        i
    }

    /// Return the base url
    pub fn base_url(&self) -> &String {
        &self.base_url
    }

    /// Set the API key credential for the API
    /// 
    /// # Arguments
    /// * `key` - API key
    pub fn set_key(&mut self, key: String) -> &Self {
        self.key = Some(key);
        self
    }

    /// Return the examples
    pub fn examples(&self) -> &String {
        &self.examples
    }

    /// Set the examples for the API
    /// 
    /// # Arguments
    /// * `examples` - API examples
    pub fn set_examples(&mut self, examples: String) -> &Self {
        self.examples = examples;
        self
    }

    /// Return the description
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Set the description for the API
    /// 
    /// # Arguments
    /// * `description` - API description
    pub fn set_description(&mut self, description: String) -> &Self {
        self.description = description;
        self
    }

    /// Return the API key
    pub fn key(&self) -> &Option<String> {
        &self.key
    }

    /// Add the key header to the supplied list
    /// 
    /// # Arguments
    /// * `key` - API key
    /// * `headers` - Existing headers
    fn add_key_header(&self, headers: &[String]) -> Vec<String> {
        let mut h = headers.to_owned();
        if let Some(key) = self.key.clone() {
            h.push(key);
        }
        h
    }

    /// Make a request to the API
    /// 
    /// # Arguments
    /// * `endpoint` - Endpoint to call
    /// * `body` - Supply a body for POST, or None for GET
    /// * `headers` - Vec of extra headers to supply to the API
    pub fn request(&self, endpoint: &str, body: Option<String>, headers: Vec<String>) -> Result<Value, ParserError> {
        let url = format!("{}/{}", self.base_url(), endpoint);
        request(&url, body, self.add_key_header(&headers))
    }
}

impl fmt::Display for ApiInstance {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = format!("{}{}", if self.description().is_empty() {""} else {"\n\t"}, self.description());
        let examples = format!("{}{}", if self.examples().is_empty() {""} else {"\n\t"}, self.examples());

        write!(f, "{}{}{}", self.base_url(), description, examples)
    }
}