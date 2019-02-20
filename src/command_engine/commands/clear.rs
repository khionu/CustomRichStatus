use crate::{
    state::app_state::State,
    utils::fail::AppError,
};

const SUCCESS: &str = "Status successfully cleared";

pub struct ClearCmd;

impl ClearCmd {
    pub fn run(state: &mut State) -> Result<String, AppError> {
        state.current_status = None;

        match state.rpc.clear_activity() {
            Ok(_p) => Ok(String::from(SUCCESS)),
            Err(err) => Err(err.into()),
        }
    }
}
