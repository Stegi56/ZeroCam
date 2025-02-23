#![allow(non_snake_case)]
mod Camera;
mod Files;
mod Net;

use crate::Camera::ClipScheduler::ClipScheduler;
use crate::Camera::CameraController::CameraController;
use crate::Files::BackupScheduler::BackupScheduler;
use crate::Files::FileListener::FileListener;
use crate::Net::ConnectionListener::ConnectionListener;

use std::error::Error;
use std::sync::Arc;
use env_logger;
use log::info;
use tokio::signal;

#[tokio::main]
async fn main(){
  env_logger::init();
  rustls::crypto::ring::default_provider().install_default().unwrap();

  let backupScheduler = Arc::new(BackupScheduler::new());

  let _fileListener = FileListener::new(backupScheduler.clone()).await.unwrap();
  info!("File Listener running.");

  let connectionListener = ConnectionListener::new();
  let _connectionListenerHandle = tokio::spawn(async move {
    connectionListener.listen(backupScheduler).await;
  });
  info!("Connection Listener running.");

  let clipScheduler = Arc::new(zerocam_lib::ClipScheduler::new()); //zerocam_lib necessary as tauri gets confused

  let _cameraProcess = Camera::CameraController::startRecording().await.unwrap();

  info!("Camera live! :D");

  zerocam_lib::run(clipScheduler.clone());

  signal::ctrl_c()
    .await
    .expect("Failed to install Ctrl+C signal handler");

  info!("Shutting Down!")
}
