use std::sync::Arc;

use crate::modules::reactive::AppState;

#[tauri::command]
pub fn set_click_type(click_type: i8, state: tauri::State<'_, Arc<AppState>>) {
    *state.click_type.write().unwrap() = click_type;
}
