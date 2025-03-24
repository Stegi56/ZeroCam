// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#![allow(non_snake_case)]
pub mod Camera;
pub mod Config;
pub mod GDFiles;
pub mod Net;

pub use crate::Camera::ClipScheduler::ClipScheduler;
pub use crate::Camera::MotionListener::MotionListener;
pub use crate::Net::NetworkConnector::getKnownNetworks;
pub use crate::Config::getConfigAsString;
pub use crate::Config::setConfigFromString;

use log::{error};
use std::process::Command;
use std::sync::{Arc, OnceLock};

static previousNetworkState: OnceLock<Vec<String>> = OnceLock::new();

#[tauri::command]
async fn feScheduleClip(state: tauri::State<'_, Arc<ClipScheduler>>) -> Result<(), String> {
  state.scheduleClip().await.map_err(|e| e.to_string())
}

#[tauri::command]
fn feSetParked(parked: bool) {
  Camera::MotionListener::setParkedState(parked);
}

#[tauri::command]
fn feGetParked() -> bool {
  Camera::MotionListener::getParkedState()
}

#[tauri::command]
fn feRebootSystem() {
  if Command::new("sudo").arg("reboot").spawn().is_err() {
    error!("Failed to reboot the system");
  }
}

#[tauri::command]
async fn feGetKnownNetworks() -> Result<Vec<String>, String> {
  getKnownNetworks().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn feGetConfig() -> Result<String, String>{
  getConfigAsString().await.map_err(|e| e.to_string())
}

#[tauri::command]
fn feSetConfig(config: String) -> Result<(), String> {
  setConfigFromString(config).map_err(|e| e.to_string())
}

pub fn run(clipScheduler: Arc<ClipScheduler>) {
  tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .manage(clipScheduler)
    .invoke_handler(tauri::generate_handler![
      feScheduleClip,
      feSetParked,
      feGetConfig,
      feSetConfig,
      feGetParked,
      feGetKnownNetworks
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
