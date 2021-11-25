mod cli;
mod config;
mod modules;
mod utils;
#[macro_use]
extern crate serde_derive;
use config::Config;
fn main() {
    Config::from_config(Config::path()).print();
}
