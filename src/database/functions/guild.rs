use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::{
    database::{entities::guild, Database},
    error::{AppError, MapError},
};

#[async_trait]
pub trait GuildRepository {
    async fn exist_guild(&self, id: u64) -> Result<bool, AppError>;
    async fn create_guild(&self, id: u64) -> Result<guild::Model, AppError>;
    async fn get_or_create_guild(&self, id: u64) -> Result<guild::Model, AppError>;
}

#[async_trait]
impl GuildRepository for Database {
    async fn exist_guild(&self, id: u64) -> Result<bool, AppError> {
        let guild_exist = guild::Entity::find_by_id(id.to_string())
            .count(&self.connection)
            .await
            .map_app_err()?;

        Ok(guild_exist > 0)
    }

    async fn create_guild(&self, id: u64) -> Result<guild::Model, AppError> {
        let guild = guild::ActiveModel {
            id: Set(id.to_string()),
        };

        guild.insert(&self.connection).await.map_app_err()
    }

    async fn get_or_create_guild(&self, id: u64) -> Result<guild::Model, AppError> {
        let guild = guild::Entity::find_by_id(id.to_string())
            .one(&self.connection)
            .await
            .map_app_err()?;

        if let Some(guild) = guild {
            Ok(guild)
        } else {
            self.create_guild(id).await
        }
    }
}
