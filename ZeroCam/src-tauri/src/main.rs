#![allow(non_snake_case)]
pub mod Camera;
mod Config;
mod GDFiles;
mod Net;
mod Telegram;

use zerocam_lib::Camera::CameraController::CameraController;
use zerocam_lib::Camera::ClipScheduler::ClipScheduler;
use zerocam_lib::Camera::MotionListener::MotionListener;
use zerocam_lib::GDFiles::BackupScheduler::BackupScheduler;
use zerocam_lib::GDFiles::FileListener::FileListener;
use zerocam_lib::Net::ConnectionListener::listen;
use crate::Telegram::TelegramBot;

use env_logger;
use log::info;
use std::error::Error;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::task::LocalSet;
use tokio::{signal, task};

#[tokio::main]
async fn main() {
  env_logger::init();
  rustls::crypto::ring::default_provider()
    .install_default()
    .unwrap();

  Config::showConfig().await;

  let backupScheduler = Arc::new(BackupScheduler::new().await.unwrap());

  let _fileListener = FileListener::new(backupScheduler.clone()).await.unwrap();
  info!("File Listener running.");

  let _connectionListenerHandle = tokio::spawn(async move {
    listen(backupScheduler).await;
  });
  info!("Connection Listener running.");

  let _cameraProcess = Camera::CameraController::startCameraAndStream()
    .await
    .unwrap();
  info!("Camera live.");

  let clipScheduler = Arc::new(ClipScheduler::new().await); //zerocam_lib necessary as tauri gets confused

  let _telegramBot = tokio::spawn(async move {
    TelegramBot::newBot().await.unwrap();
  });
  info!("Telegram bot live.");

  let motionListener = MotionListener::new(clipScheduler.clone())
    .await
    .unwrap();
  // let _motionListenerProcess = tokio::spawn(async move {
  //   motionListener.run().await;
  // });
  // info!("Motion Listener running.");

  zerocam_lib::run(clipScheduler);

  signal::ctrl_c()
    .await
    .expect("Failed to install Ctrl+C signal handler");

  info!("Shutting Down!")
}
