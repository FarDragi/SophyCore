use serde::{Deserialize, Serialize};

use crate::error::{AppError, MapError};

use super::Cache;

#[derive(Debug, Serialize, Deserialize)]
pub struct XpCache {
    last_message_at: i64,
    level: i32,
    progress: i64,
}

#[async_trait]
pub trait UserCacheCommands {
    async fn get_user_guild_xp(
        &self,
        user_id: u64,
        guild_id: u64,
    ) -> Result<Option<XpCache>, AppError>;
    async fn set_user_guild_xp(
        &self,
        user_id: u64,
        guild_id: u64,
        xp: XpCache,
    ) -> Result<(), AppError>;
    async fn get_user_global_xp(&self, user_id: u64) -> Result<Option<XpCache>, AppError>;
    async fn set_user_global_xp(&self, user_id: u64, xp: XpCache) -> Result<(), AppError>;
}

#[async_trait]
impl UserCacheCommands for Cache {
    async fn get_user_guild_xp(
        &self,
        user_id: u64,
        guild_id: u64,
    ) -> Result<Option<XpCache>, AppError> {
        let key = format!("xp:local:{}:{}", user_id, guild_id);

        let cache = self.get(&key).await?;

        if let Some(cache) = cache {
            let cache: XpCache = serde_json::from_str(&cache).map_app_err()?;

            Ok(Some(cache))
        } else {
            Ok(None)
        }
    }

    async fn set_user_guild_xp(
        &self,
        user_id: u64,
        guild_id: u64,
        xp: XpCache,
    ) -> Result<(), AppError> {
        let key = format!("xp:local:{}:{}", user_id, guild_id);

        let cache = serde_json::to_string(&xp).map_app_err()?;

        self.set(&key, &cache).await
    }

    async fn get_user_global_xp(&self, user_id: u64) -> Result<Option<XpCache>, AppError> {
        let key = format!("xp:global:{}", user_id);

        let cache = self.get(&key).await?;

        if let Some(cache) = cache {
            let cache: XpCache = serde_json::from_str(&cache).map_app_err()?;

            Ok(Some(cache))
        } else {
            Ok(None)
        }
    }

    async fn set_user_global_xp(&self, user_id: u64, xp: XpCache) -> Result<(), AppError> {
        let key = format!("xp:global:{}", user_id);

        let cache = serde_json::to_string(&xp).map_app_err()?;

        self.set(&key, &cache).await
    }
}
