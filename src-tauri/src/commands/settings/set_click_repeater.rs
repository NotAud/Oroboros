use std::sync::Arc;

use crate::modules::reactive::AppState;

#[tauri::command]
pub fn set_click_repeater(state: tauri::State<'_, Arc<AppState>>) {
    let mut click_repeater = state.click_repeater.write().unwrap();
    *click_repeater = !*click_repeater;
    drop(click_repeater);
    *state.click_counter.write().unwrap() = 0;
}
