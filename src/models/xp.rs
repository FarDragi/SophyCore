use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    cache::xp::XpCacheCommands, database::functions::user::UserRepository, error::AppError,
    services::Service,
};

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
            let xp = service.database.get_or_create_user(user_id).await?;

            let xp = Self {
                level: xp.level,
                progress: xp.progress,
                last_update: xp.xp_updated_at.unwrap_or_else(Utc::now),
            };

            service.cache.set_user_global_xp(user_id, &xp).await?;

            Ok(xp)
        }
    }
}
