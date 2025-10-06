use std::time::Duration;

use futures::{stream, StreamExt};
use tokio::time;


use crate::{santa::services::delete_pools_if_lifetime, AppState};

pub async fn delete_old_pools(state : &AppState) -> () {
    let seconds = state.config.lock().await.santa.pool_lifetime_check_freq;
    let interval = time::interval(Duration::from_secs(seconds));
    let cloned_state = state.clone();

    tokio::spawn(async move {
        let forever = stream::unfold(interval, |mut interval| async {
            interval.tick().await;
            tracing::info!("Delete old pools task started...");
            delete_pools_if_lifetime(&cloned_state).await;
            tracing::info!("Delete old pools task ended.");
            Some(((), interval))
        });
        forever.for_each(|_| async {}).await;
    });
}