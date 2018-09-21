use clap::ArgMatches;

use command_engine::command::Command;
use models::dto::ActivityDto;
use state::app_state::State;
use utils::gnr_error::{GnrError, Handling};

const SUCCESSFULLY_CLEARED: &str = "Status successfully cleared";
const FAILED_TO_CLEAR: &str = "Failed to clear status";

pub struct ClearCmd;

impl Command for ClearCmd {
    type CmdArgs = ();

    fn parse(_matches: &ArgMatches, _state: &State) -> Result<Self::CmdArgs, Box<GnrError>> {
        Ok(())
    }

    fn run(_blank: Self::CmdArgs, state: &mut State) -> Result<&'static str, Box<GnrError>> {
        state.current_state = Some(ActivityDto::default());

        match state.rpc.set_activity(|a| ActivityDto::default().apply_to_activity(a)) {
            Ok(_p) => Ok(SUCCESSFULLY_CLEARED),
            Err(err) => Err(
                GnrError::new_with_cause(FAILED_TO_CLEAR, Handling::Print, err)
            ),
        }
    }
}