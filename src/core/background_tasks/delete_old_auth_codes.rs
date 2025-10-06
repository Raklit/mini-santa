use std::time::Duration;

use futures::{stream, StreamExt};
use tokio::time;

use crate::{core::services::delete_expired_auth_codes, AppState};

pub async fn delete_old_auth_codes(state : &AppState) -> () {
    let seconds = state.config.lock().await.auth.check_auth_code_status_freq;
    let interval = time::interval(Duration::from_secs(seconds));
    let cloned_state = state.clone();
    
    tokio::spawn(async move {
        let forever = stream::unfold(interval, |mut interval| async {
            interval.tick().await;
            tracing::info!("Delete old auth codes task started...");
            delete_expired_auth_codes(&cloned_state).await;
            tracing::info!("Delete old auth codes task ended.");
            Some(((), interval))
        });
        forever.for_each(|_| async {}).await;
}   );
}