use std::env;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_dots_dir")]
    pub dots_dir: PathBuf,
    #[serde(rename = "dot")]
    pub dots: Vec<Dot>,
}

#[derive(Deserialize, Debug)]
pub struct Dot {
    #[serde(rename = "src")]
    pub source: PathBuf,
    #[serde(rename = "dest")]
    pub destination: PathBuf,
}

pub enum ConfigLoadError {
    IOError(std::io::Error),
    DeserializationError(toml::de::Error),
}

impl Config {
    pub fn load(from: Option<PathBuf>) -> Result<Config, ConfigLoadError> {
        let path = from.unwrap_or(PathBuf::from("dots.toml"));
        match std::fs::read_to_string(path).map_err(ConfigLoadError::IOError) {
            Ok(string) => match toml::from_str(&string) {
                Ok(config) => Ok(config),
                Err(err) => Err(ConfigLoadError::DeserializationError(err)),
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}

fn default_dots_dir() -> PathBuf {
    env::current_dir().expect("failed to get current dir")
}

