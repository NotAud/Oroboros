use std::sync::Arc;

use crate::modules::reactive::AppState;

#[tauri::command]
pub fn set_window_hwnd(hwnd: isize, state: tauri::State<'_, Arc<AppState>>) {
    *state.window_hwnd.value.write().unwrap() = hwnd;
}
