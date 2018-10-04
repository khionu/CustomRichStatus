use std::ops::Add;

use clap::ArgMatches;

use command_engine::command::Command;
use models::preset::Preset;
use state::app_state::State;
use utils::gnr_error::GnrError;

pub struct PresetsCmd;

impl Command for PresetsCmd {
    type CmdArgs = ();

    fn parse(_matches: &ArgMatches, _state: &State) -> Result<Self::CmdArgs, Box<GnrError>> {
        Ok(())
    }

    fn run(_args: Self::CmdArgs, _state: &mut State) -> Result<String, Box<GnrError>> {
        let mut buffer = String::new();

        for (name, desc) in Preset::list_all()? {
            buffer = buffer.add(name.as_ref()).add(" | ")
                  .add(desc.unwrap_or(String::from("Description not provided")).as_ref())
                  .add("\n");
        }

        Ok(buffer.trim_right().to_owned())
    }
}