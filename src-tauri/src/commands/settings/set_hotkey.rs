use std::sync::Arc;

use crate::modules::reactive::{AppState, Key};

#[tauri::command]
pub fn set_hotkey(hotkey: Vec<String>, state: tauri::State<'_, Arc<AppState>>) {
    let mut parsed_hotkey: Vec<Key> = Vec::new();
    for key in key_match(hotkey) {
        parsed_hotkey.push(Key::from(key));
    }

    let mut hotkey_write = state.hotkey.value.write().unwrap();
    *hotkey_write = parsed_hotkey;
    drop(hotkey_write);

    state.hotkey.emit("register_hotkey");
}

fn key_match(raw_keys: Vec<String>) -> Vec<rdev::Key> {
    let mut key_vec: Vec<rdev::Key> = Vec::new();
    for key in raw_keys {
        match key.as_str() {
            "a" => key_vec.push(rdev::Key::KeyA),
            "b" => key_vec.push(rdev::Key::KeyB),
            "c" => key_vec.push(rdev::Key::KeyC),
            "d" => key_vec.push(rdev::Key::KeyD),
            "e" => key_vec.push(rdev::Key::KeyE),
            "f" => key_vec.push(rdev::Key::KeyF),
            "g" => key_vec.push(rdev::Key::KeyG),
            "h" => key_vec.push(rdev::Key::KeyH),
            "i" => key_vec.push(rdev::Key::KeyI),
            "j" => key_vec.push(rdev::Key::KeyJ),
            "k" => key_vec.push(rdev::Key::KeyK),
            "l" => key_vec.push(rdev::Key::KeyL),
            "m" => key_vec.push(rdev::Key::KeyM),
            "n" => key_vec.push(rdev::Key::KeyN),
            "o" => key_vec.push(rdev::Key::KeyO),
            "p" => key_vec.push(rdev::Key::KeyP),
            "q" => key_vec.push(rdev::Key::KeyQ),
            "r" => key_vec.push(rdev::Key::KeyR),
            "s" => key_vec.push(rdev::Key::KeyS),
            "t" => key_vec.push(rdev::Key::KeyT),
            "u" => key_vec.push(rdev::Key::KeyU),
            "v" => key_vec.push(rdev::Key::KeyV),
            "w" => key_vec.push(rdev::Key::KeyW),
            "x" => key_vec.push(rdev::Key::KeyX),
            "y" => key_vec.push(rdev::Key::KeyY),
            "z" => key_vec.push(rdev::Key::KeyZ),
            "0" => key_vec.push(rdev::Key::Num0),
            "1" => key_vec.push(rdev::Key::Num1),
            "2" => key_vec.push(rdev::Key::Num2),
            "3" => key_vec.push(rdev::Key::Num3),
            "4" => key_vec.push(rdev::Key::Num4),
            "5" => key_vec.push(rdev::Key::Num5),
            "6" => key_vec.push(rdev::Key::Num6),
            "7" => key_vec.push(rdev::Key::Num7),
            "8" => key_vec.push(rdev::Key::Num8),
            "9" => key_vec.push(rdev::Key::Num9),
            "Control" => key_vec.push(rdev::Key::ControlLeft),
            "Shift" => key_vec.push(rdev::Key::ShiftLeft),
            "F1" => key_vec.push(rdev::Key::F1),
            "F2" => key_vec.push(rdev::Key::F2),
            "F3" => key_vec.push(rdev::Key::F3),
            "F4" => key_vec.push(rdev::Key::F4),
            "F5" => key_vec.push(rdev::Key::F5),
            "F6" => key_vec.push(rdev::Key::F6),
            "F7" => key_vec.push(rdev::Key::F7),
            "F8" => key_vec.push(rdev::Key::F8),
            "F9" => key_vec.push(rdev::Key::F9),
            "F10" => key_vec.push(rdev::Key::F10),
            "F11" => key_vec.push(rdev::Key::F11),
            "F12" => key_vec.push(rdev::Key::F12),
            "`" => key_vec.push(rdev::Key::BackQuote),
            "-" => key_vec.push(rdev::Key::Minus),
            "=" => key_vec.push(rdev::Key::Equal),
            "[" => key_vec.push(rdev::Key::LeftBracket),
            "]" => key_vec.push(rdev::Key::RightBracket),
            "\\" => key_vec.push(rdev::Key::BackSlash),
            ";" => key_vec.push(rdev::Key::SemiColon),
            "'" => key_vec.push(rdev::Key::Quote),
            "," => key_vec.push(rdev::Key::Comma),
            "." => key_vec.push(rdev::Key::Dot),
            "/" => key_vec.push(rdev::Key::Slash),
            _ => (),
        }
    }

    key_vec
}
