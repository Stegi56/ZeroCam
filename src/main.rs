#![allow(non_snake_case)]
mod Files;

use env_logger;
use log::info;
use tokio::signal;

#[tokio::main]
async fn main(){
  env_logger::init();

  let backupTriggerListener = Files::BackupTriggerListener::BackupTriggerListener::new().await.unwrap();
  log::info!("Listener running. Press Ctrl+C to exit.");

  signal::ctrl_c()
    .await
    .expect("Failed to install Ctrl+C signal handler");

  info!("Shutting Down!")
}
