// Copyright 2025 zl. All rights reserved.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod commands;
mod core;
mod error;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
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
            commands::items::get_items,
            commands::items::get_item_details,
            commands::skills::get_skills,
            commands::skills::set_skill_level,
            commands::quests::get_quests,
            commands::quests::set_quest_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
