use serde::{Deserialize, Serialize};

use crate::error::{AppError, MapError};

use super::Cache;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCache {
    id: u64,
    last_message_at: i64,
    level: i32,
    progress: i64,
}

#[async_trait]
pub trait UserCacheCommands {
    async fn get_user_cache(&self, id: u64) -> Result<Option<UserCache>, AppError>;
    async fn set_user_cache(&self, cache: UserCache) -> Result<(), AppError>;
}

#[async_trait]
impl UserCacheCommands for Cache {
    async fn get_user_cache(&self, id: u64) -> Result<Option<UserCache>, AppError> {
        let key = format!("user:{}", id);
        let cache = self.get(&key).await?;

        if let Some(cache) = cache {
            let cache: UserCache = serde_json::from_str(&cache).map_app_err()?;

            Ok(Some(cache))
        } else {
            Ok(None)
        }
    }

    async fn set_user_cache(&self, cache: UserCache) -> Result<(), AppError> {
        let key = format!("user:{}", cache.id);

        let json = serde_json::to_string(&cache).map_app_err()?;

        self.set(&key, &json).await
    }
}
