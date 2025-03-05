use crate::Camera::CameraController;

use log::{info, warn};
use std::error::Error;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

pub struct ClipScheduler {
  cameraController: CameraController::CameraController,
}

static IS_RUNNING: AtomicBool = AtomicBool::new(false);

struct RunningGuard;

impl Drop for RunningGuard {
  fn drop(&mut self) {
    IS_RUNNING.store(false, Ordering::SeqCst);
  }
}

/// This allows only 1 process to happen at any time and prevents any concurrent attempts
impl ClipScheduler {
  pub fn new() -> Self {
    Self {
      cameraController: CameraController::CameraController::new().unwrap(),
    }
  }

  pub async fn scheduleClip(&self) -> Result<(), Box<dyn Error>> {
    if IS_RUNNING
      .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
      .is_err()
    {
      warn!("Clip process in progress - skipping this request");
      return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "There is already a clip in progress - skipping this request")));
    }

    let _guard = RunningGuard;

    let result = timeout(Duration::from_secs(10), async {
      self.cameraController.clip().await
    }).await??;

    info!("Clip completed successfully");
    Ok(result)
  }
}
