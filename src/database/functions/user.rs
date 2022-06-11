use chrono::Utc;
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::{
    database::{entities::user, Database},
    error::{AppError, MapError},
};

#[async_trait]
pub trait UserRepository {
    async fn exist_user(&self, id: u64) -> Result<bool, AppError>;
    async fn create_user(&self, id: u64) -> Result<user::Model, AppError>;
    async fn get_or_create_user(&self, id: u64) -> Result<user::Model, AppError>;
}

#[async_trait]
impl UserRepository for Database {
    async fn exist_user(&self, id: u64) -> Result<bool, AppError> {
        let user_exist = user::Entity::find_by_id(id.to_string())
            .count(&self.connection)
            .await
            .map_app_err()?;

        Ok(user_exist > 0)
    }

    async fn create_user(&self, id: u64) -> Result<user::Model, AppError> {
        let user = user::ActiveModel {
            id: Set(id.to_string()),
            xp_updated_at: Set(Some(Utc::now())),
            ..Default::default()
        };

        user.insert(&self.connection).await.map_app_err()
    }

    async fn get_or_create_user(&self, id: u64) -> Result<user::Model, AppError> {
        let user = user::Entity::find_by_id(id.to_string())
            .one(&self.connection)
            .await
            .map_app_err()?;

        if let Some(user) = user {
            Ok(user)
        } else {
            self.create_user(id).await
        }
    }
}
