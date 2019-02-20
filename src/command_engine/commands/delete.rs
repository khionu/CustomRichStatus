use crate::{
    models::preset::Preset,
    utils::fail::AppError,
};

const SUCCESS: &str = "Preset successfully deleted";

pub struct DeleteCmd;

impl DeleteCmd {
    pub fn run(name: String) -> Result<String, AppError> {
        match Preset::delete(name.as_ref()) {
            Ok(()) => Ok(String::from(SUCCESS)),
            Err(err) => Err(err),
        }
    }
}
