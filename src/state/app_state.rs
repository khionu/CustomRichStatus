use discord_rpc_client::Client as DiscordRPC;

use models::dto::ActivityDto;
use state::meta_data::AppMetaData;

pub struct State {
    pub rpc: DiscordRPC,
    pub meta_data: &'static AppMetaData,
    pub current_state: Option<ActivityDto>,
}