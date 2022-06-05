use crate::{
    error::{AppError, MapError},
    models::xp::Xp,
};

use super::Cache;

#[async_trait]
pub trait XpCacheCommands {
    async fn get_user_guild_xp(&self, user_id: u64, guild_id: u64) -> Result<Option<Xp>, AppError>;
    async fn set_user_guild_xp(&self, user_id: u64, guild_id: u64, xp: &Xp)
        -> Result<(), AppError>;
    async fn get_user_global_xp(&self, user_id: u64) -> Result<Option<Xp>, AppError>;
    async fn set_user_global_xp(&self, user_id: u64, xp: &Xp) -> Result<(), AppError>;
}

#[async_trait]
impl XpCacheCommands for Cache {
    async fn get_user_guild_xp(&self, user_id: u64, guild_id: u64) -> Result<Option<Xp>, AppError> {
        let key = format!("xp:local:{}:{}", user_id, guild_id);

        let cache = self.get(&key).await?;

        if let Some(cache) = cache {
            let cache: Xp = serde_json::from_str(&cache).map_app_err()?;

            Ok(Some(cache))
        } else {
            Ok(None)
        }
    }

    async fn set_user_guild_xp(
        &self,
        user_id: u64,
        guild_id: u64,
        xp: &Xp,
    ) -> Result<(), AppError> {
        let key = format!("xp:local:{}:{}", user_id, guild_id);

        let cache = serde_json::to_string(xp).map_app_err()?;

        self.set(&key, &cache).await
    }

    async fn get_user_global_xp(&self, user_id: u64) -> Result<Option<Xp>, AppError> {
        let key = format!("xp:global:{}", user_id);

        let cache = self.get(&key).await?;

        if let Some(cache) = cache {
            let cache: Xp = serde_json::from_str(&cache).map_app_err()?;

            Ok(Some(cache))
        } else {
            Ok(None)
        }
    }

    async fn set_user_global_xp(&self, user_id: u64, xp: &Xp) -> Result<(), AppError> {
        let key = format!("xp:global:{}", user_id);

        let cache = serde_json::to_string(xp).map_app_err()?;

        self.set(&key, &cache).await
    }
}
