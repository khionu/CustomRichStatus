use std::{
    fs::File,
    io::{
        BufReader,
        Write,
    },
    path::PathBuf,
    sync::Mutex,
};

use dirs;
use serde_yaml;

use crate::utils::fail::AppError;

// ID of the provided default client.
const DEFAULT_ID: u64 = 488908526031339522;
const DEFAULT_PRESET: Option<String> = None;
const DEFAULT_PROMPT: &str = ">";
const DEFAULT_RETAIN_STATE: bool = false;
const DEFAULT_QUIT_MESSAGE: &str = "Buh-bye! o/";

lazy_static! {
    static ref CONFIG: Mutex<Option<&'static Config>> = Mutex::new(None);
}

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

fn default_id() -> u64 {
    DEFAULT_ID
}
fn default_preset() -> Option<String> {
    DEFAULT_PRESET
}
fn default_prompt() -> String {
    String::from(DEFAULT_PROMPT)
}
fn default_retain_state() -> bool {
    true
}
fn default_quit_msg() -> String {
    String::from(DEFAULT_QUIT_MESSAGE)
}

const CONFIG_FILE: &str = "config.yml";
const CONFIG_PATH_FILE: &str = "custom_rich_status/config.yml";

impl Config {
    pub fn get() -> Result<&'static Config, AppError> {
        let mut guard = match CONFIG.lock() {
            Ok(g) => g,
            Err(rip) => {
                let mut ded = rip.into_inner();
                *ded = None;
                ded // Such poison. Much ded. Ow.
            }
        };

        Ok(match *guard {
            Some(config) => config,
            None => {
                let config = Self::load()?;
                *guard = Some(Box::leak(Box::from(config)));
                guard.unwrap()
            }
        })
    }

    fn load() -> Result<Config, AppError> {
        let mut config_path = PathBuf::from(CONFIG_FILE);

        if !config_path.exists() {
            config_path = dirs::config_dir()
                .expect("unsupported system")
                .join(CONFIG_PATH_FILE);

            if !config_path.exists() {
                let mut new_file = File::create(&config_path)?;

                new_file.write(Config::default_bytes())?;

                BufReader::new(Config::default_bytes());

                return Ok(Config::default());
            }
        }

        match serde_yaml::from_reader(BufReader::new(File::open(&config_path)?)) {
            Ok(config) => Ok(config),
            Err(err) => Err(AppError::DeserializeFailure {
                data_type: String::from("config"),
                file: String::from(config_path.to_str().expect("invalid UTF8")),
                inner: err.into(),
            }),
        }
    }

    fn default_bytes() -> &'static [u8] {
        include_bytes!("../../config.yml")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            client_id: DEFAULT_ID,
            preset: DEFAULT_PRESET,
            prompt: String::from(DEFAULT_PROMPT),
            retain_state: DEFAULT_RETAIN_STATE,
            quit_msg: String::from(DEFAULT_QUIT_MESSAGE),
        }
    }
}
