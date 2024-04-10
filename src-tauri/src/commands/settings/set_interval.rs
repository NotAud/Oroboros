use std::sync::Arc;

use crate::modules::reactive::AppState;

#[tauri::command]
pub fn set_interval(interval: u64, state: tauri::State<'_, Arc<AppState>>) {
    let mut interval_write = state.interval.value.write().unwrap();
    *interval_write = interval;
}
