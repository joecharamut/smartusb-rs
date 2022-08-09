use clap::Parser;

/// Enable a Raspberry Pi to become a versatile USB device
#[derive(Parser, Debug)]
#[clap(
    name = "smartusb",
    version = "0.1.0",
    about,
)]
pub struct Args {
    /// The path to the main config file
    #[clap(short, long, value_parser, default_value = "/etc/smartusb/config.toml")]
    pub config: String,

    /// More verbose logging
    #[clap(short, long, value_parser, default_value_t = false)]
    pub verbose: bool,

    /// Run the smartusb daemon
    #[clap(short, long, value_parser, default_value_t = false)]
    pub daemon: bool,
}

