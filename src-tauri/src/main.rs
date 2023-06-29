// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{Manager, Runtime};
use window_shadows::set_shadow;
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            set_window_shadow(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn set_window_shadow<R: Runtime>(app: &tauri::App<R>) {
    let window = app.get_window("main").unwrap();
    set_shadow(&window, true).expect("Unsupported platform!");
}
