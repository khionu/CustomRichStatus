#[derive(Debug, Fail)]
pub enum AppError {
    #[fail(display = "critical development mistake: {}", 0)]
    DeveloperError(String),
    #[fail(display = "error occurred while communicating with the Discord client")]
    DiscordError(#[cause] discord_rpc_client::error::Error),
    #[fail(display = "failed to find {} \"{}\"", item_type, ident)]
    LookupFailure { item_type: String, ident: String },
    #[fail(display = "cannot write to {}, already exists", ident)]
    OverwriteFailure { ident: String },
    #[fail(display = "failed to parse {}: {}", arg, reason)]
    ParseFailure {
        arg: String,
        reason: String,
    },
    #[fail(display = "failed to serialize the {}, \"{}\"", data_type, file)]
    SerializeFailure {
        data_type: String,
        file: String,
        #[cause]
        inner: serde_yaml::Error,
    },
    #[fail(display = "failed to deserialize the {}, \"{}\"", data_type, file)]
    DeserializeFailure {
        data_type: String,
        file: String,
        #[cause]
        inner: serde_yaml::Error,
    },
    #[fail(display = "filesystem error")]
    IoFailure(#[cause] std::io::Error),
    #[fail(display = "")]
    ClapFailure(#[cause] structopt::clap::Error),
    #[fail(display = "an error has forced the program to close")]
    CriticalFailure(Box<AppError>)
}

impl AppError {
    pub fn new_crit(inner: AppError) -> Self {
        AppError::CriticalFailure(Box::new(inner))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoFailure(err)
    }
}

impl From<structopt::clap::Error> for AppError {
    fn from(err: structopt::clap::Error) -> Self {
        AppError::ClapFailure(err)
    }
}

impl From<discord_rpc_client::error::Error> for AppError {
    fn from(err: discord_rpc_client::error::Error) -> Self {
        AppError::DiscordError(err)
    }
}
