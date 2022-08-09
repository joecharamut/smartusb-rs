use std::collections::HashMap;

use serde::{Deserialize, de};


#[derive(Debug, Deserialize)]
pub struct General {
    pub name: String,
    pub idVendor: String,
    pub idProduct: String,
    pub bcdDevice: String,
    pub bcdUSB: String,
}

#[derive(Debug, Deserialize)]
pub struct Strings {
    pub serial: String,
    pub manufacturer: String,
    pub product: String,
}

#[derive(Debug, Deserialize)]
pub struct UsbConfig {
    pub configuration: String,
    pub MaxPower: String,
}

#[derive(Debug, Deserialize)]
pub struct Function {
    pub config: String,
    pub foo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GadgetConfig {
    pub general: General,
    pub strings: Strings,
    pub configs: HashMap<String, UsbConfig>,
    pub functions: HashMap<String, Function>,
}

impl GadgetConfig {
    pub fn from_str(s: &str) -> Result<GadgetConfig, impl de::Error> {
        toml::from_str(s)
    }
}
