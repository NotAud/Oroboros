use crate::modules::reactive::AppState;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

pub fn create_autoclicker(state: Arc<AppState>) {
    std::thread::spawn(move || {
        let mut now = Instant::now();

        loop {
            {
                if state.active.value.read().unwrap().clone() {
                    let current_time = Duration::from_millis(*state.interval.value.read().unwrap());
                    if now.elapsed() >= current_time {
                        // println!("Click!");
                        let _ = rdev::simulate(&rdev::EventType::ButtonPress(rdev::Button::Left));
                        let _ = rdev::simulate(&rdev::EventType::ButtonRelease(rdev::Button::Left));

                        now = Instant::now();
                    }
                }
            }

            std::thread::sleep(Duration::from_millis(5));
        }
    });
}
