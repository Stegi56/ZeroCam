#![allow(non_snake_case)]
mod Camera;
mod Files;
mod Net;

use crate::Camera::CameraController::CameraController;
use crate::Files::BackupScheduler::BackupScheduler;
use crate::Files::FileListener::FileListener;
use crate::Net::ConnectionListener::ConnectionListener;

use std::sync::Arc;
use env_logger;
use log::info;
use tokio::signal;

#[tokio::main]
async fn main(){
  env_logger::init();
  rustls::crypto::ring::default_provider().install_default().unwrap();

  let scheduler = Arc::new(BackupScheduler::new());

  let _fileListener = FileListener::new(scheduler.clone()).await.unwrap();
  info!("File Listener running.");

  let connectionListener = ConnectionListener::new();
  let _connectionListenerHandle = tokio::spawn(async move {
    connectionListener.listen(scheduler).await;
  });
  info!("Connection Listener running.");

  let _recordingHandle = tokio::spawn(async move {
    Camera::CameraController::startRecording().await.unwrap();
  });
  info!("Camera live! :D");

  zerocam_lib::run();

  signal::ctrl_c()
    .await
    .expect("Failed to install Ctrl+C signal handler");

  info!("Shutting Down!")
}
