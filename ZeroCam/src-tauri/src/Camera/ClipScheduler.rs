use crate::Camera::CameraController;
use crate::Config::getConfig;
use crate::Config::ConfigFile;

use log::{info, warn};
use std::error::Error;
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use chrono::Utc;
use tokio::time::timeout;

pub struct ClipScheduler {
  cameraController: CameraController::CameraController,
  config          : ConfigFile,
}

static IS_RUNNING: AtomicBool = AtomicBool::new(false);
static LAST_RUN: AtomicI64 = AtomicI64::new(0);

struct RunningGuard;

impl Drop for RunningGuard {
  fn drop(&mut self) {
    IS_RUNNING.store(false, Ordering::SeqCst);
  }
}

/// This allows only 1 process to happen at any time and prevents any concurrent attempts
impl ClipScheduler {
  pub async fn new() -> Self {
    Self {
      cameraController: CameraController::CameraController::new().await.unwrap(),
      config          : getConfig().await.unwrap(),
    }
  }

  pub async fn scheduleClip(&self) -> Result<(), Box<dyn Error>> {
    let currentTime = Utc::now().timestamp();
    let lastRunDifference = currentTime - LAST_RUN.load(Ordering::SeqCst);
    if(lastRunDifference < self.config.camera_input.clip.cooldown_sec) {
      let remainingCooldown = self.config.camera_input.clip.cooldown_sec - lastRunDifference;
      warn!("Clip scheduler on cooldown, {}sec  remaining - skipping this request", &remainingCooldown);
      return Err(format!("Clip scheduler on cooldown, {}sec  remaining - skipping this request", self.config.camera_input.clip.cooldown_sec - &remainingCooldown).into());
    }

    if IS_RUNNING
      .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
      .is_err()
    {
      warn!("Clip process in progress - skipping this request");
      return Err("Clip process in progress - skipping this request".into());
    }

    let _guard = RunningGuard;

    let result = self.cameraController.clip().await?;

    LAST_RUN.store(Utc::now().timestamp(), Ordering::SeqCst);
    info!("Clip completed successfully");
    Ok(result)
  }
}
