use std::{fs, path::PathBuf};
use toml::de::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    source_dir: String,
    output_dir: String,
    overwrite: bool,
    preset: Option<String>,
    hb_path: Option<PathBuf>,
    formats: Vec<String>,
}

impl Config {
    pub fn parse_config(config_path: PathBuf) -> Option<Config> {
        let config_content = fs::read_to_string(config_path).ok()?;
        let config: Result<Self, Error> = toml::from_str(&config_content);
        config.ok()
    }
}
