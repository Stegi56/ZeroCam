use log::{error, info};
use notify::{
  Event, EventKind, RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher,
};
use std::path::Path;
use std::sync::Arc;
use tokio::runtime::Handle;
use std::env;

use crate::GDFiles::BackupScheduler::BackupScheduler;
use crate::GDFiles::GDController::GDController;

pub struct FileListener {
  _watcher: RecommendedWatcher,
}

impl FileListener {
  pub async fn new(backupScheduler: Arc<BackupScheduler>) -> NotifyResult<Self> {
    let runtimeHandle = Handle::current();

    let mut watcher = notify::recommended_watcher(move |res: NotifyResult<Event>| {
      match res {
        Ok(event) if matches!(event.kind, EventKind::Create(_)) => {
          let scheduler = backupScheduler.clone();
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

    watcher.watch(Path::new(env::current_dir()?.parent().unwrap().parent().unwrap().join("Clips/").as_path()), RecursiveMode::Recursive)?;
    info!("File creation listener created");
    Ok(Self { _watcher: watcher })
  }
}
