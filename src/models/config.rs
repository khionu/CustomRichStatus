use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

use dirs;
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

const CONFIG_FILE: &str = "config.yml";
const CONFIG_PATH_FILE: &str = "custom_rich_status/config.yml";

impl Config {
    pub fn load() -> Result<Config, Box<GnrError>> {
        let mut config_path = PathBuf::from(CONFIG_FILE);

        if !config_path.exists() {
            config_path = dirs::config_dir().unwrap().join(CONFIG_PATH_FILE);

            if !config_path.exists() {
                let mut new_file = match File::create(&config_path) {
                    Ok(file) => file,
                    Err(err) => {
                        return Err(GnrError::new_with_cause("Error creating new config file",
                                                            Handling::Crash, err));
                    },
                };

                match new_file.write(Config::default().as_ref()) {
                    Err(err) => {
                        return Err(GnrError::new_with_cause("Error writing new config file",
                                                            Handling::Crash, err));
                    },
                    _ => { },
                }
            }
        }

        let config_file = match File::open(config_path) {
            Ok(file) => file,
            Err(err) =>  {
                return Err(GnrError::new_with_cause("Error reading config file",
                                                    Handling::Crash, err));
            },
        };

        let config= match serde_yaml::from_reader(BufReader::new(config_file)) {
            Ok(cfg) => cfg,
            Err(err) => {
                return Err(GnrError::new_with_cause("Error parsing preset: either invalid YAML or invalid fields",
                   Handling::Crash, err));
            },
        };

        Ok(config)
    }

    fn default() -> &'static [u8] {
        include_bytes!("../../config.yml")
    }
}