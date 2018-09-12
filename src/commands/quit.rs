use clap::ArgMatches;
use state::app_state::InternalState;
use commands::CmdResult;
use commands::CmdResult::Fatal;

pub fn run(msg: String, _state: &mut InternalState) -> CmdResult {
    Fatal(msg)
}

pub fn parse(_matches: &ArgMatches) -> Result<String, String> {
    Ok(String::from("Buh-bye! o/"))
}