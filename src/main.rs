mod cli;
mod config;
mod modules;
mod utils;
use utils::handle_error_result;
#[macro_use]
extern crate serde_derive;
use config::Config;
use libmacchina::traits::GeneralReadout as _;
use libmacchina::traits::KernelReadout as _;
use libmacchina::traits::PackageReadout as _;
use libmacchina::GeneralReadout;
use libmacchina::KernelReadout;
use libmacchina::PackageReadout;
#[macro_use]
extern crate pest_derive;

#[tokio::main]
async fn main() {
    let general_readout = GeneralReadout::new();
    let package_readout = PackageReadout::new();
    let kernel_readout = KernelReadout::new();
    handle_error_result(
        simple_logger::SimpleLogger::new().init(),
        Some("Failed to start simple_logger"),
        None,
    );
    Config::from_config(Config::path().await)
        .await
        .print(&kernel_readout, &general_readout, &package_readout)
        .await;
}
