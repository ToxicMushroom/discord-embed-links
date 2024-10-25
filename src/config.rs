#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
}

impl Config {
    pub fn init() -> Self {
        let host = std::env::var("HOST").expect("HOST must be set").parse().expect("invalid HOST");

        Self {
            host
        }
    }
}