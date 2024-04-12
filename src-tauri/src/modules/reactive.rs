use serde::Serialize;
use serde::Serializer;
use std::sync::mpsc::Sender;
use std::sync::RwLock;
use windows::Win32::Foundation::HWND;

pub struct AppState {
    pub active: Reactive<bool>,
    pub interval: Reactive<u64>,
    pub is_randomized: Reactive<bool>,
    pub randomized_min: Reactive<u64>,
    pub randomized_max: Reactive<u64>,
    pub click_type: RwLock<i8>,
    pub hotkey: Reactive<Vec<Key>>,
    pub window_detection: Reactive<bool>,
    pub window_hwnd: RwLock<HWND>,
}

pub struct Reactive<T> {
    pub value: RwLock<T>,
    pub emitter: Sender<(String, String)>,
}

impl<T: Serialize + Send + Clone> Reactive<T> {
    pub fn new(value: T, emitter: Sender<(String, String)>) -> Self {
        Self {
            value: RwLock::new(value),
            emitter,
        }
    }

    pub fn emit(&self, event: &str) {
        let value_read = { &*self.value.read().expect("Failed to lock for reading") };
        let serialized_value = serde_json::to_string(value_read).unwrap();

        let _ = self.emitter.send((event.to_string(), serialized_value));
    }
}

#[derive(Clone, Eq, Hash, PartialEq, Copy)]
pub struct Key(rdev::Key);

impl Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:?}", self.0))
    }
}

impl From<rdev::Key> for Key {
    fn from(key: rdev::Key) -> Self {
        Key(key)
    }
}
