use clap::ArgMatches;

use commands::CmdResult;
use models::dto::ActivityDto;
use state::app_state::InternalState;

pub fn run(_blank: (), state: &mut InternalState) -> CmdResult {
    state.current_state = Some(ActivityDto::default());

    match state.rpc.set_activity(|a| ActivityDto::default().apply_to_activity(a)) {
        Ok(_p) => CmdResult::Ok(String::from("Status successfully cleared")),
        Err(err) => CmdResult::Err(format!("Failed to clear status: {}", err)),
    }
}

pub fn parse(_matches: &ArgMatches, _state: &mut InternalState) -> Result<(), String> {
    Ok(())
}