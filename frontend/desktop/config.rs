#[derive(Default)]
pub struct Config {}

impl Config {
    pub fn from_env() -> Self {
        Self {}
    }
}

pub fn from_env() -> Config {
    Config::from_env()
}
