use std::sync::Arc;

use crate::modules::reactive::AppState;
use crate::modules::reactive::Key;
use indexmap::IndexSet;
use rdev::{listen, Event, EventType};

pub struct HotkeyListener {
    pressed_keys: IndexSet<Key>,
    hotkey_active: bool,
    state: Arc<AppState>,
}

impl HotkeyListener {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self {
            pressed_keys: IndexSet::new(),
            hotkey_active: false,
            state: app_state,
        }
    }

    pub fn start(mut self) {
        std::thread::spawn(move || {
            if let Err(error) = listen(move |event: Event| self.callback(event)) {
                println!("Error: {:?}", error)
            }
        });
    }

    fn callback(&mut self, event: Event) {
        match event.event_type {
            EventType::KeyPress(key) => {
                if self.hotkey_active {
                    return;
                }

                self.pressed_keys.insert(Key::from(key));

                let hotkey = { &*self.state.hotkey.value.read().unwrap() };
                if self.match_hotkey(hotkey) {
                    self.hotkey_active = true;

                    {
                        let mut active = self.state.active.value.write().unwrap();
                        *active = !*active;
                    }
                    *self.state.click_counter.write().unwrap() = 0;

                    self.state.active.emit("autoclicker_status");
                }
            }
            EventType::KeyRelease(key) => {
                self.pressed_keys.shift_remove(&Key::from(key));
                self.hotkey_active = false;
            }
            _ => (),
        }
    }

    fn match_hotkey(&self, hotkey: &Vec<Key>) -> bool {
        if self.pressed_keys.len() != hotkey.len() {
            return false;
        }

        for (i, key) in hotkey.iter().enumerate() {
            if *key != self.pressed_keys[i] {
                return false;
            }
        }

        return true;
    }
}
