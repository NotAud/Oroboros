use std::sync::Arc;

use crate::modules::reactive::AppState;

#[tauri::command]
pub fn set_randomized_interval(min: u64, max: u64, state: tauri::State<'_, Arc<AppState>>) {
    *state.randomized_min.value.write().unwrap() = min;
    *state.randomized_max.value.write().unwrap() = max;
}
