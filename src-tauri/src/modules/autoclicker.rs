use crate::modules::reactive::AppState;
use rand::{thread_rng, Rng};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use windows::Win32::Foundation::{POINT, RECT};
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, GetForegroundWindow, GetWindowRect};

pub fn create_autoclicker(state: Arc<AppState>) {
    std::thread::spawn(move || {
        let mut now = Instant::now();
        let mut rng = thread_rng();
        let mut current_interval: u64 = 0;

        loop {
            let active = { *state.active.value.read().unwrap() };
            if active {
                let window_lock = { *state.window_detection.value.read().unwrap() };
                if window_lock {
                    let window_hwnd = { *state.window_hwnd.read().unwrap() };
                    if window_hwnd != unsafe { GetForegroundWindow() } {
                        {
                            *state.active.value.write().unwrap() = false;
                        }

                        state.active.emit("autoclicker_status");

                        std::thread::sleep(Duration::from_millis(1));
                        continue;
                    }

                    let point = unsafe {
                        let mut point = POINT::default();
                        GetCursorPos(&mut point).unwrap();
                        point
                    };

                    let rect = unsafe {
                        let mut rect = RECT::default();
                        GetWindowRect(window_hwnd, &mut rect).unwrap();
                        rect
                    };

                    if point.x < rect.left
                        || point.x > rect.right
                        || point.y < rect.top
                        || point.y > rect.bottom
                    {
                        {
                            *state.active.value.write().unwrap() = false;
                        }

                        state.active.emit("autoclicker_status");
                        continue;
                    }
                }

                let is_randomized = { *state.is_randomized.value.read().unwrap() };
                if current_interval == 0 {
                    if is_randomized {
                        let min_interval = { *state.randomized_min.value.read().unwrap() };
                        let max_interval = { *state.randomized_max.value.read().unwrap() };
                        current_interval = rng.gen_range(min_interval..=max_interval);
                    } else {
                        current_interval = *state.interval.value.read().unwrap();
                    }
                }

                let current_time = Duration::from_millis(current_interval);

                if now.elapsed() >= current_time {
                    let click_repeater = { *state.click_repeater.read().unwrap() };
                    if click_repeater {
                        let repeat_amount = { *state.repeat_amount.read().unwrap() };
                        let click_counter = { *state.click_counter.read().unwrap() };
                        if click_counter >= repeat_amount {
                            {
                                *state.active.value.write().unwrap() = false;
                                *state.click_counter.write().unwrap() = 0;
                            }

                            state.active.emit("autoclicker_status");
                            continue;
                        }

                        *state.click_counter.write().unwrap() += 1;
                    }

                    let click_type = { *state.click_type.read().unwrap() };
                    let _ = rdev::simulate(&rdev::EventType::ButtonPress(rdev::Button::Left));
                    let _ = rdev::simulate(&rdev::EventType::ButtonRelease(rdev::Button::Left));

                    if click_type == 1 {
                        let _ = rdev::simulate(&rdev::EventType::ButtonPress(rdev::Button::Left));
                        let _ = rdev::simulate(&rdev::EventType::ButtonRelease(rdev::Button::Left));
                    }

                    now = Instant::now();
                    current_interval = 0;
                    continue;
                }

                std::thread::sleep(Duration::from_millis(1));
            } else {
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    });
}
