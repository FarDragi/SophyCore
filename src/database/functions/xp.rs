use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};

use crate::{
    database::{entities::xp, Database},
    error::{AppError, MapError},
};

use super::{guild::GuildRepository, user::UserRepository};

#[async_trait]
pub trait XpRepository {
    async fn create_xp(&self, user_id: u64, guild_id: u64) -> Result<xp::Model, AppError>;
    async fn get_or_create_xp(&self, user_id: u64, guild_id: u64) -> Result<xp::Model, AppError>;
}

#[async_trait]
impl XpRepository for Database {
    async fn create_xp(&self, user_id: u64, guild_id: u64) -> Result<xp::Model, AppError> {
        let xp = xp::ActiveModel {
            user_id: Set(user_id.to_string()),
            guild_id: Set(guild_id.to_string()),
            updated_at: Set(Some(Utc::now())),
            ..Default::default()
        };

        xp.insert(&self.connection).await.map_app_err()
    }

    async fn get_or_create_xp(&self, user_id: u64, guild_id: u64) -> Result<xp::Model, AppError> {
        let user = self.get_or_create_user(user_id).await?;
        let guild = self.get_or_create_guild(guild_id).await?;

        let xp = xp::Entity::find_by_id((user.id, guild.id))
            .one(&self.connection)
            .await
            .map_app_err()?;

        if let Some(xp) = xp {
            Ok(xp)
        } else {
            self.create_xp(user_id, guild_id).await
        }
    }
}
