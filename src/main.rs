extern crate chrono;
#[macro_use]
extern crate clap;
extern crate core;
extern crate discord_rpc_client;
extern crate quoted_strings;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod commands;
mod models;
mod state;
mod utils;

use state::app_state::AppState;

fn main() {
    let mut app_state = AppState::new();

    // Printing final output
    println!("{}", app_state.run());
}
