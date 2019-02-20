use crate::{
    models::{dto::ActivityDto, preset::Preset},
    state::app_state::State,
    utils::fail::AppError,
};

const SUCCESS: &str = "Preset successfully saved";

pub struct CreateCmd;

impl CreateCmd {
    pub fn run(
        name: &str,
        overite: bool,
        use_current: bool,
        mut dto: ActivityDto,
        state: &mut State,
    ) -> Result<String, AppError> {
        if use_current {
            if let Some(c) = &state.current_status {
                let mut cur = c.clone();
                cur += dto;

                dto = cur;
            }
        }

        let preset = Preset::from(dto);
        match preset.write(name, overite) {
            Ok(()) => Ok(String::from(SUCCESS)),
            Err(err) => Err(err),
        }
    }
}
