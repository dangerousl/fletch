use serde::Deserialize;

pub mod parser;

// ProxyConfig is the top-level configuration struct for the proxy
#[derive(Debug, Deserialize)]
pub struct ProxyConfig {
    pub port: u16,
    pub filters: Vec<FilterConfig>,
}

// FilterConfig is the configuration struct for a given filter... maybe
#[derive(Debug, Deserialize)]
pub struct FilterConfig {
    pub path: String,
    pub action: String,
}
