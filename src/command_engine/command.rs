use clap::ArgMatches;

use state::app_state::State;
use utils::gnr_error::GnrError;

pub trait Command {
    type CmdArgs;

    fn parse(matches: &ArgMatches, state: &State) -> Result<Self::CmdArgs, Box<GnrError>>;
    fn run(args: Self::CmdArgs, state: &mut State) -> Result<String, Box<GnrError>>;

    fn parse_and_run(matches: &ArgMatches, state: &mut State)
        -> Result<String, Box<GnrError>> {
        Self::run(Self::parse(matches, state)?, state)
    }
}