use clap::App;

use state::meta_data::AppMetaData;

// Commands

pub mod clear;
pub mod quit;
pub mod set;

// End Commands

pub fn register(meta_data: &AppMetaData) -> App {
    clap_app!(app =>
        (name: meta_data.name.as_ref())
        (version: meta_data.version.as_ref())
        (author: meta_data.authors.as_ref())
        (about: meta_data.about.as_ref())
        (@subcommand quit =>
            (about: "Closes the program")
        )
        (@subcommand clear =>
            (about: "Clears the current status")
        )
        (@subcommand set =>
            (about: "Updates fields in the custom status")
            (@arg CLEAR: --clear -c "Clears the existing status, then loads the details provided in the current command")
            (@arg PRESET: --preset -p [NAME] "Load a preset to use. Additional fields override the preset")
            (@arg DETAILS: --details -d [DETAILS] "Details content. This is where your status message should go")
            (@arg STATE: --state -S [STATE] "State content. This can be used as a second line")
            (@arg LG_IMG: --("large-img") -I [ASSET] "Key for the Large Image asset")
            (@arg SM_IMG: --("small-img") -i [ASSET] "Key for the Small Image asset")
            (@arg LG_TXT: --("large-txt") -T [ASSET] "Tooltip for the Large Image")
            (@arg SM_TXT: --("small-txt") -t [ASSET] "Tooltip for the Small Image")
            (@group timestamp =>
                (@arg START: --start -s [HH_MM_SS] "Start epoch. For showing time elapsed")
                (@arg END: --end -e [HH_MM_SS] "End epoch. For showing time remaining")
            )
        )
    ).bin_name(meta_data.prompt.as_ref())
}

pub enum CmdResult {
    Ok(String),
    Err(String),
    Fatal(String),
}