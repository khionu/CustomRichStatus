use clap::ArgMatches;
use state::app_state::InternalState;

pub fn run(msg: String, _state: &mut InternalState) -> Result<String, String> {
    Err(msg)
}

pub fn parse(_matches: &ArgMatches) -> Result<String, String> {
    Ok(String::from("Buh-bye! o/"))
}