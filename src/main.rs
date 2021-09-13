mod cli;
mod config;
mod modules;
mod utils;
use utils::handle_error_result;
#[macro_use]
extern crate serde_derive;
use config::Config;
#[macro_use]
extern crate pest_derive;

#[tokio::main]
async fn main() {
    handle_error_result(
        simple_logger::SimpleLogger::new().init(),
        Some("Failed to start simple_logger"),
        None,
    );
    Config::from_config(Config::path().await)
        .await
        .print()
        .await;
}
