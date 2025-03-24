use crate::GDFiles::BackupScheduler::BackupScheduler;
use crate::Net::NetworkConnector::ping_google;
use crate::Net::NetworkController::{evaluateNetworkStateAndHandleChange, initialiseNetworkPriorities};
use crate::Config::getConfig;

use log::{error, info};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tokio::runtime::Handle;

pub async fn listen(backupScheduler: Arc<BackupScheduler>) {
  let runtimeHandle = Handle::current();
  let scheduler = backupScheduler.clone();
  let config = getConfig().await.unwrap();

  let mut previousConnectionState = Vec::new();
  let mut isConnected = false;

  initialiseNetworkPriorities(&config).await.unwrap();

  loop {
    sleep(Duration::from_secs(20));
    let pingStatus: bool = ping_google().await;

    previousConnectionState = evaluateNetworkStateAndHandleChange(previousConnectionState.clone(), &config).await
      .unwrap_or_else(|_| Vec::new());

    if isConnected == false && pingStatus == true {
      info!("Now online!");
      isConnected = true;

      let scheduler = scheduler.clone();
      runtimeHandle.spawn(async move {
        if let Err(e) = scheduler.scheduleBackup().await {
          error!("Backup failed: {}", e);
        }
      });
    } else if isConnected == true && pingStatus == false {
      info!("Now offline!");
      isConnected = false;
    }
  }
}

