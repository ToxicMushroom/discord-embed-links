#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub title: String,
    pub thumb_id: String,
}

impl Config {
    pub fn init() -> Self {
        let host = Self::get_env("HOST");
        let title = Self::get_env("EMBED_TITLE");
        let thumb_id = Self::get_env("EMBED_THUMB_ID");

        Self {
            host,
            title,
            thumb_id
        }
    }

    fn get_env(key: &str) -> String {
        std::env::var(key)
            .expect(format!("{key} must be set").as_str())
            .parse()
            .expect(format!("invalid {key}").as_str())
    }
}