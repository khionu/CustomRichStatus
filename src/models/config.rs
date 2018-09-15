use std::fs::File;
use std::io::BufReader;

use serde_yaml;

#[derive(Deserialize)]
pub struct Config {
    pub client_id: u64,
    pub preset: Option<String>,
    pub prompt: Option<String>,
    pub retain_state: Option<bool>,
}

impl Config {
    pub fn load() -> Result<Config, String> {
        let config_file = File::open("config.yml");

        if let Err(_err) = config_file {
            panic!("Config file not found. Please provide a `config.yml` file in the executable \
            directory formatted as in the documentation");
        }

        let config = serde_yaml::from_reader(
            BufReader::new(config_file.unwrap()));

        if let Err(_err) = config {
            return Err(format!("Error parsing preset: either invalid YAML or invalid fields"));
        }

        Ok(config.unwrap())
    }
}