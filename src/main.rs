extern crate chrono;
extern crate discord_rpc_client;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;
extern crate quoted_strings;
#[macro_use]
extern crate clap;

mod commands;
mod models;
mod state;
mod utils;

use state::app_state::AppState;

fn main() {
    let mut app_state = AppState::new();

    println!("{}", app_state.run());
}
