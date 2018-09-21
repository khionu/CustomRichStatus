use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;

use serde_yaml;
use utils::gnr_error::{GnrError, Handling};

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
    pub fn from_file(name: &str) -> Result<Preset, Box<GnrError>> {
        let preset_path: PathBuf = ["presets", &format!("{}.yml", name)].iter().collect();

        let preset_file = File::open(preset_path);

        if let Err(err) = preset_file {
            return Err(GnrError::new_with_cause("Error opening preset", Handling::Print, err));
        }

        let preset = serde_yaml::from_reader(
            BufReader::new(preset_file.unwrap()));

        if let Err(err) = preset {
            return Err(GnrError::new_with_cause(
                "Error parsing preset: either invalid YAML or invalid fields",
                Handling::Print, err));
        }

        Ok(preset.unwrap())
    }
}