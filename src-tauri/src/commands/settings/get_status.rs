use std::sync::Arc;

use crate::modules::reactive::AppState;

#[tauri::command]
pub fn get_status(state: tauri::State<'_, Arc<AppState>>) {
    let state = Arc::clone(&state);

    let mut active_write = state.active.value.write().unwrap();
    *active_write = !*active_write;
    drop(active_write);

    state.active.emit("autoclicker_status");
}
