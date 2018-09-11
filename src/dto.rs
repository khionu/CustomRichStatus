use discord_rpc_client::models::Activity;

use configuration::preset::Preset;
use utils::hms_to_u64;
use utils::AddOrSub::*;

#[derive(Clone)]
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

impl Default for ActivityDto {
    fn default() -> Self {
        ActivityDto {
            details: None,
            state: None,
            large_image: None,
            small_image: None,
            large_text: None,
            small_text: None,
            start: None,
            end: None,
        }
    }
}

impl ActivityDto {
    pub fn from_preset(p: Preset) -> Result<ActivityDto, String> {
        let start = match p.start {
            Some(t) => Some(hms_to_u64(&t, &Sub)?),
            None => None
        };
        let end = match p.end{
            Some(t) => Some(hms_to_u64(&t, &Add)?),
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

    pub fn apply_to_activity(self, activity: Activity) -> Activity {
        let mut a = activity;
        let dto = self;

        macro_rules! flat_prop_add {
            [ $prop:ident ] => {
                if let Some($prop) = dto.$prop {
                    a = a.$prop($prop);
                }
            }
        };

        macro_rules! asst_prop_add {
            [ $prop:ident ] => {
                if let Some($prop) = dto.$prop {
                    a = a.assets(|assets| assets.$prop($prop));
                }
            }
        };

        macro_rules! time_prop_add {
            [ $prop:ident ] => {
                if let Some($prop) = dto.$prop {
                    a = a.timestamps(|ts| ts.$prop($prop));
                }
            }
        };

        flat_prop_add![details];
        flat_prop_add![state];

        asst_prop_add![large_image];
        asst_prop_add![small_image];
        asst_prop_add![large_text];
        asst_prop_add![small_text];

        time_prop_add![start];
        time_prop_add![end];

        a
    }
}