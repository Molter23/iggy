use crate::http::shared::AppState;
use iggy::utils::timestamp::TimeStamp;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};

pub fn start_expired_tokens_cleaner(app_state: Arc<AppState>) {
    tokio::spawn(async move {
        let mut interval_timer = tokio::time::interval(Duration::from_secs(300));
        loop {
            interval_timer.tick().await;
            info!("Deleting expired tokens...");
            let now = TimeStamp::now().to_secs();
            app_state
                .jwt_manager
                .delete_expired_revoked_tokens(now)
                .await
                .unwrap_or_else(|err| {
                    error!(
                        "Failed to delete expired revoked access tokens. Error: {}",
                        err
                    );
                });
            app_state
                .jwt_manager
                .delete_expired_refresh_tokens(now)
                .await
                .unwrap_or_else(|err| {
                    error!("Failed to delete expired refresh tokens. Error: {}", err);
                });
        }
    });
}
