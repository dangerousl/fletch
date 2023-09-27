use std::fs::File;
use std::io::Read;
use serde_yaml::from_reader;

use crate::config::ProxyConfig;

pub async fn load_config(path: &str) -> Result<ProxyConfig, Box<dyn std::error::Error + Send + Sync>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: ProxyConfig = from_reader(contents.as_bytes())?;
    Ok(config)
}
