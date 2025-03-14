// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#![allow(non_snake_case)]
mod Camera;
mod Config;

pub use crate::Camera::ClipScheduler::ClipScheduler;
use std::sync::{Arc};
pub use crate::Camera::MotionListener::MotionListener;

#[tauri::command]
async fn feScheduleClip(state: tauri::State<'_, Arc<ClipScheduler>>) -> Result<(), String> {
  state
    .scheduleClip()
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
fn feSetParked(parked: bool) {
  Camera::MotionListener::setWatching(parked);
}

pub fn run(clipScheduler:Arc<ClipScheduler>) {
  tauri::Builder::default()
    .manage(clipScheduler)
    .invoke_handler(tauri::generate_handler![feScheduleClip, feSetParked])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
