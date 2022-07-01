use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    cache::xp::XpCacheCommands,
    database::functions::{user::UserRepository, xp::XpRepository},
    error::{AppError, MapError},
    services::Service,
};

pub struct XpUpdate {
    pub add: bool,
    pub up: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Xp {
    pub level: i32,
    pub progress: i64,
    pub last_update: DateTime<Utc>,
    pub user_id: u64,
    pub guild_id: Option<u64>,
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
                guild_id: None,
                user_id: xp.id.parse::<u64>().map_app_err()?,
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
                guild_id: Some(xp.guild_id.parse::<u64>().map_app_err()?),
                user_id: xp.user_id.parse::<u64>().map_app_err()?,
            };

            service
                .cache
                .set_user_guild_xp(user_id, guild_id, &xp)
                .await?;

            Ok(xp)
        }
    }

    fn in_cooldown(&self, now: DateTime<Utc>) -> Option<(Xp, XpUpdate)> {
        debug!("Last update: {}", self.last_update);
        debug!("Now: {}", now);
        debug!("Target: {}", self.last_update + Duration::minutes(5));

        if self.last_update + Duration::minutes(5) >= now {
            debug!("Not updating XP because it was updated less than 5 minutes ago");

            Some((
                self.clone(),
                XpUpdate {
                    add: false,
                    up: false,
                },
            ))
        } else {
            None
        }
    }

    pub fn add(&self) -> (Self, XpUpdate) {
        let now = Utc::now();

        if let Some(data) = self.in_cooldown(now) {
            return data;
        }

        let new_progress = self.progress + 1;

        let progress_target = LEVELS[self.level as usize];

        debug!("Progress target: {}", progress_target);
        debug!("New progress: {}", new_progress);

        if new_progress >= progress_target {
            (
                Xp {
                    last_update: now,
                    level: self.level + 1,
                    progress: 0,
                    guild_id: self.guild_id,
                    user_id: self.user_id,
                },
                XpUpdate {
                    add: true,
                    up: true,
                },
            )
        } else {
            (
                Xp {
                    last_update: now,
                    level: self.level,
                    progress: new_progress,
                    guild_id: self.guild_id,
                    user_id: self.user_id,
                },
                XpUpdate {
                    add: true,
                    up: false,
                },
            )
        }
    }

    pub async fn save(&self, service: &Service) -> Result<(), AppError> {
        match self.guild_id {
            Some(guild_id) => {
                service
                    .cache
                    .set_user_guild_xp(self.user_id, guild_id, self)
                    .await
            }
            None => service.cache.set_user_global_xp(self.user_id, self).await,
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
