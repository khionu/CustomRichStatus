use discord_rpc_client::Client as DiscordRPC;

use crate::{
    models::dto::ActivityDto,
    state::meta_data::AppMetaData,
};

pub struct State {
    pub rpc: DiscordRPC,
    pub meta_data: &'static AppMetaData,
    pub current_status: Option<ActivityDto>,
}
