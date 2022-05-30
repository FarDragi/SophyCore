mod entities;
mod functions;

#[cfg(feature = "migrate")]
use std::process::exit;

use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

use crate::config::Config;

pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    pub async fn new(config: &Config) -> Self {
        let connection = sea_orm::Database::connect(&config.database_url)
            .await
            .expect("Fail connect to database");

        info!("Connected to database");

        info!("Start migrate database");
        Migrator::up(&connection, None).await.expect("Fail migrate");
        info!("Finish migrate database");

        #[cfg(feature = "migrate")]
        {
            exit(0);
            info!("Close application");
        }

        Database { connection }
    }
}
