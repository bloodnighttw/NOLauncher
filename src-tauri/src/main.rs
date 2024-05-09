// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;

// Learn more about Tauri commpub(crate) pub(crate) ands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![auth::msa_auth_open_browser,auth::msa_auth_init,auth::msa_auth_exchange])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
