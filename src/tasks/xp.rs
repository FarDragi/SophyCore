use std::time::Duration;

use tokio::time::interval;

use super::Task;

pub async fn xp_register_task(_task: Task) {
    let mut interval = interval(Duration::from_secs(60 * 30));

    loop {
        interval.tick().await;
    }
}
