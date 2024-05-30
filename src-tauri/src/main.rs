// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use log::{LevelFilter, Log, Metadata, Record};
use minecraft::auth::AuthFlow;
use crate::command::login::{devicecode_exchange, devicecode_init, minecraft_profile, minecraft_token, xbox_live_auth, xbox_xsts_auth};
use crate::minecraft::auth::{MinecraftAuthorizationFlow, MinecraftUUIDMap};

mod utils;
mod minecraft;
mod event;
mod command;

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
    if(std::env::var("RUST_Debug").unwrap_or("false".to_string()) == "true"){
        log::set_max_level(LevelFilter::Error);
    }else{
        log::set_max_level(LevelFilter::Info)
    }
    log::set_logger(&LOGGER).unwrap();
}

const CLIENT_ID:&str = env!("MICROSOFT_CLIENT_ID");

fn main() {
   
    init_log();
    
    let authflow = AuthFlow::new(MinecraftAuthorizationFlow::new(CLIENT_ID));
    let usermap:MinecraftUUIDMap = MinecraftUUIDMap::new(HashMap::new());
    tauri::Builder::default()
        .manage(authflow)
        .manage(usermap)
        .invoke_handler(tauri::generate_handler![
            devicecode_init,
            devicecode_exchange,
            xbox_live_auth,
            xbox_xsts_auth,
            minecraft_token,
            minecraft_profile
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
