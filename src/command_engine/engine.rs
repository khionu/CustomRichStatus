use std::io;
use std::io::Write;

use clap::App;
use quoted_strings::QuotedParts;

use state::app_state::State;

use command_engine::command::Command;
use utils::gnr_error::{GnrError, Handling};

pub struct CmdEngine {
    cmd_app: App<'static, 'static>,
    pub state: State,
}

impl CmdEngine {
    pub fn new(cmd_app: App<'static, 'static>, state: State) -> CmdEngine {
        CmdEngine {
            cmd_app,
            state,
        }
    }

    pub fn await_input(&self) -> String {
        let mut buffer = String::new();

        print!("{} ", self.state.meta_data.prompt);

        #[allow(unused_must_use)] { io::stdout().flush(); }

        io::stdin().read_line(&mut buffer).unwrap();

        String::from(buffer.trim_right())
    }

    pub fn process(&mut self, input: &str) -> Result<String, Box<GnrError>> {
        let mut cmd_str = self.state.meta_data.prompt.clone();
        cmd_str += " ";
        cmd_str += input;

        let split_by_quotes = QuotedParts::from(cmd_str.as_ref());

        let matches_result = self.cmd_app.clone().get_matches_from_safe(split_by_quotes);

        if let Err(err) = matches_result {
            return Err(GnrError::new_with_cause("Error parsing arguments", Handling::Print, err));
        }

        let matches = matches_result.unwrap();

        let sub_name = matches.subcommand_name();

        // TODO: Implement as Test instead
        if let None = sub_name {
            panic!("DEVELOPER ERROR: CREATED OPTIONS OUTSIDE OF COMMAND")
        }

        let cmd_matches = matches.subcommand().1.unwrap();

        use command_engine::commands::*;
        match sub_name.unwrap() {
            "set" => SetCmd::parse_and_run(cmd_matches, &mut self.state),
            "clear" => ClearCmd::parse_and_run(cmd_matches, &mut self.state),
            "quit" => QuitCmd::parse_and_run(cmd_matches, &mut self.state),
            &_ => panic!("DEVELOPER ERROR: FAILED TO REGISTER COMMAND") // TODO: Implement as Test instead
        }
    }
}