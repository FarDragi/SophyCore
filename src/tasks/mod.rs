mod xp;

use std::sync::Arc;

use tokio::task;

use crate::{cache::Cache, config::Config, database::Database};

use self::xp::xp_register_task;

pub struct Task {
    pub config: Arc<Config>,
    pub database: Arc<Database>,
    pub cache: Arc<Cache>,
}

impl Task {
    pub fn new(config: Arc<Config>, database: Arc<Database>, cache: Arc<Cache>) -> Self {
        Self {
            config,
            database,
            cache,
        }
    }
}

pub fn start_tasks(config: Arc<Config>, database: Arc<Database>, cache: Arc<Cache>) {
    task::spawn(async move { xp_register_task(Task::new(config, database, cache)).await });
}
