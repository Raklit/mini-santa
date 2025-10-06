use std::time::Duration;

use futures::{stream, StreamExt};
use tokio::time;

use crate::{core::services::delete_account_sessions_with_expired_refresh_tokens, AppState};

pub async fn delete_old_account_sessions(state : &AppState) -> () {
    let seconds = state.config.lock().await.auth.check_session_status_freq;
    let interval = time::interval(Duration::from_secs(seconds));
    let cloned_state = state.clone();

    tokio::spawn( async move {
        let forever = stream::unfold(interval, |mut interval| async {
            interval.tick().await;
            tracing::info!("Delete old account sessions task started...");
            delete_account_sessions_with_expired_refresh_tokens(&cloned_state).await;
            tracing::info!("Delete old account sessions task ended.");
            Some(((), interval))
        });
        forever.for_each(|_| async {}).await;
    });
}