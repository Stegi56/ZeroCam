use std::path::Path;
use std::sync::Arc;
use log::{error, info};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher};
use tokio::runtime::Handle;

use crate::Files::GDController::GDController;
use crate::Files::BackupScheduler::BackupScheduler;

pub struct FileListener {
  _watcher: RecommendedWatcher,
}

impl FileListener {
  pub async fn new(backupScheduler: Arc<BackupScheduler>) -> NotifyResult<Self> {
    let runtimeHandle = Handle::current();

    let scheduler = backupScheduler.clone();

    let mut watcher = notify::recommended_watcher(move |res: NotifyResult<Event>| {
      match res {
        Ok(event) if matches!(event.kind, EventKind::Create(_)) => {
          let scheduler = scheduler.clone();
          runtimeHandle.spawn(async move {
            if let Err(e) = scheduler.scheduleBackup().await {
              error!("Backup failed: {}", e);
            }
          });
        }
        Ok(_) => (),
        Err(e) => error!("Watch error: {}", e),
      }
    })?;

    watcher.watch(Path::new("Clips/"), RecursiveMode::Recursive)?;
    info!("File creation listener created");
    Ok(Self { _watcher: watcher })
  }
}