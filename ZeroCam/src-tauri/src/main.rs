#![allow(non_snake_case)]
mod Camera;
mod GDFiles;
mod Net;
mod Telegram;

use crate::Camera::ClipScheduler::ClipScheduler;
use crate::Camera::CameraController::CameraController;
use crate::GDFiles::BackupScheduler::BackupScheduler;
use crate::GDFiles::FileListener::FileListener;
use crate::Net::ConnectionListener::ConnectionListener;
use crate::Telegram::TelegramBot;

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

  let _cameraProcess = Camera::CameraController::startCameraAndStream().await.unwrap();
  info!("Camera live.");

  let clipScheduler = Arc::new(zerocam_lib::ClipScheduler::new()); //zerocam_lib necessary as tauri gets confused

  let _telegramBot = tokio::spawn(async move{
    TelegramBot::newBot().await.unwrap();
  });
  info!("Telegram bot live.");

  zerocam_lib::run(clipScheduler.clone());

  signal::ctrl_c()
    .await
    .expect("Failed to install Ctrl+C signal handler");

  info!("Shutting Down!")
}
