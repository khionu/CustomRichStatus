use crate::models::dto::ActivityDto;
use crate::utils::fail::AppError;

pub fn set_dto(rpc: &mut discord_rpc_client::Client, dto: &ActivityDto) -> Result<(), AppError> {
    let dto = dto.clone();

    dbg!(match rpc.set_activity(|a| {
        println!("applying to activity: {:?}", &dto);

        dto.apply_to_activity(a)
    }) {
        Ok(_) => Ok(()),
        Err(err) => Err(AppError::DiscordError(err.into())),
    })
}
