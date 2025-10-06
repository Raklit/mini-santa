use std::time::Duration;

use futures::{stream, StreamExt};
use tokio::time;


use crate::{santa::services::delete_messages_if_limit_or_lifetime, AppState};

pub async fn delete_old_messages(state : &AppState) -> () {
    let seconds = state.config.lock().await.santa.old_messages_check_freq;
    let interval = time::interval(Duration::from_secs(seconds));
    let cloned_state = state.clone();
    
    tokio::spawn(async move {
        let forever = stream::unfold(interval, |mut interval| async {
            interval.tick().await;
            tracing::info!("Delete old messages task started...");
            delete_messages_if_limit_or_lifetime(&cloned_state).await;
            tracing::info!("Delete old messages task ended.");
            Some(((), interval))
        });
        forever.for_each(|_| async {}).await;
    });
}