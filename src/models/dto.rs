use std::ops::AddAssign;

use discord_rpc_client::models::Activity;

use crate::{
    command_engine::commands::DtoFlags,
    models::preset::Preset,
    utils::{
        fail::AppError,
        time_diff,
    },
};


lazy_static! {
    static ref EMPTY: ActivityDto = ActivityDto::default();
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct ActivityDto {
    pub details: Option<String>,
    pub state: Option<String>,
    pub large_image: Option<String>,
    pub small_image: Option<String>,
    pub large_text: Option<String>,
    pub small_text: Option<String>,
    pub start: Option<u64>,
    pub start_hms: Option<String>,
    pub end: Option<u64>,
    pub end_hms: Option<String>,
}

impl ActivityDto {
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

    pub fn from_flags(flags: DtoFlags) -> Result<Self, AppError> {
        let start = match &flags.start {
            Some(s) => Some(time_diff::hms_to_u64_fwd(s.as_ref())?),
            None => None
        };

        let end = match &flags.end {
            Some(e) => Some(time_diff::hms_to_u64_rvs(e.as_ref())?),
            None => None
        };

        Ok(ActivityDto {
            details: flags.details,
            state: flags.state,
            large_image: flags.lg_img,
            small_image: flags.sm_img,
            large_text: flags.lg_txt,
            small_text: flags.sm_txt,
            start,
            start_hms: flags.start,
            end,
            end_hms: flags.end,
        })
    }

    pub fn from_preset(p: Preset) -> Result<ActivityDto, AppError> {
        let start = match &p.start {
            Some(t) => Some(time_diff::hms_to_u64_rvs(t)?),
            None => None,
        };
        let end = match &p.end {
            Some(t) => Some(time_diff::hms_to_u64_fwd(t)?),
            None => None,
        };

        Ok(ActivityDto {
            details: p.details,
            state: p.state,
            large_image: p.large_image,
            small_image: p.small_image,
            large_text: p.large_text,
            small_text: p.small_text,
            start,
            start_hms: p.start,
            end,
            end_hms: p.end,
        })
    }

    pub fn empty_ref() -> &'static ActivityDto { &EMPTY }
}

impl AddAssign for ActivityDto {
    fn add_assign(&mut self, rhs: ActivityDto) {
        macro_rules! if_some_overwrite {
            ($field:ident) => {
                if let Some(x) = rhs.$field {
                    self.$field = Some(x);
                }
            };
        }

        if_some_overwrite!(details);
        if_some_overwrite!(state);
        if_some_overwrite!(large_image);
        if_some_overwrite!(small_image);
        if_some_overwrite!(large_text);
        if_some_overwrite!(small_text);
        if_some_overwrite!(start);
        if_some_overwrite!(start_hms);
        if_some_overwrite!(end);
        if_some_overwrite!(end_hms);
    }
}
