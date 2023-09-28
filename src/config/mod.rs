use serde::Deserialize;
use std::collections::HashMap;

pub mod parser;

// ProxyConfig is the top-level configuration struct for the proxy
#[derive(Debug, Deserialize, Clone)]
pub struct ProxyConfig {
    pub proxy_port: u16,
    pub routes: HashMap<String, RouteConfig>,
}

// RouteConfig is the configuration struct for a given route, defined as a String key in the routes HashMap
#[derive(Debug, Deserialize, Clone)]
pub struct RouteConfig {
    pub host: String,
    pub port: u16,
    pub filters: Vec<FilterConfig>,
}

// FilterConfig is the configuration struct for a given filter... maybe
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum FilterConfig {
    HeaderAdder(HeaderAdderConfig),
    // Add more Soon™
}

// HeaderAdderConfig is the configuration struct for the HeaderAdder filter
// Example usage:
// filters:
//   - type: "HeaderAdder"
//     header_key: "X-My-Header"
//     header_value: "my_value"
#[derive(Debug, Deserialize, Clone)]
pub struct HeaderAdderConfig {
    pub header_key: String,
    pub header_value: String,
}
