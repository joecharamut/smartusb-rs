use serde::{Deserialize, de};

#[derive(Deserialize)]
pub struct Config {
    pub flip_screen: bool,
    pub flip_buttons: bool,
}

impl Config {
    pub fn default() -> Config {
        Config {
            flip_screen: true,
            flip_buttons: false,
        }
    }

    pub fn from_str(s: &str) -> Result<Config, impl de::Error> {
        toml::from_str(s)
    }
}

