mod config;
mod model;
mod service;
mod repository;
mod yamcha;

use dotenv::dotenv;
use crate::yamcha::run::start_bot;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    start_bot().await;
}
