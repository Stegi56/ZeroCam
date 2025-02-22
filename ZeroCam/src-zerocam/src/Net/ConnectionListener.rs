use log::{debug, error, info};
use std::net::IpAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Handle;
use tokio::time::sleep;

use crate::Files::BackupScheduler::BackupScheduler;

pub struct ConnectionListener {
  isConnected: AtomicBool,
}

impl ConnectionListener {
  pub fn new() -> Self {
    Self {
      isConnected: AtomicBool::new(true)
    }
  }

  pub async fn listen(&self, backupScheduler: Arc<BackupScheduler>) {
    let runtimeHandle = Handle::current();
    let scheduler = backupScheduler.clone();

    loop {
      sleep(Duration::from_secs(10)).await;
      let pingStatus: bool = self.ping_google().await;
      let currentStatus: bool = self.isConnected.load(Ordering::SeqCst);

      if currentStatus == false && pingStatus == true {
        info!("Now online!");
        self.isConnected.store(true, Ordering::SeqCst);

        let scheduler = scheduler.clone();
        runtimeHandle.spawn(async move {
          if let Err(e) = scheduler.scheduleBackup().await {
            error!("Backup failed: {}", e);
          }
        });
      } else if currentStatus == true && pingStatus == false {
        info!("Now offline!");
        self.isConnected.store(false, Ordering::SeqCst);
      }
    }
  }

  async fn ping_google(&self) -> bool {
    match tokio::net::TcpStream::connect("8.8.8.8:53").await {
      Ok(_) => true,
      Err(_) => false
    }
  }
}
