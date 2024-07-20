use std::time::{Duration, UNIX_EPOCH};

use tokio::time::Instant;

pub(crate) async fn daily_task<F>(hour: u8, task: F)
where
    F: FnOnce() -> tokio::task::JoinHandle<()> + Copy + Send + 'static,
{
    loop {
        let now = std::time::SystemTime::now();
        let seconds = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let next = seconds + if seconds % 86400 < hour as u64 * 3600 {
            hour as u64 * 3600 - seconds % 86400
        } else {
            86400 + hour as u64 * 3600 - seconds % 86400
        };
        tokio::time::sleep_until(Instant::now() + Duration::from_secs(next)).await;
        task().await.expect("TODO: panic message");
    }
}
