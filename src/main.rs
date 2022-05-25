mod config;
mod database;
mod logs;
mod services;

use std::sync::Arc;

use services::Service;

use crate::{config::Config, database::Database, logs::Logs};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    Logs::new();

    let config = Arc::new(Config::new());
    let database = Database::new(&config).await;

    let service = Service { config, database };

    service.start().await;
}
