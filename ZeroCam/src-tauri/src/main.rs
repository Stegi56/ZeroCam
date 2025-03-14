#![allow(non_snake_case)]
mod Config;
pub mod Camera;
mod GDFiles;
mod Net;
mod Telegram;

use crate::Camera::ClipScheduler::ClipScheduler;
use crate::Camera::CameraController::CameraController;
use crate::Camera::MotionListener::MotionListener;
use crate::GDFiles::BackupScheduler::BackupScheduler;
use crate::GDFiles::FileListener::FileListener;
use crate::Net::ConnectionListener::ConnectionListener;
use crate::Telegram::TelegramBot;

use std::error::Error;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use env_logger;
use log::info;
use tokio::{signal, task};
use tokio::task::LocalSet;

#[tokio::main]
async fn main(){
  env_logger::init();
  rustls::crypto::ring::default_provider().install_default().unwrap();

  Config::showConfig().await;

  let backupScheduler = Arc::new(BackupScheduler::new().await.unwrap());

  let _fileListener = FileListener::new(backupScheduler.clone()).await.unwrap();
  info!("File Listener running.");

  let connectionListener = ConnectionListener::new();
  let _connectionListenerHandle = tokio::spawn(async move {
    connectionListener.listen(backupScheduler).await;
  });
  info!("Connection Listener running.");

  let _cameraProcess = Camera::CameraController::startCameraAndStream().await.unwrap();
  info!("Camera live.");

  let clipScheduler = Arc::new(zerocam_lib::ClipScheduler::new().await); //zerocam_lib necessary as tauri gets confused

  let _telegramBot = tokio::spawn(async move{
    TelegramBot::newBot().await.unwrap();
  });
  info!("Telegram bot live.");

  let motionListener = zerocam_lib::MotionListener::new(clipScheduler.clone()).await.unwrap();
  let _motionListenerProcess = tokio::spawn(async move{
    motionListener.run().await;
  });

  zerocam_lib::run(clipScheduler);

  signal::ctrl_c()
    .await
    .expect("Failed to install Ctrl+C signal handler");

  info!("Shutting Down!")
}
