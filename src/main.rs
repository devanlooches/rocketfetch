mod config;
mod modules;
mod cli;
#[macro_use]
extern crate serde_derive;
use config::Config;

#[tokio::main]
async fn main() {
    Config::from_config().await.print().await;
}
