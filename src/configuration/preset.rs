use std::path::PathBuf;
use std::fs::File;
use std::error::Error;
use std::io::BufReader;

use serde_yaml;

#[derive(Serialize, Deserialize)]
pub struct Preset {
    pub details: Option<String>,
    pub state: Option<String>,
    pub large_image: Option<String>,
    pub small_image: Option<String>,
    pub large_text: Option<String>,
    pub small_text: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
}

impl Preset {
    pub fn from_file(name: &str) -> Result<Preset, String> {
        let preset_path: PathBuf = ["presets", &format!("{}.yml", name)].iter().collect();

        let preset_file = File::open(preset_path);

        if let Err(err) = preset_file {
            return Err(format!("Error opening preset: {}", err.description()));
        }

        // TODO: Validate format
        let preset: Preset = serde_yaml::from_reader(
            BufReader::new(preset_file.unwrap())).unwrap();

        Ok(preset)
    }
}