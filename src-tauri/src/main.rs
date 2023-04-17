#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

mod app;
use app::ClickItApp;
use std::sync::Arc;

fn main() {
    let ctx = tauri::generate_context!();

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = Arc::new(app.app_handle().clone());
            let app_manager = Box::new(ClickItApp::new(app_handle).run());
            app.manage(app_manager);

            #[cfg(debug_assertions)]
            {
                let main_window = app.get_window("main").unwrap();
                main_window.open_devtools();
            }

            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
