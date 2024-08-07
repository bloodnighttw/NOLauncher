// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use crate::command::login::{
    devicecode_exchange, devicecode_init, minecraft_profile, minecraft_token, xbox_live_auth,
    xbox_xsts_auth,
};
use crate::command::user::{get_current_user, get_users, logout_user, set_current_user};
use crate::utils::config::{NoLauncherConfig, Storage};
use log::{LevelFilter, Log, Metadata, Record};
use tauri::Manager;
use tokio::sync::{Mutex, RwLock};
use crate::command::instance::{create_instance, list_instance, list_versions, launch_game, get_instance_status};
use crate::utils::minecraft::auth::{AccountList, AuthFlow, MinecraftAuthorizationFlow};
use crate::utils::minecraft::instance::{InstanceLock, SafeInstanceStatus};

mod command;
mod event;
mod utils;
mod constant;

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
    let builder = tauri::Builder::default();

    #[cfg(debug_assertions)]
    let builder = builder.plugin(tauri_plugin_devtools::init());
    let instance_status:SafeInstanceStatus = HashMap::new().into();
    let runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(6).enable_io().enable_time().build().unwrap();
    let lock: InstanceLock = Mutex::new(());

    builder
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .manage(authflow)
        .manage(instance_status)
        .manage(runtime)
        .manage(lock)
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
            logout_user,
            list_versions,
            create_instance,
            list_instance,
            launch_game,
            get_instance_status
        ])
        .setup(|app| {
            let handle = app.handle();
            tauri::async_runtime::block_on(async move {
                
                match NoLauncherConfig::load_by_app(&handle){
                    Ok(config) => {
                        let data = RwLock::new(*config);
                        handle.manage(data);
                    }
                    Err(e) => {
                        log::error!("Failed to load the config,: {}", e);
                        handle.manage(RwLock::new(NoLauncherConfig::default()));
                    }
                }

                let account_list = RwLock::new(*AccountList::load_by_app(&handle).unwrap_or(Box::new(AccountList::default())));
                handle.manage(account_list);

            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
