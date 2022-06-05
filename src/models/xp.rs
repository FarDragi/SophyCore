use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{cache::xp::XpCacheCommands, error::AppError, services::Service};

#[derive(Debug, Serialize, Deserialize)]
pub struct Xp {
    pub level: i32,
    pub progress: i64,
    pub last_update: DateTime<Utc>,
}

impl Xp {
    pub async fn from_user_id(service: &Service, user_id: u64) -> Result<Self, AppError> {
        let xp = service.cache.get_user_global_xp(user_id).await?;

        if let Some(xp) = xp {
            Ok(xp)
        } else {
            todo!()
        }
    }
}
