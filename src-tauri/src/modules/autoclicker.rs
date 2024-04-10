use crate::modules::reactive::AppState;
use rand::{thread_rng, Rng};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

pub fn create_autoclicker(state: Arc<AppState>) {
    std::thread::spawn(move || {
        let mut now = Instant::now();
        let mut rng = thread_rng();
        let mut current_interval: u64 = 0;
        loop {
            let active = { *state.active.value.read().unwrap() };
            if active {
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
                    println!("Click! {}", current_interval);
                    // let _ = rdev::simulate(&rdev::EventType::ButtonPress(rdev::Button::Left));
                    // let _ = rdev::simulate(&rdev::EventType::ButtonRelease(rdev::Button::Left));

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
