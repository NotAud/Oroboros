use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowLongW, GetWindowTextW, IsWindowVisible, GWL_EXSTYLE, WS_EX_TOOLWINDOW,
};

#[tauri::command]
pub fn get_windows() -> Vec<(usize, String)> {
    let window = enumerate_windows();
    window
}

fn enumerate_windows() -> Vec<(usize, String)> {
    let mut windows = Vec::new();
    unsafe {
        let _ = EnumWindows(
            Some(enum_windows_proc),
            LPARAM(&mut windows as *mut _ as isize),
        );
    }
    windows
}

extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let data = unsafe { &mut *(lparam.0 as *mut Vec<(usize, String)>) };
    unsafe {
        if IsWindowVisible(hwnd).as_bool() {
            let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
            if ex_style & WS_EX_TOOLWINDOW.0 == 0 {
                // Exclude tool windows
                let mut title = [0u16; 256];
                let len = GetWindowTextW(hwnd, &mut title);

                if len > 0 {
                    let title = String::from_utf16_lossy(&title[..len as usize]);
                    data.push((hwnd.0 as usize, title));
                }
            }
        }
        BOOL::from(true)
    }
}
