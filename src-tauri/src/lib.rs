// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_dialog; 


pub mod apk_parser;
pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::parse_apk,
            // commands::parse_apk_data,
            commands::select_apk_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
