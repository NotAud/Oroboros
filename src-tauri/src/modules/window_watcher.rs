use std::sync::Arc;

use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::GetForegroundWindow};

use super::reactive::AppState;

pub struct WindowWatcher {
    state: Arc<AppState>,
}

impl WindowWatcher {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub fn start(self) {
        std::thread::spawn(move || loop {
            if self.state.active.value.read().unwrap().clone()
                && self.state.window_detection.value.read().unwrap().clone()
            {
                let foreground_window = unsafe { GetForegroundWindow() };
                let window_hwnd = HWND(*self.state.window_hwnd.value.read().unwrap());

                if window_hwnd != foreground_window {
                    *self.state.active.value.write().unwrap() = false;
                    self.state.active.emit("autoclicker_status");
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(250));
        });
    }
}
