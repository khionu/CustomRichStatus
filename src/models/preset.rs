use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

use dirs;
use serde_yaml;

use utils::gnr_error::{GnrError, Handling};

#[derive(Deserialize)]
pub struct Preset {
    pub meta_description: Option<String>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub large_image: Option<String>,
    pub small_image: Option<String>,
    pub large_text: Option<String>,
    pub small_text: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
}

const PRESET_CONFIG_DIR: &str = "custom_rich_status/presets/";

impl Preset {
    pub fn from_file(name: &str) -> Result<Preset, Box<GnrError>> {
        let preset_path = Self::get_dir()?.join(format!("{}.yml", name));

        let preset_file = match File::open(preset_path) {
            Ok(file) => file,
            Err(err) => {
                return Err(GnrError::new_with_cause("Error opening preset", Handling::Print, err));
            },
        };

        let preset = match serde_yaml::from_reader(BufReader::new(preset_file)) {
            Ok(pre) => pre,
            Err(err) => { return Err(GnrError::new_with_cause(
                "Error parsing preset: either invalid YAML or invalid fields",
                Handling::Print, err)); },
        };

        Ok(preset)
    }

    pub fn list_all() -> Result<Vec<(String, Option<String>)>, Box<GnrError>> {
        let path = Self::get_dir()?;

        let mut presets = Vec::new();

        let read_result = fs::read_dir(path);

        if let Err(err) = read_result {
            return Err(GnrError::new_with_cause("Error reading preset directory",
                                                Handling::Print, err));
        }

        for entry in read_result.unwrap() {
            if let Ok(item) = entry {
                let name = match item.file_name().into_string() {
                    Ok(s) => s,
                    Err(e) => e.to_string_lossy().into_owned(),
                };

                match name.ends_with(".yml") {
                    true => {
                        let name= name.trim_right_matches(".yml");
                        let preset = Self::from_file(&name)?;

                        presets.push((String::from(name), preset.meta_description));
                    },
                    _ => {},
                }
            }
        }

        Ok(presets)
    }

    fn get_dir() -> Result<PathBuf, Box<GnrError>> {
        let preset_path = PathBuf::new().join("presets");

        let preset_path = match preset_path.exists() {
            true => preset_path,
            false => dirs::config_dir().unwrap().join(PRESET_CONFIG_DIR),
        };

        if !preset_path.exists() {
            if let Err(err) = fs::create_dir_all(&preset_path) {
                return Err(GnrError::new_with_cause("Failed to create preset directory",
                                                    Handling::Crash, err));
            }

            let template_path = (&preset_path).join("template.yml");
            match File::open(template_path) {
                Ok(mut file) => {
                    match file.write(Self::template()) {
                        Ok(_size) => { },
                        Err(err) => {
                            return Err(GnrError::new_with_cause("Failed to create template preset",
                                                                Handling::Crash, err))
                        },
                    }},
                Err(err) => {
                    return Err(GnrError::new_with_cause("Failed to create template preset",
                                                        Handling::Crash, err))
                },
            }
        }

        Ok(preset_path)
    }

    fn template() -> &'static [u8] {
        include_bytes!("../../presets/template.yml")
    }
}