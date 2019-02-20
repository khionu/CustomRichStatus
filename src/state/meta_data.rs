pub struct AppMetaData {
    pub name: String,
    pub version: String,
    pub authors: String,
    pub about: String,
    pub prompt: String,
    pub quit_msg: String,
}

pub const APP_NAME: &str = "Custom Rich Status";
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

impl AppMetaData {
    pub fn get(prompt: String, quit_msg: String) -> &'static AppMetaData {
        let name = String::from(APP_NAME);
        let version = String::from(PKG_VERSION);
        let authors = String::from(PKG_AUTHORS.replace(":", ", "));
        let about = String::from(PKG_DESCRIPTION);

        let meta_data = AppMetaData {
            name,
            version,
            authors,
            about,
            prompt,
            quit_msg,
        };

        Box::leak(Box::new(meta_data))
    }
}
