use serde::{Deserialize, de};

#[derive(Deserialize)]
pub struct Config {
    pub flip_screen: bool,
    pub flip_buttons: bool,
    pub gadgets_path: String,
}

impl Config {
    pub fn default() -> Config {
        Config {
            flip_screen: true,
            flip_buttons: false,
            gadgets_path: ".".into(),
        }
    }

    pub fn from_str(s: &str) -> Result<Config, impl de::Error> {
        toml::from_str(s)
    }
}

