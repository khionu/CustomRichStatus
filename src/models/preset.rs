use std::{
    fs,
    fs::File,
    io::{
        BufReader,
        Write,
    },
    path::{
        PathBuf,
    },
    sync::Mutex,
};

use dirs;
use serde_yaml;

use crate::{
    models::dto::ActivityDto,
    utils::fail::AppError,
};

#[derive(Deserialize, Serialize)]
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
const PRESET_TEMPLATE: &[u8] = include_bytes!("../../presets/template.yml");

lazy_static! {
    static ref PRESET_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);
}

impl Preset {
    pub fn from_file(name: &str) -> Result<Preset, AppError> {
        let preset_path = match Self::check(name) {
            Ok(path) => path,
            Err(err) => return Err(err),
        };

        let preset_file = File::open(&preset_path)?;

        match serde_yaml::from_reader(BufReader::new(preset_file)) {
            Ok(p) => Ok(p),
            Err(err) => Err(AppError::DeserializeFailure {
                data_type: String::from("preset"),
                file: String::from(preset_path.to_str().expect("invalid UTF8")),
                inner: err
            })
        }
    }

    pub fn delete(name: &str) -> Result<(), AppError> {
        let preset_path = match Self::check(name) {
            Ok(path) => path,
            Err(err) => return Err(err),
        };

        fs::remove_file(preset_path)?;

        Ok(())
    }

    pub fn list_all() -> Result<Vec<(String, Option<String>)>, AppError> {
        let path = Self::get_dir(None)?;

        let mut presets = Vec::new();

        let read_result = match fs::read_dir(&path) {
            Ok(dir) => dir,
            Err(err) => return Err(AppError::IoFailure(err))
        };

        for entry in read_result {
            if let Ok(item) = entry {
                let name = match item.file_name().into_string() {
                    Ok(s) => s,
                    Err(e) => e.to_string_lossy().into_owned(),
                };

                match name.ends_with(".yml") {
                    true => {
                        let name = name.trim_right_matches(".yml");
                        let preset = Self::from_file(&name)?;

                        presets.push((String::from(name), preset.meta_description));
                    }
                    _ => {}
                }
            }
        }

        Ok(presets)
    }

    pub fn write(&self, name: &str, overwrite: bool) -> Result<(), AppError> {
        let path = Self::get_dir(name)?;

        let mut file = match File::open(path) {
            Ok(f) => match overwrite {
                true => f,
                false => return Err(AppError::OverwriteFailure { ident: String::from(name) }),
            },
            Err(err) => return Err(AppError::IoFailure(err))
        };

        let string = match serde_yaml::to_string(self) {
            Ok(s) => s,
            Err(err) => return Err(AppError::SerializeFailure {
                data_type: String::from("preset"),
                file: String::from(name),
                inner: err.into()
            })
        };

        file.write(string.as_bytes())?;

        Ok(())
    }

    pub fn check(name: &str) -> Result<PathBuf, AppError> {
        let path = Self::get_dir(name)?;

        match path.exists() {
            true => Ok(path),
            false => Err(AppError::LookupFailure {
                item_type: String::from("preset"),
                ident: String::from(name)
            }),
        }
    }

    fn get_dir<'a>(name: impl Into<Option<&'a str>>) -> Result<PathBuf, AppError> {
        let mut stored = match PRESET_DIR.lock() {
            Ok(p) => p,
            Err(rip) => {
                let mut ded = rip.into_inner();
                *ded = None;
                ded
            }
        };

        let path = match *stored {
            Some(ref path) => path.clone(),
            None => {
                let init = Self::init_dir()?;

                *stored = Some(init.clone());

                init
            }
        };

        Ok(match name.into() {
            Some(val) => path.join(format!("{}.yml", val)),
            None => path.clone(),
        })
    }

    fn init_dir() -> Result<PathBuf, AppError> {
        let path = PathBuf::new().join("presets");

        let path = match path.exists() {
            true => path,
            false => dirs::config_dir().expect("unsupported os").join(PRESET_CONFIG_DIR),
        };

        if !path.exists() {
            fs::create_dir_all(&path)?;

            let template_path = (&path).join("template.yml");

            let mut file = File::open(template_path)?;

            match file.write(PRESET_TEMPLATE) {
                Ok(_) => { },
                Err(err) => return Err(AppError::new_crit(AppError::IoFailure(err)))
            };
        }

        Ok(path)
    }
}

impl From<ActivityDto> for Preset {
    fn from(dto: ActivityDto) -> Self {
        Preset {
            meta_description: None,
            details: dto.details,
            state: dto.state,
            large_image: dto.large_image,
            small_image: dto.small_image,
            large_text: dto.large_text,
            small_text: dto.small_text,
            start: dto.start_hms,
            end: dto.end_hms,
        }
    }
}
