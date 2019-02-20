use crate::{
    models::preset::Preset,
    utils::fail::AppError,
};


const NO_DESC: &str = "Description not provided";

pub struct ListCmd;

impl ListCmd {
    pub fn run() -> Result<String, AppError> {
        let mut buffer = String::new();

        for (name, desc) in Preset::list_all()? {
            let desc = match desc {
                Some(d) => d.clone(),
                None => String::from(NO_DESC),
            };

            buffer.push_str(name.as_str());
            buffer.push_str(" | ");
            buffer.push_str(desc.as_str());
            buffer.push_str("\n");
        }

        Ok(buffer.trim_right().to_owned())
    }
}
