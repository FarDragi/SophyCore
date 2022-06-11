use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    cache::xp::XpCacheCommands,
    database::functions::{user::UserRepository, xp::XpRepository},
    error::AppError,
    services::Service,
};

pub struct XpUpdate {
    pub add: bool,
    pub up: bool,
}

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

    pub async fn from_user_id_and_guild_id(
        service: &Service,
        user_id: u64,
        guild_id: u64,
    ) -> Result<Self, AppError> {
        let xp = service.cache.get_user_guild_xp(user_id, guild_id).await?;

        if let Some(xp) = xp {
            Ok(xp)
        } else {
            let xp = service.database.get_or_create_xp(user_id, guild_id).await?;

            let xp = Self {
                level: xp.level,
                progress: xp.progress,
                last_update: xp.updated_at.unwrap_or_else(Utc::now),
            };

            service
                .cache
                .set_user_guild_xp(user_id, guild_id, &xp)
                .await?;

            Ok(xp)
        }
    }

    pub fn add(&mut self) -> XpUpdate {
        let now = Utc::now();

        if self.last_update + Duration::minutes(5) >= now {
            info!("Last update: {}", self.last_update);
            info!("Now: {}", now);

            debug!("Not updating XP because it was updated less than 5 minutes ago");

            return XpUpdate {
                add: false,
                up: false,
            };
        }

        self.progress += 1;
        self.last_update = now;

        let progress_target = LEVELS[self.level as usize];

        debug!("Progress target: {}", progress_target);

        if self.progress >= progress_target {
            self.level += 1;
            self.progress = 0;

            XpUpdate {
                add: true,
                up: true,
            }
        } else {
            XpUpdate {
                add: true,
                up: false,
            }
        }
    }
}

const LEVELS: [i64; 200] = get_levels();

const fn calc_level(level: i64) -> i64 {
    let progress_multiplier = ((level - 1) / 5 + 1) * 20;
    let level_multiplier = ((level - 1) % 5) + 1;
    let base = {
        let mut result = 0;
        let mut i = 0;
        let i_target = ((level - 1) / 5) + 1;

        while i < i_target {
            result += 100 * i;
            i += 1;
        }
        result
    };

    (progress_multiplier * level_multiplier) + base
}

const fn get_levels() -> [i64; 200] {
    let mut levels = [0; 200];

    let mut i = 0;
    while i < 200 {
        levels[i] = calc_level((i + 1) as i64);
        i += 1;
    }

    levels
}
