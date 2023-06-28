// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use tauri::Manager;
use window_shadows::set_shadow;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let wnd = &app.get_window("main").unwrap();
            wnd.set_decorations(false).unwrap();
            set_shadow(wnd, true).unwrap();
            wnd.set_fullscreen(false).unwrap();
            wnd.set_focus().unwrap();
            wnd.show().unwrap();

            #[cfg(target_os="windows")]
            {
                use winapi::shared::windef::HWND;
                use winapi::um::winuser::{GetWindowLongW, GWL_STYLE, SetWindowLongW, WS_MAXIMIZEBOX};
                if let RawWindowHandle::Win32(handle) = wnd.raw_window_handle() {
                    let hwnd = handle.hwnd as HWND;
                    unsafe {
                        SetWindowLongW(hwnd, GWL_STYLE, GetWindowLongW(hwnd, GWL_STYLE) & !WS_MAXIMIZEBOX as i32);
                    }
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}