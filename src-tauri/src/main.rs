// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::command::login::{
    devicecode_exchange, devicecode_init, minecraft_profile, minecraft_token, xbox_live_auth,
    xbox_xsts_auth,
};
use crate::command::user::{get_current_user, get_users, logout_user, set_current_user};
use crate::event::user::change_user;
use crate::utils::config::NoLauncherConfig;
use log::{LevelFilter, Log, Metadata, Record};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::RwLock;
use crate::utils::minecraft::auth::{AuthFlow, MinecraftAuthorizationFlow, MinecraftUUIDMap, read};

mod command;
mod event;
mod utils;

struct Logger;

impl Log for Logger {
    fn enabled(&self, _meta: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        eprintln!("{}: {}", record.level(), record.args());
    }

    fn flush(&self) {}
}

fn init_log() {
    static LOGGER: Logger = Logger;
    if std::env::var("RUST_Debug").unwrap_or("false".to_string()) == "true" {
        log::set_max_level(LevelFilter::Error);
    } else {
        log::set_max_level(LevelFilter::Info)
    }
    log::set_logger(&LOGGER).unwrap();
}

const CLIENT_ID: &str = env!("MICROSOFT_CLIENT_ID");

fn main() {
    
    if !cfg!(debug_assertions){
        init_log();
    }
    
    let authflow = AuthFlow::new(MinecraftAuthorizationFlow::new(CLIENT_ID));
    let usermap: MinecraftUUIDMap = MinecraftUUIDMap::new(HashMap::new());
    let builder = tauri::Builder::default();

    #[cfg(debug_assertions)]
    let builder = builder.plugin(tauri_plugin_devtools::init());

    builder
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .manage(authflow)
        .manage(usermap)
        .invoke_handler(tauri::generate_handler![
            devicecode_init,
            devicecode_exchange,
            xbox_live_auth,
            xbox_xsts_auth,
            minecraft_token,
            minecraft_profile,
            get_users,
            get_current_user,
            set_current_user,
            logout_user
        ])
        .setup(|app| {
            let handle = app.handle();
            tauri::async_runtime::block_on(async move {
                let res = read(handle).await;
                if let Err(e) = res {
                    log::error!("Failed to load the usermap: {}", e);
                }

                let config_path = handle.path().app_config_dir().unwrap();

                match NoLauncherConfig::read_from_path(config_path.join("config.json")).await {
                    Ok(config) => {
                        if let Some(id) = config.clone().read().await.activate_user_uuid.clone() {
                            change_user(Some(id), handle).await;
                        }
                        handle.manage(config);
                    }
                    Err(e) => {
                        log::error!("Failed to load the config,: {}", e);
                        handle.manage(Arc::new(RwLock::new(NoLauncherConfig::default())));
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
