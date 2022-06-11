use std::time::Duration;

use tokio::time::interval;

use crate::{cache::xp::XpCacheCommands, error::AppError};

use super::Task;

pub async fn xp_register_task(task: Task) {
    let mut interval = interval(Duration::from_secs(60 * 30));

    loop {
        interval.tick().await;

        info!("Running xp register task");

        let result = || async {
            let global_keys = task.cache.list_global_xp().await?;

            for item in global_keys {
                let infos = item.split(':').collect::<Vec<&str>>();

                if infos.len() != 3 {
                    continue;
                }

                info!("Running xp register task for user {}", infos[2]);

                let xp = task.cache.get_xp(&item).await?;

                info!("{:?}", xp);
            }

            let guild_keys = task.cache.list_guild_xp().await?;

            for item in guild_keys {
                let infos = item.split(':').collect::<Vec<&str>>();

                if infos.len() != 4 {
                    continue;
                }

                info!(
                    "Running xp register task for guild {} user {}",
                    infos[2], infos[3]
                );

                let xp = task.cache.get_xp(&item).await?;

                info!("{:?}", xp);
            }

            Ok(()) as Result<(), AppError>
        };

        if let Err(err) = result().await {
            error!("Error running xp register task: {:?}", err);
        }
    }
}
