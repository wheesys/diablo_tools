// Copyright 2025 zl. All rights reserved.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod commands;
mod core;
mod error;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::save::open_d2s,
            commands::save::save_d2s,
            commands::save::get_character_info,
            commands::save::backup_save,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
