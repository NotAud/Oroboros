// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod modules;

use std::sync::{Arc, RwLock};

use modules::autoclicker::create_autoclicker;
use tauri::Manager;

use crate::commands::settings::{
    get_status::get_status, set_click_repeater::set_click_repeater, set_click_type::set_click_type,
    set_hotkey::set_hotkey, set_interval::set_interval, set_is_randomized::set_is_randomized,
    set_randomized_interval::set_randomized_interval, set_repeat_amount::set_repeat_amount,
    set_window_detection::set_window_detection, set_window_hwnd::set_window_hwnd,
};
use crate::commands::util::get_windows::get_windows;
use crate::modules::hotkeys::HotkeyListener;
use crate::modules::reactive::{AppState, Reactive};
use crate::modules::state_broadcaster::create_broadcaster;
use std::sync::mpsc;
use windows::Win32::Foundation::HWND;

fn main() {
    let (sender, receiver) = mpsc::channel();

    // Possibly remove sender/reactive from values where emitting is not needed
    let app_state = Arc::new(AppState {
        active: Reactive::new(false, sender.clone()),
        interval: Reactive::new(1000, sender.clone()),
        is_randomized: Reactive::new(false, sender.clone()),
        randomized_min: Reactive::new(1000, sender.clone()),
        randomized_max: Reactive::new(1000, sender.clone()),
        click_type: RwLock::new(0),
        click_repeater: RwLock::new(false),
        click_counter: RwLock::new(0),
        repeat_amount: RwLock::new(1),
        hotkey: Reactive::new(vec![], sender.clone()),
        window_detection: Reactive::new(false, sender.clone()),
        window_hwnd: RwLock::new(HWND(0)),
    });

    HotkeyListener::new(Arc::clone(&app_state)).start();
    create_autoclicker(Arc::clone(&app_state));

    tauri::Builder::default()
        .manage(app_state.clone())
        .invoke_handler(tauri::generate_handler![
            get_status,
            set_interval,
            set_is_randomized,
            set_randomized_interval,
            set_click_type,
            set_click_repeater,
            set_repeat_amount,
            set_hotkey,
            get_windows,
            set_window_detection,
            set_window_hwnd
        ])
        .setup(move |app| {
            let handle = app.app_handle();
            create_broadcaster(receiver, handle);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
