mod cache;
mod config;
mod database;
mod error;
mod logs;
mod models;
mod services;

use std::sync::Arc;

use cache::Cache;
use services::Service;

use crate::{config::Config, database::Database, logs::Logs};

#[macro_use]
extern crate log;

#[macro_use]
extern crate async_trait;

#[tokio::main]
async fn main() {
    Logs::new();

    let config = Arc::new(Config::new());
    let database = Database::new(&config).await;
    let cache = Cache::new(&config).await;

    let service = Service {
        config,
        database,
        cache,
    };

    service.start().await;
}
