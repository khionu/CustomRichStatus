use std::fs::File;
use std::io::BufReader;

use serde_yaml;

use utils::gnr_error::{GnrError, Handling};

#[derive(Deserialize)]
pub struct Config {
    pub client_id: u64,
    #[serde(default = "default_preset")]
    pub preset: Option<String>,
    #[serde(default = "default_prompt")]
    pub prompt: String,
    #[serde(default = "default_retain_state")]
    pub retain_state: bool,
    #[serde(default = "default_quit_msg")]
    pub quit_msg: String,
}

fn default_preset() -> Option<String> { None }
fn default_prompt() -> String { String::from(">") }
fn default_retain_state() -> bool { true }
fn default_quit_msg() -> String { String::from("Buh-bye! o/") }

impl Config {
    pub fn load() -> Result<Config, Box<GnrError>> {
        let config_file = match File::open("config.yml") {
            Ok(file) => file,
            Err(_err) => panic!("Config file not found. Please provide a `config.yml` file in the \
                                 executable directory formatted as in the documentation"),
        };

        let config= match serde_yaml::from_reader(BufReader::new(config_file)) {
            Ok(cfg) => cfg,
            Err(err) => { return Err(GnrError::new_with_cause(
                "Error parsing preset: either invalid YAML or invalid fields",
                Handling::Crash, err)); },
        };

        Ok(config)
    }
}