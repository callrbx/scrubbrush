use std::{fs, path::PathBuf};
use toml::de::Error;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub source_dir: PathBuf,
    pub output_dir: PathBuf,
    pub encode_dir: Option<PathBuf>,
    pub overwrite: bool,
    pub csv_file: Option<PathBuf>,
    pub preset: Option<String>,
    pub hb_path: Option<String>,
    pub formats: Vec<String>,
    pub conv_to: String,
}

impl Config {
    pub fn parse_config(config_path: PathBuf) -> Option<Config> {
        let config_content = fs::read_to_string(config_path).ok()?;
        let config: Result<Self, Error> = toml::from_str(&config_content);
        config.ok()
    }
}
