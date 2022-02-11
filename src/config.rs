use std::fs;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub riot: RiotConfig,
}

#[derive(Debug, Deserialize)]
pub struct RiotConfig {
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub ip: [u8; 4],
    pub port: u16,
}

pub fn get_config() -> Config {
    let file = fs::read_to_string("./config.toml").unwrap();

    toml::from_str(&file).unwrap()
}
