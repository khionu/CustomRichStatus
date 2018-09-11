use clap::App;

// Commands

pub mod set;
pub mod quit;

// End Commands

pub fn register() -> App<'static, 'static> {
    clap_app!(app =>
        (name: "Custom Rich Status")
        (version: "0.1.0")
        (author: "Khionu Sybiern <dev@khionu.net>")
        (about: "Set and control a custom status that won't be overwritten by Verified Non-rich games")
        (@subcommand quit =>
            (about: "Closes the program")
        )
        (@subcommand set =>
            (about: "Updates fields in the custom status")
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
    ).bin_name(">")
}