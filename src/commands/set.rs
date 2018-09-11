use state::app_state::InternalState;
use clap::ArgMatches;
use utils::{AddOrSub, hms_to_u64};
use models::{preset::Preset, dto::ActivityDto};

pub fn run(dto: ActivityDto, state: &mut InternalState) -> Result<String, String> {
    if dto == ActivityDto::default() {
        return Err(String::from("Skipping empty status update"));
    }

    match state.rpc.set_activity(|a|dto.apply_to_activity(a)) {
        Ok(_p) => Ok(String::from("Status successfully updated")),
        Err(err) => Err(format!("Failed to set status: {}", err))
    }
}

pub fn parse(matches: &ArgMatches) -> Result<ActivityDto, String> {
    let mut dto = ActivityDto::default();

    if matches.is_present("PRESET") {
        let preset_name = matches.value_of("PRESET").unwrap();
        let preset = Preset::from_file(preset_name)?;

        dto = ActivityDto::from_preset(preset)?;
    }

    macro_rules! hms_args_to_dto {
        [ $prop:ident, $arg:expr, $act:ident ] => {
            if matches.is_present($arg) {
                let $prop = hms_to_u64(matches.value_of($arg).unwrap(), &AddOrSub::$act)?;

                dto.$prop = Some($prop);
            }
        };
    }

    macro_rules! str_args_to_dto {
        [ $prop:ident, $arg:expr ] => {
            if matches.is_present($arg) {
                dto.$prop = Some(matches.value_of($arg).unwrap().to_string());
            }
        };
    }

    hms_args_to_dto![start, "START",    Sub];
    hms_args_to_dto![end,   "END",      Add];

    str_args_to_dto![details,      "DETAILS"];
    str_args_to_dto![state,        "STATE"];
    str_args_to_dto![large_image,  "LG_IMG"];
    str_args_to_dto![small_image,  "SM_IMG"];
    str_args_to_dto![large_text,   "LG_TXT"];
    str_args_to_dto![small_text,   "SM_TXT"];

    Ok(dto)
}