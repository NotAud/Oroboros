use crate::shared::reactive::Reactive;
use crossbeam_channel::{Receiver, Sender};
use std::sync::{Arc, RwLock};

pub struct AppState {
    pub active: RwLock<Reactive<bool>>,
    pub hotkey_channel: (Sender<u32>, Receiver<u32>),
}
