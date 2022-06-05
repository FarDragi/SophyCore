mod cache;
mod config;
mod database;
mod error;
mod logs;
mod models;
mod services;
mod tasks;

use std::sync::Arc;

use cache::Cache;
use services::Service;
use tasks::start_tasks;

use crate::{config::Config, database::Database, logs::Logs};

#[macro_use]
extern crate log;

#[macro_use]
extern crate async_trait;

#[tokio::main]
async fn main() {
    Logs::start();

    let config = Arc::new(Config::new());
    let database = Arc::new(Database::new(&config).await);
    let cache = Arc::new(Cache::new(&config).await);

    let service = Service {
        config: config.clone(),
        database: database.clone(),
        cache: cache.clone(),
    };

    start_tasks(config, database, cache);

    service.start().await;
}
