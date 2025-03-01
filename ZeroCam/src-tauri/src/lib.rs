// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#![allow(non_snake_case)]
mod Camera;

pub use crate::Camera::ClipScheduler::ClipScheduler;
use std::sync::Arc;
use log::info;

#[tauri::command]
async fn feScheduleClip(state: tauri::State<'_, Arc<ClipScheduler>>) -> Result<(), String> {
  state
    .scheduleClip()
    .await
    .map_err(|e| e.to_string())
}

pub fn run(clipScheduler:Arc<ClipScheduler>) {
  tauri::Builder::default()
    .manage(clipScheduler)
    .invoke_handler(tauri::generate_handler![feScheduleClip])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
