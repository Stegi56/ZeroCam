// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#![allow(non_snake_case)]
mod Camera;

pub use crate::Camera::ClipScheduler::ClipScheduler;
use std::sync::Arc;
use log::info;

#[derive(Clone)]
pub struct AppState {
  pub clipScheduler: Arc<ClipScheduler>,
}

#[tauri::command]
async fn feScheduleClip(state: tauri::State<'_, AppState>) -> Result<(), String> {
  state
    .clipScheduler
    .scheduleClip()
    .await
    .map_err(|e| e.to_string())
}

pub fn run(clipScheduler:Arc<ClipScheduler>) {
  let appState = AppState{ clipScheduler};

  tauri::Builder::default()
    .manage(appState)
    .invoke_handler(tauri::generate_handler![feScheduleClip])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
