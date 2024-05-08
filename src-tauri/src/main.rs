// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use oauth2::StandardDeviceAuthorizationResponse;

mod auth;

// Learn more about Tauri commpub(crate) pub(crate) ands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {

    println!("MSA Auth Init");
    let ms_auth_flow = crate::auth::msa_auth::MicrosoftAuthFlow::new().unwrap();
    let detail:StandardDeviceAuthorizationResponse = ms_auth_flow.generate_msa_device_code_auth().unwrap();

    println!(
        "Open this URL in your browser:\n{}\nand enter the code: {}",
        &detail.verification_uri().to_string(),
        &detail.user_code().secret().to_string()
    );
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![auth::msa_auth_init])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
