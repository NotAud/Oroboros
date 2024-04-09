use std::sync::Arc;

use crate::modules::reactive::AppState;

#[tauri::command]
pub fn set_window_detection(window_detection: bool, state: tauri::State<'_, Arc<AppState>>) {
    *state.window_detection.value.write().unwrap() = window_detection;
}
