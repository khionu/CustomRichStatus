use std::fs::File;
use std::io::BufReader;

use serde_yaml;
use serde_yaml::Error;

use utils::gnr_error::{GnrError, Handling};

#[derive(Deserialize)]
pub struct Config {
    pub client_id: u64,
    pub preset: Option<String>,
    pub prompt: Option<String>,
    pub retain_state: Option<bool>,
    pub quit_msg: Option<String>,
}

impl Config {
    pub fn load() -> Result<Config, Box<GnrError>> {
        let config_file = File::open("config.yml");

        if let Err(_err) = config_file {
            panic!("Config file not found. Please provide a `config.yml` file in the executable \
            directory formatted as in the documentation");
        }

        let config_result: Result<Config, Error> = serde_yaml::from_reader(
            BufReader::new(config_file.unwrap()));

        if let Err(err) = config_result {
            return Err(GnrError::new_with_cause(
                "Error parsing preset: either invalid YAML or invalid fields",
                Handling::Crash, err));
        }

        let mut config = config_result.unwrap();

        if config.retain_state == None {
            config.retain_state = Some(true);
        }

        Ok(config)
    }
}