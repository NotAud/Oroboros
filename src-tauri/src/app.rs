use autopilot::mouse;
use device_query::{DeviceQuery, DeviceState, Keycode};
use serde_json::json;
use std::{
    sync::{
        atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, Ordering},
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};
use tauri::Manager;

#[derive(serde::Deserialize)]
struct Message<T> {
    value: T,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct Position {
    x: Arc<AtomicI32>,
    y: Arc<AtomicI32>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct Size {
    width: Arc<AtomicU32>,
    height: Arc<AtomicU32>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct ClickItApp {
    #[serde(skip_serializing, skip_deserializing)]
    app_handle: Option<Arc<tauri::AppHandle>>,
    #[serde(skip_serializing, skip_deserializing)]
    autoclicker_channel: Option<(Arc<Mutex<Sender<()>>>, Arc<Mutex<Receiver<()>>>)>,

    // Settings
    is_autoclicking: Arc<AtomicBool>,
    delay: Arc<AtomicU64>,
    window_position: Position,
    window_size: Size,
}

impl ClickItApp {
    pub fn new(app_handle: Arc<tauri::AppHandle>) -> Self {
        let main_window = app_handle.get_window("main");

        match main_window {
            Some(main_window) => {
                let window_position = main_window.outer_position();
                let window_size = main_window.outer_size();
                match (window_position, window_size) {
                    (Ok(window_position), Ok(window_size)) => {
                        let (x, y) = (window_position.x, window_position.y);
                        let (width, height) = (window_size.width, window_size.height);
                        let channel = channel();
                        return Self {
                            app_handle: Some(app_handle),
                            autoclicker_channel: Some((
                                Arc::new(Mutex::new(channel.0)),
                                Arc::new(Mutex::new(channel.1)),
                            )),
                            is_autoclicking: Arc::new(AtomicBool::new(false)),
                            delay: Arc::new(AtomicU64::new(50)),
                            window_position: Position {
                                x: Arc::new(AtomicI32::new(x)),
                                y: Arc::new(AtomicI32::new(y)),
                            },
                            window_size: Size {
                                width: Arc::new(AtomicU32::new(width)),
                                height: Arc::new(AtomicU32::new(height)),
                            },
                        };
                    }
                    _ => (),
                }
            }
            None => panic!("Could not get window position or window size"),
        }

        panic!("Could not get main window");
    }

    pub fn run(self) {
        let this = self.clone();
        self.app_handle.as_ref().unwrap().as_ref().listen_global(
            "start_autoclicker",
            move |_event| {
                this.start_autoclicker();
            },
        );

        let this = self.clone();
        self.app_handle
            .as_ref()
            .unwrap()
            .listen_global("set_delay", move |event| {
                let payload = event.payload();
                match payload {
                    Some(payload) => {
                        let delay: Message<u64> = serde_json::from_str(payload).unwrap();
                        this.set_delay(delay.value);
                    }
                    None => (),
                }
            });

        thread::spawn({
            let this = self.clone();

            move || loop {
                // let is_visible = this
                //     .app_handle
                //     .clone()
                //     .unwrap()
                //     .get_window("main")
                //     .unwrap()
                //     .is_visible();
                // if is_visible.is_err() {
                //     panic!("Could not get window visibility")
                // }

                if this.is_autoclicking.load(Ordering::Relaxed) {
                    // println!("{:?}", is_visible.unwrap());
                    if !this.boundary_check() {
                        let _ = mouse::click(mouse::Button::Left, Some(0));
                    }
                }

                let (_, ref receiver) = this.autoclicker_channel.as_ref().unwrap();
                match receiver
                    .lock()
                    .unwrap()
                    .recv_timeout(Duration::from_millis(this.delay.load(Ordering::Relaxed)))
                {
                    Ok(_) => continue,
                    Err(_) => continue,
                }
            }
        });

        thread::spawn(move || {
            let device_state = DeviceState::new();

            let mut f2_pressed = false;
            loop {
                let keys = device_state.get_keys();
                if keys.contains(&Keycode::F2) {
                    if !f2_pressed {
                        f2_pressed = true;

                        self.start_autoclicker();
                    }
                } else {
                    f2_pressed = false;
                }
                thread::sleep(Duration::from_millis(50));
            }
        });
    }

    pub fn start_autoclicker(&self) {
        self.update_window_position();

        let is_autoclicking = self.is_autoclicking.load(Ordering::Relaxed);
        self.is_autoclicking
            .store(!is_autoclicking, Ordering::Relaxed);

        if !is_autoclicking {
            match self.autoclicker_channel {
                Some((ref sender, ref _receiver)) => {
                    let sender = sender.lock().unwrap();
                    sender.send(()).unwrap();
                }
                None => (),
            }
        }

        self.sync();
    }

    pub fn set_delay(&self, delay: u64) {
        self.delay.store(delay, Ordering::Relaxed);
        self.sync();
    }

    pub fn update_window_position(&self) {
        let window_position = self
            .app_handle
            .as_ref()
            .unwrap()
            .get_window("main")
            .unwrap()
            .outer_position();

        match window_position {
            Ok(window_position) => {
                let (x, y) = (window_position.x, window_position.y);
                self.window_position.x.store(x, Ordering::Relaxed);
                self.window_position.y.store(y, Ordering::Relaxed);
            }
            Err(e) => println!("Error: {:#?}", e),
        }
    }

    fn boundary_check(&self) -> bool {
        let mouse_position = mouse::location();
        return (mouse_position.x > self.window_position.x.load(Ordering::Relaxed) as f64
            && mouse_position.x
                < self.window_position.x.load(Ordering::Relaxed) as f64
                    + self.window_size.width.load(Ordering::Relaxed) as f64)
            && (mouse_position.y > self.window_position.y.load(Ordering::Relaxed) as f64
                && mouse_position.y
                    < self.window_position.y.load(Ordering::Relaxed) as f64
                        + self.window_size.height.load(Ordering::Relaxed) as f64);
    }

    pub fn sync(&self) {
        let json_data = json!({
            "is_autoclicking": self.is_autoclicking.load(Ordering::Relaxed),
            "delay": self.delay.load(Ordering::Relaxed),
        });

        let result = self
            .app_handle
            .as_ref()
            .unwrap()
            .emit_all("update_settings", json_data);
        match result {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }
}
