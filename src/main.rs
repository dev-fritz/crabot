mod config;
mod model;
mod service;
mod repository;
mod yamcha;
mod utils;

use dotenv::dotenv;
use sqlx::Executor;
use crate::yamcha::run::start_bot;
use crate::config::database::establish_connection;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");
    let pool = establish_connection().await.expect("Failed to connect to database");

    let result = pool.fetch_all("Select 'oi'").await.expect("Failed to fetch query");
    println!("{:?}", result.first());
    log::info!("Connected to database");

    start_bot().await;
}
