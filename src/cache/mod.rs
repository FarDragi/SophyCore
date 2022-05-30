use std::fmt::Debug;

use deadpool_redis::{Config as PoolConfig, Connection, Pool, Runtime};
use redis::AsyncCommands;

use crate::{
    config::Config,
    error::{AppError, MapError},
};

pub struct Cache {
    pool: Pool,
}

impl Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cache").finish()
    }
}

impl Cache {
    pub async fn new(config: &Config) -> Cache {
        let pool = PoolConfig::from_url(&config.redis_url)
            .create_pool(Some(Runtime::Tokio1))
            .expect("Failed to create redis pool");

        info!("Connected to redis");

        Cache { pool }
    }

    async fn get_connection(&self) -> Result<Connection, AppError> {
        self.pool.get().await.map_err(AppError::Pool)
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<(), AppError> {
        let mut conn = self.get_connection().await?;

        conn.set(key, value).await.map_app_err()?;

        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>, AppError> {
        let mut conn = self.get_connection().await?;

        conn.get(key).await.map_app_err()
    }
}
