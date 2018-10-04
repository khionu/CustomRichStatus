use clap::ArgMatches;

use command_engine::command::Command;
use state::app_state::State;
use utils::gnr_error::{GnrError, Handling};

pub struct QuitCmd;

impl Command for QuitCmd {
    type CmdArgs = &'static str;

    fn parse(_matches: &ArgMatches, state: &State) -> Result<Self::CmdArgs, Box<GnrError>> {
        Ok(&state.meta_data.quit_msg)
    }

    fn run(msg: Self::CmdArgs, _state: &mut State) -> Result<String, Box<GnrError>> {
        Err(GnrError::new(msg, Handling::Exit))
    }
}