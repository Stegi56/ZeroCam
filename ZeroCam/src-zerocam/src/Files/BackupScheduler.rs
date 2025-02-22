use crate::Files::GDController::GDController;
use log::{info, warn};
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

pub struct BackupScheduler {
  isRunning: Arc<AtomicBool>,
}

struct RunningGuard {
  flag: Arc<AtomicBool>,
}

impl Drop for RunningGuard {
  fn drop(&mut self) {
    self.flag.store(false, Ordering::SeqCst);
  }
}

/// This allows only 1 backup process to happen at any time and prevents any concurrent attempts
impl BackupScheduler {
  pub fn new() -> Self {
    Self {
      isRunning: Arc::new(AtomicBool::new(false)),
    }
  }

  pub async fn scheduleBackup(&self) -> Result<(), Box<dyn Error>> {
    if self.isRunning.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
      warn!("Backup already in progress - skipping this request");
      return Ok(());
    }

    let _guard = RunningGuard {
      flag: Arc::clone(&self.isRunning),
    };

    let result = timeout(Duration::from_secs(10), async {
      let mut controller = GDController::new().await?;
      controller.backupNow().await
    }).await??;

    info!("Backup completed successfully");
    Ok(result)
  }
}
