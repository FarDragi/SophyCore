use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, Set};

use crate::{
    database::{entities::user, Database},
    error::{AppError, MapError},
};

#[async_trait]
pub trait UserRepository {
    async fn exist_user(&self, id: i64) -> Result<bool, AppError>;
    async fn create_user(&self, id: i64) -> Result<(), AppError>;
}

#[async_trait]
impl UserRepository for Database {
    async fn exist_user(&self, id: i64) -> Result<bool, AppError> {
        let user_exist = user::Entity::find_by_id(id)
            .count(&self.connection)
            .await
            .map_app_err()?;

        Ok(user_exist > 0)
    }

    async fn create_user(&self, id: i64) -> Result<(), AppError> {
        let user = user::ActiveModel {
            id: Set(id),
            ..Default::default()
        };

        user.insert(&self.connection).await.map_app_err()?;

        Ok(())
    }
}
