use sea_orm::error::DbErr as DbError;

#[derive(Debug)]
pub enum AppError {
    Database(DbError),
    Custom(&'static str),
}

pub trait MapError<T> {
    fn map_app_err(self) -> Result<T, AppError>;
    fn custom_error(self, message: &'static str) -> Result<T, AppError>;
}

impl<T> MapError<T> for Result<T, DbError> {
    fn map_app_err(self) -> Result<T, AppError> {
        self.map_err(|err| AppError::Database(err))
    }

    fn custom_error(self, message: &'static str) -> Result<T, AppError> {
        self.map_err(|_| AppError::Custom(message))
    }
}
