use std::sync::mpsc::Receiver;
use tauri::{AppHandle, Manager};

pub fn create_broadcaster(rx: Receiver<(String, String)>, handle: AppHandle) {
    std::thread::spawn(move || {
        while let Ok((event, payload)) = rx.recv() {
            handle.emit_all(&event, payload).expect("failed to emit")
        }
    });
}
