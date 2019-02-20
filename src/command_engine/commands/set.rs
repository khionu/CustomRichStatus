use crate::{
    models::dto::ActivityDto,
    state::app_state::State,
    utils::{
        discord,
        fail::AppError,
    },
};

const SKIPPING_EMPTY: &str = "Skipping empty status update";
const SUCCESS: &str = "Status successfully updated";

pub struct SetCmd;

impl SetCmd {
    pub fn run(mut dto: ActivityDto, clear: bool, state: &mut State) -> Result<String, AppError> {
        if !clear {
            if let Some(current) = &state.current_status {
                let mut c = current.clone();
                c += dto;

                dto = c;
            }
        }

        let current = match &state.current_status {
            Some(c) => c,
            None => ActivityDto::empty_ref(),
        };

        if current == &dto {
            return Ok(String::from(SKIPPING_EMPTY));
        }

        match discord::set_dto(&mut state.rpc, &dto) {
            Ok(_) => {
                state.current_status = Some(dto);
                Ok(String::from(SUCCESS))
            }
            Err(err) => Err(err),
        }
    }
}
