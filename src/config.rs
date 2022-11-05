use serde::Deserialize;
use std::{error::Error, fs};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub streamers: Vec<String>,
    pub threads_num: Option<usize>,
}

impl Config {
    pub fn from(path: &String) -> Result<Config, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;

        Ok(config)
    }
}
