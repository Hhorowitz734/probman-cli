// src/config.rs
use std::fs;
use std::path::PathBuf;
use dirs::config_dir;
use toml;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_base_url: String
}

impl Config {
    pub fn load() ->
    Result<Self, Box<dyn std::error::Error>> {
        let mut path = config_dir().ok_or("Could not find config directory")?;
        path.push("probman/config.toml");

        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}
