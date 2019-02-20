// Commands
mod clear;
mod create;
mod delete;
mod list;
mod set;

pub use self::{
    clear::ClearCmd,
    create::CreateCmd,
    delete::DeleteCmd,
    list::ListCmd,
    set::SetCmd
};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "CustomRichStatus")]
pub enum CliRoot {
    #[structopt(name = "clear", about = "Clears the current status")]
    Clear,
    #[structopt(name = "exit", about = "Alias for \"quit\"")]
    Exit,
    Preset(PresetCli),
    #[structopt(name = "quit", about = "Closes the program")]
    Quit,
    #[structopt(name = "set", about = "Updates fields in the custom status")]
    Set {
        #[structopt(flatten)]
        dto_flags: DtoFlags,
        #[structopt(
            short = "c",
            help = "Clears the existing status, then loads the details provided in the current command"
        )]
        clear: bool,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "preset", about = "Commands to manage presets")]
pub enum PresetCli {
    #[structopt(name = "create", about = "Creates a new preset")]
    Create(PresetCreateArgs),
    #[structopt(name = "delete", about = "Deletes a preset")]
    Delete {
        #[structopt(required = true, help = "Name of the preset to delete")]
        name: String,
    },
    #[structopt(name = "list", about = "Lists available presets")]
    List,
}

#[derive(Debug, StructOpt)]
pub struct PresetCreateArgs {
    #[structopt(required = true, help = "Name of the preset to create")]
    pub name: String,
    #[structopt(short = "o", help = "Will overwrite the preset with the same name")]
    pub overwrite: bool,
    #[structopt(
        short = "c",
        long = "current",
        conflicts_with = "preset",
        help = "Uses the current status. Additional fields override the preset.\
                Requires Retained State"
    )]
    pub use_current: bool,
    #[structopt(flatten)]
    pub dto_flags: DtoFlags,
}

#[derive(Debug, StructOpt)]
pub struct DtoFlags {
    #[structopt(
        short = "p",
        help = "Select a preset for the base. Additional fields override the preset"
    )]
    pub preset: Option<String>,
    #[structopt(
        short = "d",
        help = "Details content. This is where your status message should go"
    )]
    pub details: Option<String>,
    #[structopt(short = "S", help = "State content. This can be used as a second line")]
    pub state: Option<String>,
    #[structopt(
        short = "I",
        long = "large-img",
        help = "Key for the Large Image asset"
    )]
    pub lg_img: Option<String>,
    #[structopt(
        short = "i",
        long = "small-img",
        help = "Key for the Small Image asset"
    )]
    pub sm_img: Option<String>,
    #[structopt(short = "T", long = "large-txt", help = "Tooltip for the Large Image")]
    pub lg_txt: Option<String>,
    #[structopt(short = "t", long = "small-txt", help = "Tooltip for the Small Image")]
    pub sm_txt: Option<String>,
    #[structopt(
        short = "s",
        conflicts_with = "end",
        help = "Start epoch. For showing time elapsed"
    )]
    pub start: Option<String>,
    #[structopt(
        short = "e",
        conflicts_with = "start",
        help = "End epoch. For showing time remaining"
    )]
    pub end: Option<String>,
}
