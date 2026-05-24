use crate::{core::scheduler, db::Database};
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

pub fn spawn(db: Arc<Mutex<Database>>) {
    tauri::async_runtime::spawn(async move {
        loop {
            if let Ok(mut db) = db.lock() {
                let _ = scheduler::run_due_schedules(&mut db);
            }
            sleep(Duration::from_secs(30)).await;
        }
    });
}
