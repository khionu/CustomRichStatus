extern crate chrono;
#[macro_use]
extern crate clap;
extern crate core;
extern crate discord_rpc_client;
extern crate quoted_strings;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod command_engine;
mod models;
mod state;
mod utils;

use std::error::Error;

use discord_rpc_client::Client as DiscordRPC;

use command_engine::{command::Command, commands, commands::set::SetCmd};
use command_engine::engine::CmdEngine;
use models::config::Config;
use models::dto::ActivityDto;
use models::preset::Preset;
use state::app_state::State;
use state::meta_data::AppMetaData;
use utils::gnr_error::{GnrError, Handling};

fn main() {
    let config_result = Config::load();

    if let Err(err) = config_result {
        panic!("{}", err);
    }

    let config = config_result.unwrap();

    let rpc = DiscordRPC::new(config.client_id)
        .expect("Failed to create Discord RPC client");

    let initial_dto = match config.preset {
        None => ActivityDto::default(),
        Some(preset) => match load_initial_dto(&preset) {
            Ok(dto) => dto,
            Err(err) => {
                println!("Error loading initial preset: {}", err);
                ActivityDto::default()
            }
        }
    };

    let meta_data = AppMetaData::get(config.prompt.unwrap_or(String::from(">")),
                                     config.quit_msg.unwrap_or(String::from("Buh-bye! o/")));

    let cmd_app = commands::register(meta_data);

    let mut cmd_engine =
        CmdEngine::new(cmd_app, State {
            rpc,
            meta_data,
            current_state: match config.retain_state {
                Some(true) => Some(ActivityDto::default()),
                _ => None,
            },
        });

    cmd_engine.state.rpc.start();

    match SetCmd::run(initial_dto, &mut cmd_engine.state) {
        Ok(msg) => println!("{}", msg),
        Err(err) => panic!("{}", err),
    }

    loop {
        let input = cmd_engine.await_input();

        match cmd_engine.process(input.as_ref()) {
            Ok(msg) => println!("{}", msg),
            Err(err) => {
                match err.handling {
                    Handling::Crash => panic!("{}", err),
                    Handling::Print => print_x_errors(&err, 4),
                    Handling::Exit => {
                        println!("{}", err);
                        std::process::exit(0)
                    },
                }
            }
        }
    }
}

fn load_initial_dto(preset_name: &str) -> Result<ActivityDto, Box<GnrError>>  {
    let preset_load = Preset::from_file(preset_name);

    if let Err(_err) = preset_load {
        panic!("Preset specified in Config was invalid. Please double-check your preset files \
            and your config file");
    }

    let preset = preset_load.unwrap();

    ActivityDto::from_preset(preset)
}

fn print_x_errors(err: &dyn Error, x: i32) {
    eprintln!("{}", err);

    if err.cause().is_some() {
        if x > 0 {
            print_x_errors(err.cause().unwrap(), x - 1);
        } else {
            eprintln!("(Rest of trace excluded)");
        }
    }
}