use clap::ArgMatches;
use discord_rpc_client::models::Activity;

use models::preset::Preset;
use utils::time_diff::{hms_to_u64, AddOrSub};
use utils::gnr_error::GnrError;

#[derive(Clone, PartialEq, Default)]
pub struct ActivityDto {
    pub details: Option<String>,
    pub state: Option<String>,
    pub large_image: Option<String>,
    pub small_image: Option<String>,
    pub large_text: Option<String>,
    pub small_text: Option<String>,
    pub start: Option<u64>,
    pub end: Option<u64>,
}

impl ActivityDto {
    pub fn from_preset(p: Preset) -> Result<ActivityDto, Box<GnrError>> {
        let start = match p.start {
            Some(t) => Some(hms_to_u64(&t, &AddOrSub::Sub)?),
            None => None
        };
        let end = match p.end{
            Some(t) => Some(hms_to_u64(&t, &AddOrSub::Add)?),
            None => None
        };

        Ok(ActivityDto {
            details: p.details,
            state: p.state,
            large_image: p.large_image,
            small_image: p.small_image,
            large_text: p.large_text,
            small_text: p.small_text,
            start,
            end,
        })
    }

    pub fn add_cmd_args(&mut self, matches: &ArgMatches) -> Result<ActivityDto, Box<GnrError>> {
        let mut dto = self.clone();

        if matches.is_present("CLEAR") {
            dto = ActivityDto::default();
        }

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

    pub fn apply_to_activity(self, activity: Activity) -> Activity {
        let mut a = activity;
        let dto = &self;

        macro_rules! flat_prop_add {
            [ $prop:ident ] => {
                if let Some(ref $prop) = dto.$prop {
                    a = a.$prop($prop.clone());
                }
            }
        };

        flat_prop_add![details];
        flat_prop_add![state];

        a = a.assets(|ass| {
            let mut assets = ass;

            macro_rules! asst_prop_add {
                [ $prop:ident ] => {
                    if let Some(ref $prop) = dto.$prop {
                        assets = assets.$prop($prop.clone());
                    }
                }
            };

            asst_prop_add![large_image];
            asst_prop_add![small_image];
            asst_prop_add![large_text];
            asst_prop_add![small_text];

            assets
        });

        a = a.timestamps(|timestamp| {
            let mut ts = timestamp;

            macro_rules! time_prop_add {
                [ $prop:ident ] => {
                    if let Some(ref $prop) = dto.$prop {
                        ts = ts.$prop($prop.clone());
                    }
                }
            };

            time_prop_add![start];
            time_prop_add![end];

            ts
        });

        a
    }
}