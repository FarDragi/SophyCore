use deadpool_redis::PoolError;
use redis::RedisError;
use sea_orm::error::DbErr as DbError;
use serde_json::Error as JsonError;
use tonic::Status;

#[derive(Debug)]
pub enum AppError {
    Database(DbError),
    Redis(RedisError),
    Pool(PoolError),
    Custom(&'static str),
    Json(JsonError),
}

pub trait MapError<T> {
    fn map_app_err(self) -> Result<T, AppError>;
    fn custom_error(self, message: &'static str) -> Result<T, AppError>;
}

impl<T> MapError<T> for Result<T, DbError> {
    fn map_app_err(self) -> Result<T, AppError> {
        self.map_err(AppError::Database)
    }

    fn custom_error(self, message: &'static str) -> Result<T, AppError> {
        self.map_err(|_| AppError::Custom(message))
    }
}

impl<T> MapError<T> for Result<T, RedisError> {
    fn map_app_err(self) -> Result<T, AppError> {
        self.map_err(AppError::Redis)
    }

    fn custom_error(self, message: &'static str) -> Result<T, AppError> {
        self.map_err(|_| AppError::Custom(message))
    }
}

impl<T> MapError<T> for Result<T, JsonError> {
    fn map_app_err(self) -> Result<T, AppError> {
        self.map_err(AppError::Json)
    }

    fn custom_error(self, message: &'static str) -> Result<T, AppError> {
        self.map_err(|_| AppError::Custom(message))
    }
}

pub trait ServiceError<T> {
    fn map_service_error(self) -> Result<T, Status>;
}

impl<T> ServiceError<T> for Result<T, AppError> {
    fn map_service_error(self) -> Result<T, Status> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => {
                error!("{:?}", err);

                match err {
                    AppError::Custom(message) => Err(Status::internal(message)),
                    err => Err(Status::internal(format!("{:?}", err))),
                }
            }
        }
    }
}
