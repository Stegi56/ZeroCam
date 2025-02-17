use std::path::Path;
use log::{error, info};
use crate::Files::GDController;
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Result, Watcher, Event};
use tokio::sync::mpsc;
use tokio::time::timeout;
use std::time::Duration;
use std::panic::{AssertUnwindSafe, catch_unwind};
use futures::FutureExt;

pub struct BackupTriggerListener {
  _watcher: RecommendedWatcher,
}

impl BackupTriggerListener {
  pub async fn new() -> Result<Self> {
    let (tx, mut rx) = mpsc::channel(1); // Buffer size of 1 ensures only one pending backup

    let mut watcher = notify::recommended_watcher(move |res: Result<Event>| {
      if let Ok(event) = res {
        if matches!(event.kind, EventKind::Create(_)) {
          let res = tx.try_send(());
          if res.is_err() {
            info!("Backup already pending, dropping event");
          }
        }
      }
    })?;

    watcher.watch(Path::new("Clips/"), RecursiveMode::Recursive)?;

    tokio::spawn(async move {
      while rx.recv().await.is_some() {
        match timeout(Duration::from_secs(120), async {
          match GDController::GDController::new().await {
            Ok(mut controller) => {
              // Wrap the potentially panicking future in catch_unwind
              let backup_future = AssertUnwindSafe(async move {
                controller.backupNow().await;
              }).catch_unwind();

              match backup_future.await {
                Ok(_) => info!("Backup completed successfully"),
                Err(_) => error!("Backup thread panicked, killing thread"),
              }
            }
            Err(e) => info!("Failed to create controller: {}", e),
          }
        }).await {
          Err(_) => info!("Backup thread timed out"),
          Ok(_) => {}
        }
      }
    });

    Ok(Self { _watcher: watcher })
  }
}