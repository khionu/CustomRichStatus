use std;
use std::error::Error;
use std::io;
use std::io::Write;

use clap::App;
use discord_rpc_client::Client as DiscordRPC;
use quoted_strings::QuotedParts;

use commands;
use commands::*;
use models::{config::Config, preset::Preset, dto::ActivityDto};

pub struct AppState {
    initial_dto: ActivityDto,
    state: InternalState,
    cmd_app: App<'static, 'static>,
}

pub struct InternalState {
    pub rpc: DiscordRPC
}

impl AppState {
    pub fn new() -> AppState {
        let config_result = Config::load();

        if let Err(err) = config_result {
            panic!("{}", err);
        }

        let config = config_result.unwrap();

        let rpc = DiscordRPC::new(config.client_id)
            .expect("Failed to create Discord RPC client");

        let initial_dto = match config.preset {
            None => ActivityDto::default(),
            Some(preset) => match AppState::load_initial_dto(&preset) {
                Ok(dto) => dto,
                Err(err) => {
                    println!("Error loading initial preset: {}", err);
                    ActivityDto::default()
                }
            }
        };

        AppState {
            initial_dto,
            state: InternalState { rpc },
            cmd_app: commands::register(),
        }
    }

    pub fn run(&mut self) {
        self.state.rpc.start();

        println!("{}", set::run(self.initial_dto.clone(), &mut self.state));

        loop {
            let mut buffer = String::new();

            print!("> ");

            #[allow(unused_must_use)] { io::stdout().flush(); }

            io::stdin().read_line(&mut buffer).unwrap();

            let result = self.parse_and_execute(buffer.trim_right());

            println!("{}", result);
        }
    }

    fn parse_and_execute(&mut self, input: &str) -> String {
        let matches_result =
            self.cmd_app.clone().get_matches_from_safe(QuotedParts::from(&("> ".to_owned() + input)));

        if let Err(err) = matches_result {
            return format!("Error parsing arguments: {}", err.description());
        }

        let matches = matches_result.unwrap();

        let sub_name = matches.subcommand_name();

        if let None = sub_name {
            return String::from("No command provided. Run \"help\" for more information");
        }

        let cmd = matches.subcommand().1.unwrap();

        macro_rules! load_command {
            [ $ns:ident ] => {
                match $ns::parse(cmd) {
                    Err(err) => return format!("Error parsing command: {}", err),
                    Ok(args) => $ns::run(args, &mut self.state),
                }
            }
        }

        match matches.subcommand_name().unwrap() {
            "quit" => {
                println!("Buh-bye! o/");
                std::process::exit(0);
            },
            "set" => { load_command![set] },
            &_ => String::from("Command not found. Please check your syntax"),
        }

    }

    fn load_initial_dto(preset_name: &str) -> Result<ActivityDto, String>  {
        let preset_load = Preset::from_file(preset_name);

        if let Err(_err) = preset_load {
            panic!("Preset specified in Config was invalid. Please double-check your preset files \
            and your config file");
        }

        let preset = preset_load.unwrap();

        ActivityDto::from_preset(preset)
    }
}