use crate::Camera::CameraController;

use log::{info, warn};
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

pub struct ClipScheduler {
  isRunning: Arc<AtomicBool>,
  cameraController: CameraController::CameraController,
}

struct RunningGuard {
  flag: Arc<AtomicBool>,
}

impl Drop for RunningGuard {
  fn drop(&mut self) {
    self.flag.store(false, Ordering::SeqCst);
  }
}

/// This allows only 1 process to happen at any time and prevents any concurrent attempts
impl ClipScheduler {
  pub fn new() -> Self {
    Self {
      isRunning: Arc::new(AtomicBool::new(false)),
      cameraController: CameraController::CameraController::new().unwrap(),
    }
  }

  pub async fn scheduleClip(&self) -> Result<(), Box<dyn Error>> {
    if self.isRunning.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
      warn!("Clip process in progress - skipping this request");
      return Ok(());
    }

    let _guard = RunningGuard {
      flag: Arc::clone(&self.isRunning),
    };

    let result = timeout(Duration::from_secs(10), async {
      self.cameraController.clip()
    }).await??;

    info!("Clip completed successfully");
    Ok(result)
  }
}
