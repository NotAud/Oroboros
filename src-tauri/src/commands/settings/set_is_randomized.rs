use std::sync::Arc;

use crate::modules::reactive::AppState;

#[tauri::command]
pub fn set_is_randomized(is_randomized: bool, state: tauri::State<'_, Arc<AppState>>) {
    *state.is_randomized.value.write().unwrap() = is_randomized;
}
