use clap::ArgMatches;

use command_engine::command::Command;
use models::dto::ActivityDto;
use state::app_state::State;
use utils::gnr_error::{GnrError, Handling};

const SKIPPING_EMPTY: &str = "Skipping empty status update";
const SUCCESSFUL_UPDATE: &str = "Status successfully updated";
const FAILED_TO_UPDATE: &str = "Failed to set status";

pub struct SetCmd;

impl Command for SetCmd {
    type CmdArgs = ActivityDto;

    fn parse(matches: &ArgMatches, state: &State) -> Result<Self::CmdArgs, Box<GnrError>> {
        let mut dto = match state.current_state {
            Some(ref current) => current.clone(),
            _ => ActivityDto::default(),
        };

        dto.add_cmd_args(matches)
    }

    fn run(dto: Self::CmdArgs, state: &mut State) -> Result<String, Box<GnrError>> {
        if state.current_state.as_ref().map(|state| state == &dto).unwrap_or(false) {
            return Ok(String::from(SKIPPING_EMPTY));
        }

        let dto_clone = dto.clone();

        match state.rpc.set_activity(|a| dto_clone.apply_to_activity(a)) {
            Ok(_p) => {
                state.current_state = Some(dto);
                Ok(String::from(SUCCESSFUL_UPDATE))
            },
            Err(err) => Err(
                GnrError::new_with_cause(FAILED_TO_UPDATE, Handling::Print, err)
            ),
        }
    }
}