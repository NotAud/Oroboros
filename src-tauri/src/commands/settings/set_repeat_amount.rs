use std::sync::Arc;

use crate::modules::reactive::AppState;

#[tauri::command]
pub fn set_repeat_amount(repeat_amount: u64, state: tauri::State<'_, Arc<AppState>>) {
    *state.repeat_amount.write().unwrap() = repeat_amount;
    *state.click_counter.write().unwrap() = 0;
}
