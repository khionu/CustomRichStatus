use std::{
    io,
    io::Write,
    process,
};

use structopt::StructOpt;

use crate::{
    command_engine::commands,
    models::dto::ActivityDto,
    quoted_strings::QuotedParts,
    state::app_state::State,
    utils::fail::AppError,
};

const CRIT_ERR: &str = "critical io error, please report to dev";

pub struct CmdEngine {}

impl CmdEngine {
    pub fn await_input(state: &State) -> String {
        let mut buffer = String::new();
        buffer.push_str(state.meta_data.prompt.as_ref());
        buffer.push(' ');

        print!("{}", &buffer);

        io::stdout().flush().expect(CRIT_ERR);

        io::stdin().read_line(&mut buffer).expect(CRIT_ERR);

        String::from(buffer.trim_right())
    }

    pub fn process(cmd_str: &str, state: &mut State) -> Result<String, AppError> {
        let split_by_quotes = QuotedParts::from(cmd_str);

        let root = commands::CliRoot::from_iter_safe(split_by_quotes)?;

        use crate::command_engine::commands::*;
        match root {
            CliRoot::Quit | CliRoot::Exit => {
                println!("{}", state.meta_data.quit_msg);
                process::exit(0)
            }
            CliRoot::Clear => ClearCmd::run(state),
            CliRoot::Preset(sub) => match sub {
                PresetCli::List => ListCmd::run(),
                PresetCli::Create(args) => {
                    CreateCmd::run(
                        args.name.as_ref(),
                        args.overwrite,
                        args.use_current,
                        ActivityDto::from_flags(args.dto_flags)?,
                        state
                    )
                },
                PresetCli::Delete{ name } => DeleteCmd::run(name),
            },
            CliRoot::Set { dto_flags, clear } => {
                SetCmd::run(ActivityDto::from_flags(dto_flags)?, clear, state)
            }
        }
    }
}
