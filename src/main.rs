mod config;
mod modules;
#[macro_use]
extern crate serde_derive;
use config::Config;
use modules::*;

fn main() {
    Config::from_config();
}
