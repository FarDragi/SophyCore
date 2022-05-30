use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};

use crate::{
    database::{entities::xp, Database},
    error::{AppError, MapError},
};

use super::{guild::GuildRepository, user::UserRepository};

#[async_trait]
pub trait XpRepository {
    async fn add_xp(&self, user_id: i64, guild: i64, level: i32, xp: i64) -> Result<(), AppError>;
}

#[async_trait]
impl XpRepository for Database {
    async fn add_xp(
        &self,
        user_id: i64,
        guild_id: i64,
        level: i32,
        progress: i64,
    ) -> Result<(), AppError> {
        if !self.exist_user(user_id).await? {
            self.create_user(user_id).await?;
        }

        if !self.exist_guild(guild_id).await? {
            self.create_guild(guild_id).await?;
        }

        let xp = xp::Entity::find_by_id((user_id, guild_id))
            .one(&self.connection)
            .await
            .map_app_err()?;

        if let Some(xp) = xp {
            let mut xp: xp::ActiveModel = xp.into();
            xp.level = Set(level);
            xp.progress = Set(progress);

            xp.update(&self.connection).await.map_app_err()?;
        } else {
            let xp = xp::ActiveModel {
                user_id: Set(user_id),
                guild_id: Set(guild_id),
                updated_at: Set(Some(chrono::Utc::now())),
                ..Default::default()
            };

            xp.insert(&self.connection).await.map_app_err()?;
        }

        todo!()
    }
}
