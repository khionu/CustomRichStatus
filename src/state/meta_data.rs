pub struct AppMetaData {
    pub name: String,
    pub version: String,
    pub authors: String,
    pub about: String,
}

impl AppMetaData {
    pub fn get() -> &'static AppMetaData {
        let name = String::from("Custom Rich Status");
        let version = String::from(env!("CARGO_PKG_VERSION"));
        let authors = String::from(env!("CARGO_PKG_AUTHORS").replace(":", ", "));
        let about = String::from(env!("CARGO_PKG_DESCRIPTION"));

        let meta_data = AppMetaData {
            name,
            version,
            authors,
            about,
        };

        Box::leak(Box::new(meta_data))
    }
}