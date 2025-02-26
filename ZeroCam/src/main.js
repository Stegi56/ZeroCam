const { invoke } = window.__TAURI__.core;

function scheduleClip() {
  invoke("feScheduleClip");
}

window.addEventListener("DOMContentLoaded", () => {
  document.getElementById("clip-button").addEventListener("click", scheduleClip);
});