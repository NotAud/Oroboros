use std::sync::Arc;

use crate::modules::reactive::AppState;
use windows::Win32::Foundation::HWND;

#[tauri::command]
pub fn set_window_hwnd(hwnd: isize, state: tauri::State<'_, Arc<AppState>>) {
    *state.window_hwnd.write().unwrap() = HWND(hwnd);
}
