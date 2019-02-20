#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate lazy_static;
extern crate quoted_strings;
#[macro_use]
extern crate serde_derive;

mod command_engine;
mod models;
mod state;
mod utils;

use discord_rpc_client::Client as DiscordRPC;
use command_engine::{
    engine::CmdEngine,
    commands::SetCmd
};
use models::{
    config::Config,
    dto::ActivityDto,
    preset::Preset,
};
use state::{
    app_state::State,
    meta_data::AppMetaData,
};
use utils::ctrl;
use utils::fail::AppError;

fn main() {
    let config = match Config::get() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("An error occurred trying to load the configuration:");
            ctrl::print_x_errors(&err, 4);
            ctrl::enter_then_quit(-1);
        }
    };

    let rpc = DiscordRPC::new(config.client_id);

    let initial_dto = match &config.preset {
        None => None,
        Some(preset) => Some(match load_initial_dto(preset.as_str()) {
            Ok(dto) => dto,
            Err(err) => {
                eprintln!("Error loading initial preset:");
                ctrl::print_x_errors(&err, 3);
                ActivityDto::default()
            }
        }),
    };

    let meta_data = AppMetaData::get(config.prompt.clone(), config.quit_msg.clone());

    let mut state = State {
        rpc,
        meta_data,
        current_status: None,
    };

    if let Some(init) = initial_dto {
        match SetCmd::run(init, true, &mut state) {
            Ok(msg) => println!("{}", msg),
            Err(err) => panic!("{}", err),
        }
    }

    loop {
        let input = CmdEngine::await_input(&state);

        match CmdEngine::process(input.as_ref(), &mut state) {
            Ok(msg) => println!("{}", msg),
            Err(err) => match err {
                AppError::ClapFailure(c) => {
                    eprintln!("{}", c)
                },
                _ => ctrl::print_x_errors(&err, 4),
            }
        }
    }
}

fn load_initial_dto(preset_name: &str) -> Result<ActivityDto, AppError> {
    let preset = match Preset::from_file(preset_name) {
        Ok(pre) => pre,
        Err(_err) => panic!(
            "Preset specified in Config was invalid. Please double-check \
             your preset files and your config file"
        ),
    };

    ActivityDto::from_preset(preset)
}
